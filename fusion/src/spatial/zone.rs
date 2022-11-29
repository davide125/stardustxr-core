use crate::{
	fields::Field,
	node::{ClientOwned, Node, NodeError, NodeType},
	HandlerWrapper,
};

use super::Spatial;
use parking_lot::Mutex;
use rustc_hash::FxHashMap;
use stardust_xr::{schemas::flex::deserialize, values::Transform};
use std::sync::Arc;

pub trait ZoneHandler: Send + Sync {
	fn enter(&mut self, uid: &str, spatial: &Spatial);
	fn capture(&mut self, uid: &str, spatial: &Spatial);
	fn release(&mut self, uid: &str);
	fn leave(&mut self, uid: &str);
}

#[derive(Debug)]
pub struct Zone {
	pub spatial: Spatial,
	pub captured: Mutex<FxHashMap<String, Spatial>>,
}

impl<'a> Zone {
	pub fn create<F, Fi: Field + ClientOwned, T>(
		spatial_parent: &'a Spatial,
		position: Option<mint::Vector3<f32>>,
		rotation: Option<mint::Quaternion<f32>>,
		field: &'a Fi,
		wrapped_init: F,
	) -> Result<HandlerWrapper<Zone, T>, NodeError>
	where
		F: FnOnce(&Arc<Zone>) -> T,
		T: ZoneHandler + 'static,
	{
		let id = nanoid::nanoid!();
		let zone = Zone {
			spatial: Spatial {
				node: Node::new(
					&spatial_parent.node.client()?,
					"/spatial",
					"create_zone",
					"/spatial/zone",
					true,
					&id.clone(),
					(
						id,
						spatial_parent.node().get_path()?,
						Transform {
							position,
							rotation,
							scale: None,
						},
						field.node().get_path()?,
					),
				)?,
			},
			captured: Mutex::new(FxHashMap::default()),
		};

		let handler_wrapper =
			HandlerWrapper::new(zone, |_weak_handler, node_ref| wrapped_init(node_ref));
		handler_wrapper.add_handled_signal("enter", Self::handle_enter)?;
		handler_wrapper.add_handled_signal("capture", Self::handle_capture)?;
		handler_wrapper.add_handled_signal("release", Self::handle_release)?;
		handler_wrapper.add_handled_signal("leave", Self::handle_leave)?;

		// handler_wrapper.
		Ok(handler_wrapper)
	}

	fn handle_enter<T: ZoneHandler>(
		zone: Arc<Zone>,
		handler: Arc<Mutex<T>>,
		data: &[u8],
	) -> anyhow::Result<()> {
		let uid: &str = deserialize(data)?;
		let spatial =
			Spatial::from_path(&zone.node().client()?, zone.node().get_path()?, uid, false);
		handler.lock().enter(uid, &spatial);
		Ok(())
	}
	fn handle_capture<T: ZoneHandler>(
		zone: Arc<Zone>,
		handler: Arc<Mutex<T>>,
		data: &[u8],
	) -> anyhow::Result<()> {
		let uid: &str = deserialize(data)?;
		let spatial =
			Spatial::from_path(&zone.node().client()?, zone.node().get_path()?, uid, false);
		zone.captured.lock().insert(uid.to_string(), spatial);
		let captured = zone.captured.lock();
		let spatial = captured.get(uid).unwrap();
		handler.lock().capture(uid, spatial);
		Ok(())
	}
	fn handle_release<T: ZoneHandler>(
		zone: Arc<Zone>,
		handler: Arc<Mutex<T>>,
		data: &[u8],
	) -> anyhow::Result<()> {
		let uid: &str = deserialize(data)?;
		zone.captured.lock().remove(uid);
		handler.lock().release(uid);
		Ok(())
	}
	fn handle_leave<T: ZoneHandler>(
		_zone: Arc<Zone>,
		handler: Arc<Mutex<T>>,
		data: &[u8],
	) -> anyhow::Result<()> {
		let uid: &str = deserialize(data)?;
		handler.lock().leave(uid);
		Ok(())
	}

	pub fn update(&self) -> Result<(), NodeError> {
		self.node.send_remote_signal("update", &())
	}
	pub fn capture(&self, uid: &str) -> Result<(), NodeError> {
		self.node.send_remote_signal("capture", &uid)
	}
	pub fn release(&self, uid: &str) -> Result<(), NodeError> {
		self.node.send_remote_signal("release", &uid)
	}
}
impl NodeType for Zone {
	fn node(&self) -> &Node {
		&self.spatial.node
	}
}
impl std::ops::Deref for Zone {
	type Target = Spatial;

	fn deref(&self) -> &Self::Target {
		&self.spatial
	}
}

#[tokio::test]
async fn fusion_zone() {
	use crate::client::Client;
	let (client, event_loop) = Client::connect_with_async_loop()
		.await
		.expect("Couldn't connect");
	client.set_base_prefixes(&[manifest_dir_macros::directory_relative_path!("res")]);

	let model_parent = Spatial::builder()
		.spatial_parent(client.get_root())
		.zoneable(true)
		.build()
		.unwrap();
	let _model = crate::drawable::Model::builder()
		.spatial_parent(&model_parent)
		.resource(&crate::resource::NamespacedResource::new(
			"fusion",
			"gyro_gem.glb",
		))
		.build()
		.unwrap();

	let field = crate::fields::SphereField::builder()
		.spatial_parent(client.get_root())
		.radius(0.1)
		.build()
		.unwrap();

	struct LifeCycle(HandlerWrapper<Zone, ZoneTest>);
	impl crate::client::LifeCycleHandler for LifeCycle {
		fn logic_step(&mut self, info: crate::client::LogicStepInfo) {
			self.0.node().update().unwrap();
			for (_, spatial) in self.0.node().captured.lock().iter() {
				spatial
					.set_position(None, glam::vec3(0.0, info.elapsed.sin() as f32 * 0.1, 0.0))
					.unwrap();
			}
		}
	}

	struct ZoneTest(Arc<Zone>);
	impl ZoneHandler for ZoneTest {
		fn enter(&mut self, uid: &str, _spatial: &Spatial) {
			println!("Spatial {} entered zone", uid);
			self.0.capture(uid).unwrap();
		}
		fn capture(&mut self, uid: &str, _spatial: &Spatial) {
			println!("Spatial {} was captured", uid);
		}
		fn release(&mut self, uid: &str) {
			println!("Spatial {} was released", uid);
		}
		fn leave(&mut self, uid: &str) {
			println!("Spatial {} left zone", uid);
		}
	}
	let demo = LifeCycle(
		Zone::create(client.get_root(), None, None, &field, |zone| {
			ZoneTest(zone.clone())
		})
		.unwrap(),
	);

	let _handler = client.wrap_root(demo);

	tokio::select! {
		biased;
		_ = tokio::signal::ctrl_c() => (),
		e = event_loop => e.unwrap().unwrap(),
	};
}

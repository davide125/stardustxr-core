use super::Field;
use crate::{
	fusion::{
		node::GenNodeInfo,
		node::{Node, NodeError},
		spatial::Spatial,
	},
	push_to_vec,
	values::{Quat, Vec3, QUAT_IDENTITY, VEC3_ZERO},
};
use anyhow::Result;
use std::ops::Deref;

pub struct CylinderField {
	pub field: Field,
}
#[buildstructor::buildstructor]
impl<'a> CylinderField {
	#[builder(entry = "builder")]
	pub fn create(
		spatial_parent: &'a Spatial,
		position: Option<Vec3>,
		rotation: Option<Quat>,
		length: f32,
		radius: f32,
	) -> Result<Self, NodeError> {
		Ok(CylinderField {
			field: Field {
				spatial: Spatial {
					node: generate_node!(
						GenNodeInfo {
							client: spatial_parent.node.client.clone(),
							parent_path: "/field",
							interface_path: "/field",
							interface_method: "createCylinderField"
						},
						spatial_parent.node.get_path(),
						position.unwrap_or(VEC3_ZERO),
						rotation.unwrap_or(QUAT_IDENTITY),
						length,
						radius
					),
				},
			},
		})
	}
}
impl Deref for CylinderField {
	type Target = Field;

	fn deref(&self) -> &Self::Target {
		&self.field
	}
}

#[tokio::test]
async fn fusion_cylinder_field() {
	use crate::fusion::client::Client;
	let (client, event_loop) = Client::connect_with_async_loop()
		.await
		.expect("Couldn't connect");

	let cylinder_field = CylinderField::builder()
		.spatial_parent(client.get_root())
		.length(1.0)
		.radius(0.5)
		.build()
		.expect("Unable to make cylinder field");
	let distance = cylinder_field
		.field
		.distance(
			client.get_root(),
			mint::Vector3::from([0_f32, 2_f32, 0_f32]),
		)
		.await
		.expect("Unable to cylinder box field distance");
	assert_eq!(distance, 1_f32);

	tokio::select! {
		biased;
		_ = tokio::signal::ctrl_c() => (),
		_ = event_loop => (),
	};
}

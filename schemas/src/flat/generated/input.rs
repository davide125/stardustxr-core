// automatically generated by the FlatBuffers compiler, do not modify


// @generated

use crate::input_pointer::*;
use crate::input_hand::*;
use crate::input_tip::*;
use crate::common::*;
use core::mem;
use core::cmp::Ordering;

extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};

#[allow(unused_imports, dead_code)]
pub mod stardust_xr {

  use crate::input_pointer::*;
  use crate::input_hand::*;
  use crate::input_tip::*;
  use crate::common::*;
  use core::mem;
  use core::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::{EndianScalar, Follow};

#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MIN_INPUT_DATA_RAW: u8 = 0;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MAX_INPUT_DATA_RAW: u8 = 3;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
#[allow(non_camel_case_types)]
pub const ENUM_VALUES_INPUT_DATA_RAW: [InputDataRaw; 4] = [
  InputDataRaw::NONE,
  InputDataRaw::Pointer,
  InputDataRaw::Hand,
  InputDataRaw::Tip,
];

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct InputDataRaw(pub u8);
#[allow(non_upper_case_globals)]
impl InputDataRaw {
  pub const NONE: Self = Self(0);
  pub const Pointer: Self = Self(1);
  pub const Hand: Self = Self(2);
  pub const Tip: Self = Self(3);

  pub const ENUM_MIN: u8 = 0;
  pub const ENUM_MAX: u8 = 3;
  pub const ENUM_VALUES: &'static [Self] = &[
    Self::NONE,
    Self::Pointer,
    Self::Hand,
    Self::Tip,
  ];
  /// Returns the variant's name or "" if unknown.
  pub fn variant_name(self) -> Option<&'static str> {
    match self {
      Self::NONE => Some("NONE"),
      Self::Pointer => Some("Pointer"),
      Self::Hand => Some("Hand"),
      Self::Tip => Some("Tip"),
      _ => None,
    }
  }
}
impl core::fmt::Debug for InputDataRaw {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    if let Some(name) = self.variant_name() {
      f.write_str(name)
    } else {
      f.write_fmt(format_args!("<UNKNOWN {:?}>", self.0))
    }
  }
}
impl<'a> flatbuffers::Follow<'a> for InputDataRaw {
  type Inner = Self;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    let b = flatbuffers::read_scalar_at::<u8>(buf, loc);
    Self(b)
  }
}

impl flatbuffers::Push for InputDataRaw {
    type Output = InputDataRaw;
    #[inline]
    unsafe fn push(&self, dst: &mut [u8], _written_len: usize) {
        flatbuffers::emplace_scalar::<u8>(dst, self.0);
    }
}

impl flatbuffers::EndianScalar for InputDataRaw {
  type Scalar = u8;
  #[inline]
  fn to_little_endian(self) -> u8 {
    self.0.to_le()
  }
  #[inline]
  #[allow(clippy::wrong_self_convention)]
  fn from_little_endian(v: u8) -> Self {
    let b = u8::from_le(v);
    Self(b)
  }
}

impl<'a> flatbuffers::Verifiable for InputDataRaw {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    u8::run_verifier(v, pos)
  }
}

impl flatbuffers::SimpleToVerifyInSlice for InputDataRaw {}
pub struct InputDataRawUnionTableOffset {}

#[allow(clippy::upper_case_acronyms)]
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub enum InputDataRawT {
  NONE,
  Pointer(Box<PointerT>),
  Hand(Box<HandT>),
  Tip(Box<TipT>),
}
impl Default for InputDataRawT {
  fn default() -> Self {
    Self::NONE
  }
}
impl InputDataRawT {
  pub fn input_data_raw_type(&self) -> InputDataRaw {
    match self {
      Self::NONE => InputDataRaw::NONE,
      Self::Pointer(_) => InputDataRaw::Pointer,
      Self::Hand(_) => InputDataRaw::Hand,
      Self::Tip(_) => InputDataRaw::Tip,
    }
  }
  pub fn pack(&self, fbb: &mut flatbuffers::FlatBufferBuilder) -> Option<flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>> {
    match self {
      Self::NONE => None,
      Self::Pointer(v) => Some(v.pack(fbb).as_union_value()),
      Self::Hand(v) => Some(v.pack(fbb).as_union_value()),
      Self::Tip(v) => Some(v.pack(fbb).as_union_value()),
    }
  }
  /// If the union variant matches, return the owned PointerT, setting the union to NONE.
  pub fn take_pointer(&mut self) -> Option<Box<PointerT>> {
    if let Self::Pointer(_) = self {
      let v = core::mem::replace(self, Self::NONE);
      if let Self::Pointer(w) = v {
        Some(w)
      } else {
        unreachable!()
      }
    } else {
      None
    }
  }
  /// If the union variant matches, return a reference to the PointerT.
  pub fn as_pointer(&self) -> Option<&PointerT> {
    if let Self::Pointer(v) = self { Some(v.as_ref()) } else { None }
  }
  /// If the union variant matches, return a mutable reference to the PointerT.
  pub fn as_pointer_mut(&mut self) -> Option<&mut PointerT> {
    if let Self::Pointer(v) = self { Some(v.as_mut()) } else { None }
  }
  /// If the union variant matches, return the owned HandT, setting the union to NONE.
  pub fn take_hand(&mut self) -> Option<Box<HandT>> {
    if let Self::Hand(_) = self {
      let v = core::mem::replace(self, Self::NONE);
      if let Self::Hand(w) = v {
        Some(w)
      } else {
        unreachable!()
      }
    } else {
      None
    }
  }
  /// If the union variant matches, return a reference to the HandT.
  pub fn as_hand(&self) -> Option<&HandT> {
    if let Self::Hand(v) = self { Some(v.as_ref()) } else { None }
  }
  /// If the union variant matches, return a mutable reference to the HandT.
  pub fn as_hand_mut(&mut self) -> Option<&mut HandT> {
    if let Self::Hand(v) = self { Some(v.as_mut()) } else { None }
  }
  /// If the union variant matches, return the owned TipT, setting the union to NONE.
  pub fn take_tip(&mut self) -> Option<Box<TipT>> {
    if let Self::Tip(_) = self {
      let v = core::mem::replace(self, Self::NONE);
      if let Self::Tip(w) = v {
        Some(w)
      } else {
        unreachable!()
      }
    } else {
      None
    }
  }
  /// If the union variant matches, return a reference to the TipT.
  pub fn as_tip(&self) -> Option<&TipT> {
    if let Self::Tip(v) = self { Some(v.as_ref()) } else { None }
  }
  /// If the union variant matches, return a mutable reference to the TipT.
  pub fn as_tip_mut(&mut self) -> Option<&mut TipT> {
    if let Self::Tip(v) = self { Some(v.as_mut()) } else { None }
  }
}
pub enum InputDataOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct InputData<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for InputData<'a> {
  type Inner = InputData<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> InputData<'a> {
  pub const VT_UID: flatbuffers::VOffsetT = 4;
  pub const VT_INPUT_TYPE: flatbuffers::VOffsetT = 6;
  pub const VT_INPUT: flatbuffers::VOffsetT = 8;
  pub const VT_DISTANCE: flatbuffers::VOffsetT = 10;
  pub const VT_DATAMAP: flatbuffers::VOffsetT = 12;
  pub const VT_ORDER: flatbuffers::VOffsetT = 14;
  pub const VT_CAPTURED: flatbuffers::VOffsetT = 16;

  pub const fn get_fully_qualified_name() -> &'static str {
    "StardustXR.InputData"
  }

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    InputData { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args InputDataArgs<'args>
  ) -> flatbuffers::WIPOffset<InputData<'bldr>> {
    let mut builder = InputDataBuilder::new(_fbb);
    builder.add_order(args.order);
    if let Some(x) = args.datamap { builder.add_datamap(x); }
    builder.add_distance(args.distance);
    if let Some(x) = args.input { builder.add_input(x); }
    if let Some(x) = args.uid { builder.add_uid(x); }
    builder.add_captured(args.captured);
    builder.add_input_type(args.input_type);
    builder.finish()
  }

  pub fn unpack(&self) -> InputDataT {
    let uid = {
      let x = self.uid();
      x.to_string()
    };
    let input = match self.input_type() {
      InputDataRaw::NONE => InputDataRawT::NONE,
      InputDataRaw::Pointer => InputDataRawT::Pointer(Box::new(
        self.input_as_pointer()
            .expect("Invalid union table, expected `InputDataRaw::Pointer`.")
            .unpack()
      )),
      InputDataRaw::Hand => InputDataRawT::Hand(Box::new(
        self.input_as_hand()
            .expect("Invalid union table, expected `InputDataRaw::Hand`.")
            .unpack()
      )),
      InputDataRaw::Tip => InputDataRawT::Tip(Box::new(
        self.input_as_tip()
            .expect("Invalid union table, expected `InputDataRaw::Tip`.")
            .unpack()
      )),
      _ => InputDataRawT::NONE,
    };
    let distance = self.distance();
    let datamap = self.datamap().map(|x| {
      x.into_iter().collect()
    });
    let order = self.order();
    let captured = self.captured();
    InputDataT {
      uid,
      input,
      distance,
      datamap,
      order,
      captured,
    }
  }

  #[inline]
  pub fn uid(&self) -> &'a str {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(InputData::VT_UID, None).unwrap()}
  }
  #[inline]
  pub fn input_type(&self) -> InputDataRaw {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<InputDataRaw>(InputData::VT_INPUT_TYPE, Some(InputDataRaw::NONE)).unwrap()}
  }
  #[inline]
  pub fn input(&self) -> flatbuffers::Table<'a> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Table<'a>>>(InputData::VT_INPUT, None).unwrap()}
  }
  #[inline]
  pub fn distance(&self) -> f32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<f32>(InputData::VT_DISTANCE, Some(0.0)).unwrap()}
  }
  #[inline]
  pub fn datamap(&self) -> Option<flatbuffers::Vector<'a, u8>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u8>>>(InputData::VT_DATAMAP, None)}
  }
  #[inline]
  pub fn order(&self) -> u32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u32>(InputData::VT_ORDER, Some(0)).unwrap()}
  }
  #[inline]
  pub fn captured(&self) -> bool {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<bool>(InputData::VT_CAPTURED, Some(false)).unwrap()}
  }
  #[inline]
  #[allow(non_snake_case)]
  pub fn input_as_pointer(&self) -> Option<Pointer<'a>> {
    if self.input_type() == InputDataRaw::Pointer {
      let u = self.input();
      // Safety:
      // Created from a valid Table for this object
      // Which contains a valid union in this slot
      Some(unsafe { Pointer::init_from_table(u) })
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn input_as_hand(&self) -> Option<Hand<'a>> {
    if self.input_type() == InputDataRaw::Hand {
      let u = self.input();
      // Safety:
      // Created from a valid Table for this object
      // Which contains a valid union in this slot
      Some(unsafe { Hand::init_from_table(u) })
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn input_as_tip(&self) -> Option<Tip<'a>> {
    if self.input_type() == InputDataRaw::Tip {
      let u = self.input();
      // Safety:
      // Created from a valid Table for this object
      // Which contains a valid union in this slot
      Some(unsafe { Tip::init_from_table(u) })
    } else {
      None
    }
  }

}

impl flatbuffers::Verifiable for InputData<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("uid", Self::VT_UID, true)?
     .visit_union::<InputDataRaw, _>("input_type", Self::VT_INPUT_TYPE, "input", Self::VT_INPUT, true, |key, v, pos| {
        match key {
          InputDataRaw::Pointer => v.verify_union_variant::<flatbuffers::ForwardsUOffset<Pointer>>("InputDataRaw::Pointer", pos),
          InputDataRaw::Hand => v.verify_union_variant::<flatbuffers::ForwardsUOffset<Hand>>("InputDataRaw::Hand", pos),
          InputDataRaw::Tip => v.verify_union_variant::<flatbuffers::ForwardsUOffset<Tip>>("InputDataRaw::Tip", pos),
          _ => Ok(()),
        }
     })?
     .visit_field::<f32>("distance", Self::VT_DISTANCE, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, u8>>>("datamap", Self::VT_DATAMAP, false)?
     .visit_field::<u32>("order", Self::VT_ORDER, false)?
     .visit_field::<bool>("captured", Self::VT_CAPTURED, false)?
     .finish();
    Ok(())
  }
}
pub struct InputDataArgs<'a> {
    pub uid: Option<flatbuffers::WIPOffset<&'a str>>,
    pub input_type: InputDataRaw,
    pub input: Option<flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>>,
    pub distance: f32,
    pub datamap: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u8>>>,
    pub order: u32,
    pub captured: bool,
}
impl<'a> Default for InputDataArgs<'a> {
  #[inline]
  fn default() -> Self {
    InputDataArgs {
      uid: None, // required field
      input_type: InputDataRaw::NONE,
      input: None, // required field
      distance: 0.0,
      datamap: None,
      order: 0,
      captured: false,
    }
  }
}

pub struct InputDataBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> InputDataBuilder<'a, 'b> {
  #[inline]
  pub fn add_uid(&mut self, uid: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(InputData::VT_UID, uid);
  }
  #[inline]
  pub fn add_input_type(&mut self, input_type: InputDataRaw) {
    self.fbb_.push_slot::<InputDataRaw>(InputData::VT_INPUT_TYPE, input_type, InputDataRaw::NONE);
  }
  #[inline]
  pub fn add_input(&mut self, input: flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(InputData::VT_INPUT, input);
  }
  #[inline]
  pub fn add_distance(&mut self, distance: f32) {
    self.fbb_.push_slot::<f32>(InputData::VT_DISTANCE, distance, 0.0);
  }
  #[inline]
  pub fn add_datamap(&mut self, datamap: flatbuffers::WIPOffset<flatbuffers::Vector<'b , u8>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(InputData::VT_DATAMAP, datamap);
  }
  #[inline]
  pub fn add_order(&mut self, order: u32) {
    self.fbb_.push_slot::<u32>(InputData::VT_ORDER, order, 0);
  }
  #[inline]
  pub fn add_captured(&mut self, captured: bool) {
    self.fbb_.push_slot::<bool>(InputData::VT_CAPTURED, captured, false);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> InputDataBuilder<'a, 'b> {
    let start = _fbb.start_table();
    InputDataBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<InputData<'a>> {
    let o = self.fbb_.end_table(self.start_);
    self.fbb_.required(o, InputData::VT_UID,"uid");
    self.fbb_.required(o, InputData::VT_INPUT,"input");
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for InputData<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("InputData");
      ds.field("uid", &self.uid());
      ds.field("input_type", &self.input_type());
      match self.input_type() {
        InputDataRaw::Pointer => {
          if let Some(x) = self.input_as_pointer() {
            ds.field("input", &x)
          } else {
            ds.field("input", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        InputDataRaw::Hand => {
          if let Some(x) = self.input_as_hand() {
            ds.field("input", &x)
          } else {
            ds.field("input", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        InputDataRaw::Tip => {
          if let Some(x) = self.input_as_tip() {
            ds.field("input", &x)
          } else {
            ds.field("input", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        _ => {
          let x: Option<()> = None;
          ds.field("input", &x)
        },
      };
      ds.field("distance", &self.distance());
      ds.field("datamap", &self.datamap());
      ds.field("order", &self.order());
      ds.field("captured", &self.captured());
      ds.finish()
  }
}
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub struct InputDataT {
  pub uid: String,
  pub input: InputDataRawT,
  pub distance: f32,
  pub datamap: Option<Vec<u8>>,
  pub order: u32,
  pub captured: bool,
}
impl Default for InputDataT {
  fn default() -> Self {
    Self {
      uid: "".to_string(),
      input: InputDataRawT::NONE,
      distance: 0.0,
      datamap: None,
      order: 0,
      captured: false,
    }
  }
}
impl InputDataT {
  pub fn pack<'b>(
    &self,
    _fbb: &mut flatbuffers::FlatBufferBuilder<'b>
  ) -> flatbuffers::WIPOffset<InputData<'b>> {
    let uid = Some({
      let x = &self.uid;
      _fbb.create_string(x)
    });
    let input_type = self.input.input_data_raw_type();
    let input = self.input.pack(_fbb);
    let distance = self.distance;
    let datamap = self.datamap.as_ref().map(|x|{
      _fbb.create_vector(x)
    });
    let order = self.order;
    let captured = self.captured;
    InputData::create(_fbb, &InputDataArgs{
      uid,
      input_type,
      input,
      distance,
      datamap,
      order,
      captured,
    })
  }
}
#[inline]
/// Verifies that a buffer of bytes contains a `InputData`
/// and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_input_data_unchecked`.
pub fn root_as_input_data(buf: &[u8]) -> Result<InputData, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root::<InputData>(buf)
}
#[inline]
/// Verifies that a buffer of bytes contains a size prefixed
/// `InputData` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `size_prefixed_root_as_input_data_unchecked`.
pub fn size_prefixed_root_as_input_data(buf: &[u8]) -> Result<InputData, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root::<InputData>(buf)
}
#[inline]
/// Verifies, with the given options, that a buffer of bytes
/// contains a `InputData` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_input_data_unchecked`.
pub fn root_as_input_data_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<InputData<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root_with_opts::<InputData<'b>>(opts, buf)
}
#[inline]
/// Verifies, with the given verifier options, that a buffer of
/// bytes contains a size prefixed `InputData` and returns
/// it. Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_input_data_unchecked`.
pub fn size_prefixed_root_as_input_data_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<InputData<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root_with_opts::<InputData<'b>>(opts, buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a InputData and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid `InputData`.
pub unsafe fn root_as_input_data_unchecked(buf: &[u8]) -> InputData {
  flatbuffers::root_unchecked::<InputData>(buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a size prefixed InputData and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid size prefixed `InputData`.
pub unsafe fn size_prefixed_root_as_input_data_unchecked(buf: &[u8]) -> InputData {
  flatbuffers::size_prefixed_root_unchecked::<InputData>(buf)
}
#[inline]
pub fn finish_input_data_buffer<'a, 'b>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<InputData<'a>>) {
  fbb.finish(root, None);
}

#[inline]
pub fn finish_size_prefixed_input_data_buffer<'a, 'b>(fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>, root: flatbuffers::WIPOffset<InputData<'a>>) {
  fbb.finish_size_prefixed(root, None);
}
}  // pub mod StardustXR


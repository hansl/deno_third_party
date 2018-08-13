// automatically generated by the FlatBuffers compiler, do not modify


pub mod NamespaceA {
  #[allow(unused_imports)]
  use std::mem;
  #[allow(unused_imports)]
  use std::marker::PhantomData;
  #[allow(unused_imports)]
  #[allow(unreachable_code)]
  extern crate flatbuffers;
  #[allow(unused_imports)]
  use self::flatbuffers::flexbuffers;
  #[allow(unused_imports)]
  use std::cmp::Ordering;
pub mod NamespaceB {
  #[allow(unused_imports)]
  use std::mem;
  #[allow(unused_imports)]
  use std::marker::PhantomData;
  #[allow(unused_imports)]
  #[allow(unreachable_code)]
  extern crate flatbuffers;
  #[allow(unused_imports)]
  use self::flatbuffers::flexbuffers;
  #[allow(unused_imports)]
  use std::cmp::Ordering;

#[repr(i8)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum EnumInNestedNS {
  A = 0,
  B = 1,
  C = 2
}

const EnumValuesEnumInNestedNS:[EnumInNestedNS; 3] = [
  EnumInNestedNS::A,
  EnumInNestedNS::B,
  EnumInNestedNS::C
];

const EnumNamesEnumInNestedNS:[&'static str; 3] = [
    "A",
    "B",
    "C"
];

pub fn EnumNameEnumInNestedNS(e: EnumInNestedNS) -> &'static str {
  let index: usize = e as usize;
  EnumNamesEnumInNestedNS[index]
}

// MANUALLY_ALIGNED_STRUCT(4)
#[repr(C, packed)]
#[derive(Clone, Copy, Default, Debug)]
pub struct StructInNestedNS {
  a_: i32,
  b_: i32,
} // pub struct StructInNestedNS
//impl flatbuffers::GeneratedStruct for StructInNestedNS {}

impl StructInNestedNS {
  pub fn Reset(&mut self) {
    //memset(this, 0, size_of(StructInNestedNS));
  }
  pub fn new(_a: i32, _b: i32) -> Self {
    StructInNestedNS {
      a_: flatbuffers::endian_scalar(_a),
      b_: flatbuffers::endian_scalar(_b),

    }
  }
  pub fn a(&self) -> i32 {
    flatbuffers::endian_scalar(self.a_)
  }
  fn mutate_a(&mut self, _a: i32) {
    flatbuffers::write_scalar(&self.a_, _a);
  }
  pub fn b(&self) -> i32 {
    flatbuffers::endian_scalar(self.b_)
  }
  fn mutate_b(&mut self, _b: i32) {
    flatbuffers::write_scalar(&self.b_, _b);
  }
}
// STRUCT_END(StructInNestedNS, 8);

pub enum TableInNestedNSOffset {}
#[derive(Copy, Clone, PartialEq)]
pub struct TableInNestedNS<'a> {
  pub _tab: flatbuffers::Table<'a>,
  _phantom: PhantomData<&'a ()>,
}
impl<'a> flatbuffers::Follow<'a> for TableInNestedNS<'a> {
    type Inner = TableInNestedNS<'a>;
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self { _tab: flatbuffers::Table { buf: buf, loc: loc }, _phantom: PhantomData }
    }
}
impl<'a> TableInNestedNS<'a> /* private flatbuffers::Table */ {
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        TableInNestedNS {
            _tab: table,
            _phantom: PhantomData,
        }
    }
    pub const VT_FOO: flatbuffers::VOffsetT = 4;

  #[inline]
  pub fn foo(&'a self) -> i32 {
    self._tab.get::<i32>(TableInNestedNS::VT_FOO, Some(0)).unwrap()
  }
}

pub struct TableInNestedNSArgs<'a> {
    pub foo: i32,
    pub _phantom: PhantomData<&'a ()>, // pub for default trait
}
impl<'a> Default for TableInNestedNSArgs<'a> {
    fn default() -> Self {
        TableInNestedNSArgs {
            foo: 0,
            _phantom: PhantomData,
        }
    }
}
pub struct TableInNestedNSBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::Offset<flatbuffers::TableOffset>,
}
impl<'a: 'b, 'b> TableInNestedNSBuilder<'a, 'b> {
  pub fn add_foo(&mut self, foo: i32) {
    self.fbb_.push_slot_scalar::<i32>(TableInNestedNS::VT_FOO, foo, 0);
  }
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> TableInNestedNSBuilder<'a, 'b> {
    let start = _fbb.start_table(1);
    TableInNestedNSBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  // TableInNestedNSBuilder &operator=(const TableInNestedNSBuilder &);
  //pub fn finish<'c>(mut self) -> flatbuffers::Offset<flatbuffers::TableOffset> {
  pub fn finish<'c>(mut self) -> flatbuffers::Offset<TableInNestedNS<'a>> {
    let o = self.fbb_.end_table(self.start_);
    //let o = flatbuffers::Offset::<TableInNestedNS<'a>>::new(end);
    flatbuffers::Offset::new(o.value())
  }
}

#[inline]
pub fn CreateTableInNestedNS<'a: 'b, 'b: 'c, 'c>(
    _fbb: &'c mut flatbuffers::FlatBufferBuilder<'a>,
    args: &'b TableInNestedNSArgs<'b>) -> flatbuffers::Offset<TableInNestedNS<'a>> {
  let mut builder = TableInNestedNSBuilder::new(_fbb);
  builder.add_foo(args.foo);
  builder.finish()
}

}  // pub mod NamespaceB
}  // pub mod NamespaceA


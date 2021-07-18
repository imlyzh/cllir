use std::collections::HashMap;

use sexpr_ir::gast::{Handle, symbol::Symbol};



#[derive(Debug, Clone)]
pub struct TypeBind {
    pub name: Handle<Symbol>,
    pub type_: Handle<Type>,
}


#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum PrimitiveType {
    Str = 0x00,
    Integer = 0x01,
    UInteger = 0x02,
    Float = 0x03,
    Reference = 0x04,
    Char,
    RawChar,
    Pointer,
}


#[derive(Debug, Clone)]
pub enum Type {
    Primitive(PrimitiveType),
    NameReference(Handle<Symbol>),
    Reference(Handle<Type>),
    Pointer(Option<Handle<Type>>),
    Struct(StructType),
    Tuple(TupleType),
    Union(StructType),
    Function(FunctionType),
}


#[derive(Debug, Clone)]
pub struct StructType (pub HashMap<Handle<Symbol>, PrimitiveType>);

#[derive(Debug, Clone)]
pub struct UnionType (pub HashMap<Handle<Symbol>, PrimitiveType>);

#[derive(Debug, Clone)]
pub struct TupleType (pub Vec<Handle<Symbol>>);

#[derive(Debug, Clone)]
pub struct FunctionType (pub Vec<Handle<Symbol>>, pub Handle<Type>);

use sexpr_ir::gast::{Handle, symbol::Symbol};

use crate::types::Type;





#[derive(Debug, Clone)]
pub struct FunctionDef {
    name: Handle<Symbol>,
    params: Vec<(Handle<Symbol>, Handle<Type>)>,
    blocks: Vec<Block>,
}


#[derive(Debug, Clone)]
pub struct Block {
    name: Handle<Symbol>,
    insts: Vec<Inst>,
}


#[derive(Debug, Clone)]
pub struct Register(u16);


#[derive(Debug, Clone)]
pub enum Inst {
    Unreachable,
    Return(Register),
}
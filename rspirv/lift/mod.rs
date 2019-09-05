//! Infrastructure of lifting the data representation (DR) into structured
//! representation (SR).

use crate::{
    dr,
    sr::{
        instructions,
        module::{BasicBlock},
        ops,
        storage::Token,
        InstructionError, OperandError,
        //Constant,
        Type,
    },
};

use std::{
    collections::HashMap,
    hash::BuildHasherDefault,
};

use fxhash::FxHasher;
use spirv;


/// Reverse lookup table that associates SPIR-V <id> with SR tokens.
type LookupMap<T> = HashMap<spirv::Word, Token<T>, BuildHasherDefault<FxHasher>>;

pub struct LiftContext {
    types: LookupMap<Type>,
    //constants: LookupMap<Constant>,
    basic_blocks: LookupMap<BasicBlock>,
}

include!("autogen_context.rs");

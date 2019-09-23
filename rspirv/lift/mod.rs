//! Infrastructure of lifting the data representation (DR) into structured
//! representation (SR).

mod storage;

use self::storage::LiftStorage;
use crate::{
    dr,
    sr::{
        instructions,
        module,
        ops,
        storage::{Storage, Token},
        Type,
    },
};

use spirv;

use std::borrow::Borrow;


struct OpInfo {
    op: Token<ops::Op>,
    ty: Option<Token<Type>>,
}

impl Borrow<Token<ops::Op>> for OpInfo {
    fn borrow(&self) -> &Token<ops::Op> {
        &self.op
    }
}

pub struct LiftContext {
    //current_block: Option<Token<module::Block>>,
    //types: LookupMap<Type>,
    //constants: LookupMap<Constant>,
    blocks: LiftStorage<module::Block>,
    ops: LiftStorage<ops::Op, OpInfo>,
}

include!("autogen_context.rs");

/// Error lifting a data representation of an operand into the structured
/// representation.
#[derive(Clone, Debug)]
pub enum OperandError {
    /// Operand has a wrong type.
    WrongType,
    /// Operand is missing from the list.
    Missing,
}

/// Error lifting a data representation of an instruction.
#[derive(Clone, Debug)]
pub enum InstructionError {
    /// Instruction has a wrong opcode.
    WrongOpcode,
    /// Instruction is missing a result <id> or type.
    MissingResult,
    /// One of the operands can not be lifted.
    Operand(OperandError),
}

impl From<OperandError> for InstructionError {
    fn from(error: OperandError) -> Self {
        InstructionError::Operand(error)
    }
}

/// Error that may occur during the convesion from the data representation
/// of a module into a structured representation.
#[derive(Clone, Debug)]
pub enum ConversionError {
    MissingHeader,
    MissingFunction,
    MissingFunctionType,
    MissingLabel,
    MissingTerminator,
    Instruction(InstructionError),
}

impl From<InstructionError> for ConversionError {
    fn from(error: InstructionError) -> Self {
        ConversionError::Instruction(error)
    }
}

impl LiftContext {
    /// Convert a module from the data representation into structured representation.
    pub fn convert(module: &dr::Module) -> Result<module::Module, ConversionError> {
        let mut context = LiftContext {
            blocks: LiftStorage::new(),
            ops: LiftStorage::new(),
        };
        let mut types = Storage::new();
        let constants = Storage::new();
        let mut functions = Vec::new();
        let entry_points = Vec::new();

        for fun in module.functions.iter() {
            let def = context.lift_function(
                fun.def
                    .as_ref()
                    .ok_or(ConversionError::MissingFunction)?
            )?;
            //TODO: lift function type instruction

            let mut blocks = Vec::with_capacity(fun.blocks.len());
            for block in fun.blocks.iter() {
                let mut arguments = Vec::new();
                for inst in &block.instructions {
                    match inst.class.opcode {
                        spirv::Op::Line => {} // skip line decorations
                        spirv::Op::Phi => {
                            match inst.operands[0] {
                                dr::Operand::IdRef(id) => {
                                    let (_, info) = context.ops.lookup(id);
                                    arguments.push(info.ty
                                        .expect("No return type for Phi source?")
                                    );
                                }
                                _ => return Err(ConversionError::Instruction(
                                    InstructionError::Operand(OperandError::Missing)
                                )),
                            };
                        }
                        _ => break,
                    }
                }

                let terminator = context.lift_terminator(
                    block.instructions
                        .last()
                        .ok_or(ConversionError::MissingTerminator)?
                )?;

                blocks.push(context.blocks.append_id(
                    block.label.as_ref().unwrap().result_id.unwrap(),
                    module::Block {
                        arguments,
                        ops: Vec::new(),
                        terminator,
                    },
                ));
            }

            functions.push(module::Function {
                control: def.function_control,
                result: types.append(Type::Void), //TODO: fty.return_type,
                parameters: Vec::new(),
                blocks,
            });
        }

        Ok(module::Module {
            version: match module.header {
                Some(ref header) => header.version,
                None => return Err(ConversionError::MissingHeader),
            },
            capabilities: module.capabilities
                .iter()
                .map(|cap| context.lift_capability(cap).map(|cap| cap.capability))
                .collect::<Result<_, InstructionError>>()?,
            extensions: Vec::new(),
            ext_inst_imports: Vec::new(),
            memory_model: match module.memory_model {
                Some(ref mm) => context.lift_memory_model(mm)?,
                None => return Err(ConversionError::MissingHeader),
            },
            entry_points,
            types,
            constants,
            blocks: context.blocks.unwrap(),
            ops: context.ops.unwrap(),
            functions,
        })
    }

    fn lookup_jump(&self, destination: spirv::Word) -> module::Jump {
        let (_, block) = self.blocks.lookup(destination);
        module::Jump {
            block: *block,
            arguments: Vec::new(), //TODO
        }
    }
}

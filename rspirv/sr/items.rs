use crate::{
    dr,
    sr::{
        LiftError, Token,
        constants::Constant,
        context::Context,
        instructions,
        ops::{Op, Terminator},
        types::{Type},
    },
};
use spirv;
use std::collections::HashMap;

pub struct EntryPoint {
    pub execution_model: spirv::ExecutionModel,
    pub function: Token<Function>,
    pub name: String,
    //pub interface: Vec<spirv::Word>,
}

pub struct BasicBlock {
    pub ops: Vec<Op>,
    pub terminator: Terminator,
}

pub struct Function {
    pub control: spirv::FunctionControl,
    /// Function result type.
    pub result: Token<Type>,
    /// Function parameters.
    pub parameters: Vec<Token<Type>>,
    /// Basic blocks in this function.
    pub basic_blocks: Vec<BasicBlock>,
}

pub struct Module {
    /// The module header.
    pub header: dr::ModuleHeader,
    /// All OpCapability instructions.
    pub capabilities: Vec<spirv::Capability>,
    /// All OpExtension instructions.
    pub extensions: Vec<String>,
    /// All OpExtInstImport instructions.
    pub ext_inst_imports: Vec<String>,
    /// The OpMemoryModel instruction.
    pub memory_model: instructions::MemoryModel,
    /// All entry point declarations.
    pub entry_points: Vec<EntryPoint>,

    /// All constants.
    pub types: Vec<Type>,
    /// All constants.
    pub constants: Vec<Constant>,
    /// All functions.
    pub functions: Vec<Function>,
}

/// Error that may oocur during the convesion from the data representation
/// of a module into a structured representation.
#[derive(Clone, Debug)]
pub enum ConversionError {
    MissingHeader,
    MissingFunction,
    MissingFunctionType,
    MissingLabel,
    MissingTerminator,
    Lift(LiftError),
}

impl From<LiftError> for ConversionError {
    fn from(error: LiftError) -> Self {
        ConversionError::Lift(error)
    }
}

impl Module {
    /// Convert a module from the data representation into structured.
    pub fn from_data_representation(module: &dr::Module) -> Result<Self, ConversionError> {
        let mut context = Context::new();
        let mut functions = Vec::new();
        let entry_points = Vec::new();
        let mut function_type_map = HashMap::new();

        for inst in module.types_global_values.iter() {
            match inst.class.opcode {
                spirv::Op::TypeFunction => {
                    match inst.result_id {
                        Some(id) => {
                            function_type_map.insert(
                                id,
                                context.lift_type_function(inst)?,
                            );
                        }
                        None => {
                            //warn!("{} without a result <id>", inst.opcode);
                        }
                    }
                }
                _ => {} //TODO
            }
        }

        for fun in module.functions.iter() {
            let def = context.lift_function(
                fun.def
                    .as_ref()
                    .ok_or(ConversionError::MissingFunction)?
            )?;
            let fty = function_type_map
                .get(&def.function_type)
                .ok_or(ConversionError::MissingFunctionType)?;

            let mut basic_blocks = Vec::with_capacity(fun.basic_blocks.len());
            for block in fun.basic_blocks.iter() {
                basic_blocks.push(BasicBlock {
                    ops: Vec::new(),
                    terminator: context.lift_terminator(
                        block.instructions
                            .last()
                            .ok_or(ConversionError::MissingTerminator)?
                    )?,
                });
            }

            functions.push(Function {
                control: def.function_control,
                result: fty.return_type,
                parameters: Vec::new(),
                basic_blocks,
            });
        }

        Ok(Module {
            header: match module.header {
                Some(ref header) => header.clone(),
                None => return Err(ConversionError::MissingHeader),
            },
            capabilities: module.capabilities
                .iter()
                .map(|cap| context.lift_capability(cap).map(|cap| cap.capability))
                .collect::<Result<_, LiftError>>()?,
            extensions: Vec::new(),
            ext_inst_imports: Vec::new(),
            memory_model: match module.memory_model {
                Some(ref mm) => context.lift_memory_model(mm)?,
                None => return Err(ConversionError::MissingHeader),
            },
            entry_points,
            types: Vec::new(),
            constants: Vec::new(),
            functions,
        })
    }
}

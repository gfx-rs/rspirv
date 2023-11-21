use crate::grammar;
use crate::spirv;

use crate::spirv::Word;
use crate::utils::version;
use std::{convert, fmt};

/// Data representation of a SPIR-V module.
///
/// Most of the fields are just vectors of `Instruction`s, but some fields
/// store values decomposed from `Instruction`s for better investigation.
///
/// The order of its fields basically reveal the requirements in the
/// [Logical Layout of a Module](https://goo.gl/2kVnfX) of the SPIR-V
/// of the SPIR-V specification.
#[derive(Clone, Debug, Default)]
pub struct Module {
    /// The module header.
    pub header: Option<ModuleHeader>,
    /// All OpCapability instructions.
    pub capabilities: Vec<Instruction>,
    /// All OpExtension instructions.
    pub extensions: Vec<Instruction>,
    /// All OpExtInstImport instructions.
    pub ext_inst_imports: Vec<Instruction>,
    /// The OpMemoryModel instruction.
    ///
    /// Although it is required by the specification to appear exactly once
    /// per module, we keep it optional here to allow flexibility.
    pub memory_model: Option<Instruction>,
    /// All entry point declarations, using OpEntryPoint.
    pub entry_points: Vec<Instruction>,
    /// All execution mode declarations, using OpExecutionMode.
    pub execution_modes: Vec<Instruction>,
    /// Debug subsection: All OpString, OpSourceExtension, OpSource, and OpSourceContinued.
    pub debug_string_source: Vec<Instruction>,
    /// Debug subsection: All OpName and all OpMemberName.
    pub debug_names: Vec<Instruction>,
    /// Debug subsection: All OpModuleProcessed instructions.
    pub debug_module_processed: Vec<Instruction>,
    /// All annotation instructions.
    pub annotations: Vec<Instruction>,
    /// All types, constants, and global variables.
    ///
    /// As per the specification, they have to be bundled together
    /// because they can depend on one another.
    pub types_global_values: Vec<Instruction>,
    /// All functions.
    pub functions: Vec<Function>,
}

/// Data representation of a SPIR-V module header.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ModuleHeader {
    pub magic_number: Word,
    pub version: Word,
    pub generator: Word,
    pub bound: Word,
    pub reserved_word: Word,
}

/// Data representation of a SPIR-V function.
#[derive(Clone, Debug, Default)]
pub struct Function {
    /// First (defining) instruction in this function.
    pub def: Option<Instruction>,
    /// Last (ending) instruction in this function.
    pub end: Option<Instruction>,
    /// Function parameters.
    pub parameters: Vec<Instruction>,
    /// Blocks in this function.
    pub blocks: Vec<Block>,
}

/// Data representation of a SPIR-V block.
#[derive(Clone, Debug, Default)]
pub struct Block {
    /// The label starting this block.
    pub label: Option<Instruction>,
    /// Instructions in this block.
    pub instructions: Vec<Instruction>,
}

/// Data representation of a SPIR-V instruction.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Instruction {
    /// The class (grammar specification) of this instruction.
    pub class: &'static grammar::Instruction<'static>,
    /// Result type id.
    pub result_type: Option<Word>,
    /// Result id.
    pub result_id: Option<Word>,
    /// Operands.
    pub operands: Vec<Operand>,
}

impl Instruction {
    /// Compare two instructions by opcode and operands; this is the equality identity for `OpType` instructions
    pub fn is_type_identical(&self, other: &Instruction) -> bool {
        self.class.opcode == other.class.opcode && self.operands == other.operands
    }
}

include!("autogen_operand.rs");

impl Module {
    /// Creates a new empty `Module` instance.
    pub fn new() -> Self {
        Module {
            header: None,
            capabilities: vec![],
            extensions: vec![],
            ext_inst_imports: vec![],
            memory_model: None,
            entry_points: vec![],
            execution_modes: vec![],
            debug_string_source: vec![],
            debug_names: vec![],
            debug_module_processed: vec![],
            annotations: vec![],
            types_global_values: vec![],
            functions: vec![],
        }
    }

    /// Returns an iterator over all global instructions.
    pub fn global_inst_iter(&self) -> impl Iterator<Item = &Instruction> {
        self.capabilities
            .iter()
            .chain(&self.extensions)
            .chain(&self.ext_inst_imports)
            .chain(&self.memory_model)
            .chain(&self.entry_points)
            .chain(&self.execution_modes)
            .chain(&self.debug_string_source)
            .chain(&self.debug_names)
            .chain(&self.debug_module_processed)
            .chain(&self.annotations)
            .chain(&self.types_global_values)
    }

    /// Returns a mut iterator over all global instructions.
    pub fn global_inst_iter_mut(&mut self) -> impl Iterator<Item = &mut Instruction> {
        self.capabilities
            .iter_mut()
            .chain(&mut self.extensions)
            .chain(&mut self.ext_inst_imports)
            .chain(&mut self.memory_model)
            .chain(&mut self.entry_points)
            .chain(&mut self.execution_modes)
            .chain(&mut self.debug_string_source)
            .chain(&mut self.debug_names)
            .chain(&mut self.debug_module_processed)
            .chain(&mut self.annotations)
            .chain(&mut self.types_global_values)
    }

    /// Returns a iterator over all instructions.
    pub fn all_inst_iter(&self) -> impl Iterator<Item = &Instruction> {
        self.capabilities
            .iter()
            .chain(&self.extensions)
            .chain(&self.ext_inst_imports)
            .chain(&self.memory_model)
            .chain(&self.entry_points)
            .chain(&self.execution_modes)
            .chain(&self.debug_string_source)
            .chain(&self.debug_names)
            .chain(&self.debug_module_processed)
            .chain(&self.annotations)
            .chain(&self.types_global_values)
            .chain(self.functions.iter().flat_map(|f| f.all_inst_iter()))
    }

    /// Returns a mut iterator over all instructions.
    pub fn all_inst_iter_mut(&mut self) -> impl Iterator<Item = &mut Instruction> {
        self.capabilities
            .iter_mut()
            .chain(&mut self.extensions)
            .chain(&mut self.ext_inst_imports)
            .chain(&mut self.memory_model)
            .chain(&mut self.entry_points)
            .chain(&mut self.execution_modes)
            .chain(&mut self.debug_string_source)
            .chain(&mut self.debug_names)
            .chain(&mut self.debug_module_processed)
            .chain(&mut self.annotations)
            .chain(&mut self.types_global_values)
            .chain(
                self.functions
                    .iter_mut()
                    .flat_map(|f| f.all_inst_iter_mut()),
            )
    }
}

impl ModuleHeader {
    /// Creates a new `ModuleHeader` instance.
    pub fn new(bound: Word) -> ModuleHeader {
        ModuleHeader {
            magic_number: spirv::MAGIC_NUMBER,
            version: version::create_word_from_version(spirv::MAJOR_VERSION, spirv::MINOR_VERSION),
            generator: 0x000f_0000, // TODO: lower 16-bit: tool version number
            bound,
            reserved_word: 0,
        }
    }

    /// Sets the SPIR-V version to the given major.minor version.
    pub fn set_version(&mut self, major: u8, minor: u8) {
        self.version = version::create_word_from_version(major, minor);
    }

    /// Returns the major and minor version numbers as a tuple.
    pub fn version(&self) -> (u8, u8) {
        version::create_version_from_word(self.version)
    }

    /// Returns the generator's name and version as a tuple.
    pub fn generator(&self) -> (&str, u16) {
        let tool = (self.generator & 0xffff_0000) >> 16;
        let version = (self.generator & 0xffff) as u16;
        let tool: &str = match tool {
            0 => "The Khronos Group",
            1 => "LunarG",
            2 => "Valve",
            3 => "Codeplay",
            4 => "NVIDIA",
            5 => "ARM",
            6 => "LLVM/SPIR-V Translator",
            7 => "SPIR-V Tools Assembler",
            8 => "Glslang",
            9 => "Qualcomm",
            10 => "AMD",
            11 => "Intel",
            12 => "Imagination",
            13 => "Shaderc",
            14 => "spiregg",
            15 => "rspirv",
            _ => "Unknown",
        };
        (tool, version)
    }
}

impl Function {
    /// Creates a new empty `Function` instance.
    pub fn new() -> Self {
        Function {
            def: None,
            end: None,
            parameters: vec![],
            blocks: vec![],
        }
    }

    pub fn def_id(&self) -> Option<Word> {
        self.def.as_ref().and_then(|inst| inst.result_id)
    }

    pub fn all_inst_iter(&self) -> impl Iterator<Item = &Instruction> {
        self.def
            .iter()
            .chain(self.parameters.iter())
            .chain(
                self.blocks
                    .iter()
                    .flat_map(|b| b.label.iter().chain(b.instructions.iter())),
            )
            .chain(self.end.iter())
    }

    pub fn all_inst_iter_mut(&mut self) -> impl Iterator<Item = &mut Instruction> {
        self.def
            .iter_mut()
            .chain(self.parameters.iter_mut())
            .chain(
                self.blocks
                    .iter_mut()
                    .flat_map(|b| b.label.iter_mut().chain(b.instructions.iter_mut())),
            )
            .chain(self.end.iter_mut())
    }
}

impl Block {
    /// Creates a new empty `Block` instance.
    pub fn new() -> Self {
        Block {
            label: None,
            instructions: vec![],
        }
    }

    pub fn label_id(&self) -> Option<Word> {
        self.label.as_ref().and_then(|inst| inst.result_id)
    }
}

impl Instruction {
    /// Creates a new `Instruction` instance.
    pub fn new(
        opcode: spirv::Op,
        result_type: Option<Word>,
        result_id: Option<Word>,
        operands: Vec<Operand>,
    ) -> Self {
        Instruction {
            class: grammar::CoreInstructionTable::get(opcode),
            result_type,
            result_id,
            operands,
        }
    }
}

// Sadly cannot use impl<T: Into<String>> here.
impl<'a> convert::From<&'a str> for Operand {
    fn from(val: &'a str) -> Self {
        Operand::LiteralString(val.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use crate::dr;
    use crate::spirv;

    #[test]
    fn test_convert_from_string() {
        assert_eq!(
            dr::Operand::LiteralString("wow".to_string()),
            dr::Operand::from("wow")
        );
        assert_eq!(
            dr::Operand::LiteralString("wow".to_string()),
            dr::Operand::from("wow".to_string())
        );
    }

    #[test]
    fn test_convert_from_numbers() {
        assert_eq!(dr::Operand::LiteralBit32(16u32), dr::Operand::from(16u32));
        assert_eq!(
            dr::Operand::LiteralBit64(128934u64),
            dr::Operand::from(128934u64)
        );
        assert_eq!(
            dr::Operand::LiteralBit32(std::f32::consts::PI.to_bits()),
            dr::Operand::from(std::f32::consts::PI.to_bits())
        );
        assert_eq!(
            dr::Operand::LiteralBit64(10.4235f64.to_bits()),
            dr::Operand::from(10.4235f64.to_bits())
        );
    }

    #[test]
    fn test_convert_from_bit_enums() {
        assert_eq!(
            dr::Operand::LoopControl(spirv::LoopControl::DONT_UNROLL | spirv::LoopControl::UNROLL),
            dr::Operand::from(spirv::LoopControl::DONT_UNROLL | spirv::LoopControl::UNROLL)
        );
        assert_eq!(
            dr::Operand::MemoryAccess(spirv::MemoryAccess::NONE),
            dr::Operand::from(spirv::MemoryAccess::NONE)
        );
    }

    #[test]
    fn test_convert_from_value_enums() {
        assert_eq!(
            dr::Operand::BuiltIn(spirv::BuiltIn::Position),
            dr::Operand::from(spirv::BuiltIn::Position)
        );
        assert_eq!(
            dr::Operand::Capability(spirv::Capability::Pipes),
            dr::Operand::from(spirv::Capability::Pipes)
        );
    }

    #[test]
    fn test_convert_from_op() {
        assert_eq!(
            dr::Operand::LiteralSpecConstantOpInteger(spirv::Op::IAdd),
            dr::Operand::from(spirv::Op::IAdd)
        );
    }

    #[test]
    fn test_operand_display() {
        assert_eq!(
            format!(
                "{}",
                dr::Operand::FunctionControl(
                    spirv::FunctionControl::INLINE | spirv::FunctionControl::CONST
                )
            ),
            "FunctionControl(INLINE | CONST)",
        );
        assert_eq!(format!("{}", dr::Operand::IdRef(3)), "%3");
        assert_eq!(format!("{}", dr::Operand::LiteralBit32(3)), "3");
    }
}

use spirv;

/// A token for representing a SPIR-V instruction.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct InstructionToken {
    index: usize,
}

impl InstructionToken {
    pub fn new(index: usize) -> Self {
        InstructionToken { index: index }
    }

    pub fn get(&self) -> usize {
        self.index
    }
}

include!("autogen_instruction.rs");

use grammar;
use spirv::Word;

#[derive(Debug)]
pub struct Module {
    pub header: Option<ModuleHeader>,
    pub instructions: Vec<Instruction>,
}

#[derive(Debug)]
pub struct ModuleHeader {
    pub magic_number: Word,
    pub version: Word,
    pub generator: Word,
    pub bound: Word,
    pub reserved_word: Word,
}

#[derive(Debug)]
pub struct Instruction {
    pub class: &'static grammar::Instruction<'static>,
    pub result_type: Option<Word>,
    pub result_id: Option<Word>,
    pub operands: Vec<Word>,
}

impl Module {
    pub fn new() -> Module {
        Module {
            header: None,
            instructions: vec![],
        }
    }
}

impl Instruction {
    pub fn new(class: &'static grammar::Instruction<'static>) -> Instruction {
        Instruction {
            class: class,
            result_type: None,
            result_id: None,
            operands: vec![],
        }
    }
}

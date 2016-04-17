use mr;
use spirv;

use grammar::InstructionTable as InstTable;

use std::result;
use std::mem;

pub enum State {
    Normal,
    UnknownOpcode,
}

type Result<T> = result::Result<T, State>;

trait FromWord {
    fn from_word(word: spirv::Word) -> Self;
}

impl FromWord for spirv::Capability {
    fn from_word(word: spirv::Word) -> spirv::Capability {
        unsafe { mem::transmute(word) }
    }
}

pub struct Builder<'a> {
    module: Option<mr::Module<'a>>,
}

impl<'a> Builder<'a> {
    pub fn new() -> Builder<'a> {
        Builder { module: None }
    }

    pub fn initialize(&mut self, header: mr::ModuleHeader) {
        let mut module = mr::Module::new();
        module.header = Some(header);
        self.module = Some(module);
    }

    pub fn add_capability(&mut self, cap: spirv::Word) {
        self.module
            .as_mut()
            .unwrap()
            .capabilities
            .push(spirv::Capability::from_word(cap));
    }

    pub fn add_instruction(&mut self, opcode: u16, operands: Vec<spirv::Word>) -> State {
        assert!(self.module.is_some());
        if let Some(inst) = InstTable::lookup_opcode(opcode) {
            match inst.opcode {
                spirv::Op::Capability => self.add_capability(operands[0]),
                _ => println!("opcode: {:?}, operands: {:?}", opcode, operands),
            }
            State::Normal
        } else {
            State::UnknownOpcode
        }
    }

    pub fn finalize(&mut self) -> Option<mr::Module<'a>> {
        self.module.take()
    }
}

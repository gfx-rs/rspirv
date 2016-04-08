use nom;
use nom::le_u32;

use grammar;
use mr;
use spirv;

type InstTable = grammar::InstructionTable;
type ReaderState = nom::ConsumerState<(), (), nom::Move>;

const WORD_NBYTE: usize = 4;
const HEADER_NBYTE: usize = 5 * WORD_NBYTE;

#[derive(Debug)]
enum Endianness {
    None,
    Small,
    Big,
}

named!(parse_magic_number, tag!(b"\x03\x02\x23\x07"));

named!(parse_header<&[u8], mr::ModuleHeader>,
       chain!(
           magic_number: le_u32 ~
           version: le_u32 ~
           generator: le_u32 ~
           bound: le_u32 ~
           reserved_word: le_u32,
           || {
               mr::ModuleHeader {
                   magic_number: magic_number,
                   version: version,
                   generator: generator,
                   bound: bound,
                   reserved_word: reserved_word,
               }
           }
       )
);

named!(parse_instruction_first_word<&[u8], (u16, u16)>,
    map!(le_u32,
         |word: u32| (
             (word >> 16) as u16,
             (word & 0xffff) as u16,
         )
    )
);

fn get_n_words(input: &[u8], n: usize) -> nom::IResult<&[u8], Vec<spirv::Word>, u32> {
    count!(input, le_u32, n)
}

#[derive(Debug)]
enum SpirvSection {
    Init,
    Header,
    Module,
    Function,
    Block,
    Instruction(&'static grammar::Instruction<'static>, u16),
}

pub struct Reader<'a> {
    state: ReaderState,
    section: SpirvSection,
    endianness: Endianness,
    module: mr::Module<'a>,
}

impl<'a> Reader<'a> {
    pub fn new() -> Reader<'a> {
        Reader {
            state: nom::ConsumerState::Continue(nom::Move::Consume(0)),
            section: SpirvSection::Init,
            endianness: Endianness::None,
            module: mr::Module::new(),
        }
    }

    fn peek_magic_number(&mut self, input: &[u8]) {
        self.state = match parse_magic_number(input) {
            nom::IResult::Done(_, _) => {
                self.endianness = Endianness::Small;
                self.section = SpirvSection::Header;
                nom::ConsumerState::Continue(nom::Move::Consume(0))
            }
            nom::IResult::Incomplete(needed) => {
                match needed {
                    nom::Needed::Unknown => nom::ConsumerState::Error(()),
                    nom::Needed::Size(_) => nom::ConsumerState::Continue(nom::Move::Await(needed)),
                }
            }
            nom::IResult::Error(_) => nom::ConsumerState::Error(()),
        }
    }

    fn consume_header(&mut self, input: &[u8]) {
        self.state = match parse_header(input) {
            nom::IResult::Done(_, header) => {
                println!("Header: {:?}", header);
                self.section = SpirvSection::Module;
                nom::ConsumerState::Continue(nom::Move::Consume(HEADER_NBYTE))
            }
            nom::IResult::Incomplete(needed) => {
                match needed {
                    nom::Needed::Unknown => nom::ConsumerState::Error(()),
                    nom::Needed::Size(_) => nom::ConsumerState::Continue(nom::Move::Await(needed)),
                }
            }
            nom::IResult::Error(_) => nom::ConsumerState::Error(()),
        }
    }

    fn consume_module(&mut self, input: &[u8]) {
        self.state = match parse_instruction_first_word(input) {
            nom::IResult::Done(_, (wcount, opcode)) => {
                println!("Wcount: {} Opcode: {}", wcount, opcode);
                match InstTable::lookup_opcode(opcode) {
                    Some(inst) => {
                        println!("{:?}", inst);
                        self.section = SpirvSection::Instruction(inst, wcount - 1);
                        nom::ConsumerState::Continue(nom::Move::Consume(WORD_NBYTE))
                    }
                    None => nom::ConsumerState::Error(()),
                }
            }
            nom::IResult::Incomplete(needed) => {
                match needed {
                    nom::Needed::Unknown => nom::ConsumerState::Error(()),
                    nom::Needed::Size(_) => nom::ConsumerState::Continue(nom::Move::Await(needed)),
                }
            }
            nom::IResult::Error(_) => nom::ConsumerState::Error(()),
        }
    }

    fn consume_instruction(&mut self,
                           input: &[u8],
                           inst: &'static grammar::Instruction<'static>,
                           word_count: u16) {
        self.state = match get_n_words(input, word_count as usize) {
            nom::IResult::Done(_, operands) => {
                println!("Operands: {:?}", operands);
                let mut instance = mr::Instruction::new(inst);
                instance.operands = operands;
                self.module.instructions.push(instance);
                self.section = SpirvSection::Module;
                nom::ConsumerState::Continue(nom::Move::Consume(WORD_NBYTE * word_count as usize))
            }
            nom::IResult::Incomplete(needed) => {
                match needed {
                    nom::Needed::Unknown => nom::ConsumerState::Error(()),
                    nom::Needed::Size(_) => nom::ConsumerState::Continue(nom::Move::Await(needed)),
                }
            }
            nom::IResult::Error(_) => nom::ConsumerState::Error(()),
        }
    }
}

impl<'a, 'i> nom::Consumer<&'i [u8], (), (), nom::Move> for Reader<'a> {
    fn handle(&mut self, input: nom::Input<&'i [u8]>) -> &ReaderState {
        match input {
            nom::Input::Element(data) | nom::Input::Eof(Some(data)) => {
                match self.section {
                    SpirvSection::Init => self.peek_magic_number(data),
                    SpirvSection::Header => self.consume_header(data),
                    SpirvSection::Module => self.consume_module(data),
                    SpirvSection::Instruction(inst, size) => {
                        self.consume_instruction(data, inst, size)
                    }
                    _ => unimplemented!(),
                }
            }
            nom::Input::Empty | nom::Input::Eof(None) => {
                match self.section {
                    SpirvSection::Init | SpirvSection::Header | SpirvSection::Module => {
                        self.state = nom::ConsumerState::Done(nom::Move::Consume(0), ())
                    }
                    SpirvSection::Instruction(inst, size) => {
                        self.state = nom::ConsumerState::Error(())
                    }
                    _ => unimplemented!(),
                }
            }
        }
        &self.state
    }

    fn state(&self) -> &ReaderState {
        &self.state
    }
}

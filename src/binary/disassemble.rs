use mr;

pub trait Disassemble {
    fn disassemble(&self) -> String;
}

impl Disassemble for mr::ModuleHeader {
    fn disassemble(&self) -> String {
        let (major, minor) = self.version();
        let (vendor, _) = self.generator();
        format!("; SPIR-V\n; Version: {}.{}\n; Generator: {}\n; Bound: {}",
                major,
                minor,
                vendor,
                self.bound())
    }
}

impl Disassemble for mr::Operand {
    fn disassemble(&self) -> String {
        format!("{}", self)
    }
}

fn disas_join<T: Disassemble>(insts: &Vec<T>, delimiter: &str) -> String {
    insts.iter().map(|ref i| i.disassemble()).collect::<Vec<String>>().join(delimiter)
}

impl Disassemble for mr::Instruction {
    fn disassemble(&self) -> String {
        format!("{rid}{opcode}{rtype} {operands}",
                rid = self.result_id.map_or(String::new(), |w| format!("%{} = ", w)),
                opcode = format!("Op{}", self.class.opname),
                // extra space both before and after the reseult type
                rtype = self.result_type.map_or(String::new(), |w| format!("  %{} ", w)),
                operands = disas_join(&self.operands, " "))
    }
}

impl Disassemble for mr::BasicBlock {
    fn disassemble(&self) -> String {
        format!("{label}\n{insts}",
                label = self.label.disassemble(),
                insts = disas_join(&self.instructions, "\n"))
    }
}

impl Disassemble for mr::Function {
    fn disassemble(&self) -> String {
        if self.parameters.is_empty() {
            format!("{def}\n{blocks}\n{end}",
                    def = self.def.as_ref().unwrap().disassemble(),
                    blocks = disas_join(&self.basic_blocks, "\n"),
                    end = self.end.as_ref().unwrap().disassemble())
        } else {
            format!("{def}\n{params}\n{blocks}\n{end}",
                    def = self.def.as_ref().unwrap().disassemble(),
                    params = disas_join(&self.parameters, "\n"),
                    blocks = disas_join(&self.basic_blocks, "\n"),
                    end = self.end.as_ref().unwrap().disassemble())
        }
    }
}

fn push_if_non_empty(vec: &mut Vec<String>, st: String) {
    if !st.is_empty() {
        vec.push(st)
    }
}

impl Disassemble for mr::Module {
    fn disassemble(&self) -> String {
        let mut text = vec![];
        push_if_non_empty(&mut text, self.header.as_ref().unwrap().disassemble());
        push_if_non_empty(&mut text,
                          self.capabilities
                              .iter()
                              .map(|c| format!("OpCapability {:?}", c))
                              .collect::<Vec<String>>()
                              .join("\n"));
        push_if_non_empty(&mut text,
                          self.extensions
                              .iter()
                              .map(|e| format!("OpExtension {:?}", e))
                              .collect::<Vec<String>>()
                              .join("\n"));
        push_if_non_empty(&mut text, disas_join(&self.ext_inst_imports, "\n"));
        let (address, memory) = self.memory_model.unwrap();
        push_if_non_empty(&mut text,
                          format!("OpMemoryModel {:?} {:?}", address, memory));
        push_if_non_empty(&mut text, disas_join(&self.entry_points, "\n"));
        push_if_non_empty(&mut text, disas_join(&self.execution_modes, "\n"));
        push_if_non_empty(&mut text, disas_join(&self.debugs, "\n"));
        push_if_non_empty(&mut text,
                          self.names
                              .iter()
                              .map(|(k, v)| format!("OpName {} {:?}", k, v))
                              .collect::<Vec<String>>()
                              .join("\n"));
        // names
        push_if_non_empty(&mut text, disas_join(&self.annotations, "\n"));
        push_if_non_empty(&mut text, disas_join(&self.types_global_values, "\n"));
        push_if_non_empty(&mut text, disas_join(&self.functions, "\n"));
        text.join("\n")
    }
}

// Copyright 2016 Google Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// AUTOMATICALLY GENERATED from the SPIR-V JSON grammar:
//   external/spirv.core.grammar.json.
// DO NOT MODIFY!

impl Context {
    pub fn lift_type_function(
        &mut self,
        raw: &mr::Instruction,
    ) -> Result<types::Function, LiftError> {
        if raw.class.opcode as u32 != 33u32 {
            return Err(LiftError::OpCode);
        }
        let mut operands = raw.operands.iter();
        Ok(types::Function {
            decorations: Vec::new(),
            return_type: (match operands.next() {
                Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                None => None,
                Some(_) => Err(OperandError::Wrong)?,
            })
            .ok_or(OperandError::Missing)?,
            parameter_types: {
                let mut vec = Vec::new();
                while let Some(value) = match operands.next() {
                    Some(&mr::Operand::IdRef(ref value)) => Some(Token::new(*value)),
                    None => None,
                    Some(_) => Err(OperandError::Wrong)?,
                } {
                    vec.push(value);
                }
                vec
            },
        })
    }
}

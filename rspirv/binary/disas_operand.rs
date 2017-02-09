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

impl Disassemble for spirv::ImageOperands {
    fn disassemble(&self) -> String {
        if self.is_empty() { return "None".to_string() }
        let mut bits = vec![];
        if self.contains(spirv::IMAGE_OPERANDS_BIAS) { bits.push("Bias") }
        if self.contains(spirv::IMAGE_OPERANDS_LOD) { bits.push("Lod") }
        if self.contains(spirv::IMAGE_OPERANDS_GRAD) { bits.push("Grad") }
        if self.contains(spirv::IMAGE_OPERANDS_CONST_OFFSET) { bits.push("ConstOffset") }
        if self.contains(spirv::IMAGE_OPERANDS_OFFSET) { bits.push("Offset") }
        if self.contains(spirv::IMAGE_OPERANDS_CONST_OFFSETS) { bits.push("ConstOffsets") }
        if self.contains(spirv::IMAGE_OPERANDS_SAMPLE) { bits.push("Sample") }
        if self.contains(spirv::IMAGE_OPERANDS_MIN_LOD) { bits.push("MinLod") }
        bits.join("|")
    }
}

impl Disassemble for spirv::FPFastMathMode {
    fn disassemble(&self) -> String {
        if self.is_empty() { return "None".to_string() }
        let mut bits = vec![];
        if self.contains(spirv::FPFAST_MATH_MODE_NOT_NA_N) { bits.push("NotNaN") }
        if self.contains(spirv::FPFAST_MATH_MODE_NOT_INF) { bits.push("NotInf") }
        if self.contains(spirv::FPFAST_MATH_MODE_NSZ) { bits.push("NSZ") }
        if self.contains(spirv::FPFAST_MATH_MODE_ALLOW_RECIP) { bits.push("AllowRecip") }
        if self.contains(spirv::FPFAST_MATH_MODE_FAST) { bits.push("Fast") }
        bits.join("|")
    }
}

impl Disassemble for spirv::SelectionControl {
    fn disassemble(&self) -> String {
        if self.is_empty() { return "None".to_string() }
        let mut bits = vec![];
        if self.contains(spirv::SELECTION_CONTROL_FLATTEN) { bits.push("Flatten") }
        if self.contains(spirv::SELECTION_CONTROL_DONT_FLATTEN) { bits.push("DontFlatten") }
        bits.join("|")
    }
}

impl Disassemble for spirv::LoopControl {
    fn disassemble(&self) -> String {
        if self.is_empty() { return "None".to_string() }
        let mut bits = vec![];
        if self.contains(spirv::LOOP_CONTROL_UNROLL) { bits.push("Unroll") }
        if self.contains(spirv::LOOP_CONTROL_DONT_UNROLL) { bits.push("DontUnroll") }
        if self.contains(spirv::LOOP_CONTROL_DEPENDENCY_INFINITE) { bits.push("DependencyInfinite") }
        if self.contains(spirv::LOOP_CONTROL_DEPENDENCY_LENGTH) { bits.push("DependencyLength") }
        bits.join("|")
    }
}

impl Disassemble for spirv::FunctionControl {
    fn disassemble(&self) -> String {
        if self.is_empty() { return "None".to_string() }
        let mut bits = vec![];
        if self.contains(spirv::FUNCTION_CONTROL_INLINE) { bits.push("Inline") }
        if self.contains(spirv::FUNCTION_CONTROL_DONT_INLINE) { bits.push("DontInline") }
        if self.contains(spirv::FUNCTION_CONTROL_PURE) { bits.push("Pure") }
        if self.contains(spirv::FUNCTION_CONTROL_CONST) { bits.push("Const") }
        bits.join("|")
    }
}

impl Disassemble for spirv::MemorySemantics {
    fn disassemble(&self) -> String {
        if self.is_empty() { return "None".to_string() }
        let mut bits = vec![];
        if self.contains(spirv::MEMORY_SEMANTICS_ACQUIRE) { bits.push("Acquire") }
        if self.contains(spirv::MEMORY_SEMANTICS_RELEASE) { bits.push("Release") }
        if self.contains(spirv::MEMORY_SEMANTICS_ACQUIRE_RELEASE) { bits.push("AcquireRelease") }
        if self.contains(spirv::MEMORY_SEMANTICS_SEQUENTIALLY_CONSISTENT) { bits.push("SequentiallyConsistent") }
        if self.contains(spirv::MEMORY_SEMANTICS_UNIFORM_MEMORY) { bits.push("UniformMemory") }
        if self.contains(spirv::MEMORY_SEMANTICS_SUBGROUP_MEMORY) { bits.push("SubgroupMemory") }
        if self.contains(spirv::MEMORY_SEMANTICS_WORKGROUP_MEMORY) { bits.push("WorkgroupMemory") }
        if self.contains(spirv::MEMORY_SEMANTICS_CROSS_WORKGROUP_MEMORY) { bits.push("CrossWorkgroupMemory") }
        if self.contains(spirv::MEMORY_SEMANTICS_ATOMIC_COUNTER_MEMORY) { bits.push("AtomicCounterMemory") }
        if self.contains(spirv::MEMORY_SEMANTICS_IMAGE_MEMORY) { bits.push("ImageMemory") }
        bits.join("|")
    }
}

impl Disassemble for spirv::MemoryAccess {
    fn disassemble(&self) -> String {
        if self.is_empty() { return "None".to_string() }
        let mut bits = vec![];
        if self.contains(spirv::MEMORY_ACCESS_VOLATILE) { bits.push("Volatile") }
        if self.contains(spirv::MEMORY_ACCESS_ALIGNED) { bits.push("Aligned") }
        if self.contains(spirv::MEMORY_ACCESS_NONTEMPORAL) { bits.push("Nontemporal") }
        bits.join("|")
    }
}

impl Disassemble for spirv::KernelProfilingInfo {
    fn disassemble(&self) -> String {
        if self.is_empty() { return "None".to_string() }
        let mut bits = vec![];
        if self.contains(spirv::KERNEL_PROFILING_INFO_CMD_EXEC_TIME) { bits.push("CmdExecTime") }
        bits.join("|")
    }
}
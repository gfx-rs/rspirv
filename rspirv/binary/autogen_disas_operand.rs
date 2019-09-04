// AUTOMATICALLY GENERATED from the SPIR-V JSON grammar:
//   external/spirv.core.grammar.json.
// DO NOT MODIFY!

impl Disassemble for spirv::ImageOperands {
    fn disassemble(&self) -> String {
        if self.is_empty() {
            return "None".to_string();
        }
        let mut bits = vec![];
        if self.contains(spirv::ImageOperands::BIAS) {
            bits.push("Bias")
        }
        if self.contains(spirv::ImageOperands::LOD) {
            bits.push("Lod")
        }
        if self.contains(spirv::ImageOperands::GRAD) {
            bits.push("Grad")
        }
        if self.contains(spirv::ImageOperands::CONST_OFFSET) {
            bits.push("ConstOffset")
        }
        if self.contains(spirv::ImageOperands::OFFSET) {
            bits.push("Offset")
        }
        if self.contains(spirv::ImageOperands::CONST_OFFSETS) {
            bits.push("ConstOffsets")
        }
        if self.contains(spirv::ImageOperands::SAMPLE) {
            bits.push("Sample")
        }
        if self.contains(spirv::ImageOperands::MIN_LOD) {
            bits.push("MinLod")
        }
        if self.contains(spirv::ImageOperands::MAKE_TEXEL_AVAILABLE_KHR) {
            bits.push("MakeTexelAvailableKHR")
        }
        if self.contains(spirv::ImageOperands::MAKE_TEXEL_VISIBLE_KHR) {
            bits.push("MakeTexelVisibleKHR")
        }
        if self.contains(spirv::ImageOperands::NON_PRIVATE_TEXEL_KHR) {
            bits.push("NonPrivateTexelKHR")
        }
        if self.contains(spirv::ImageOperands::VOLATILE_TEXEL_KHR) {
            bits.push("VolatileTexelKHR")
        }
        bits.join("|")
    }
}
impl Disassemble for spirv::FPFastMathMode {
    fn disassemble(&self) -> String {
        if self.is_empty() {
            return "None".to_string();
        }
        let mut bits = vec![];
        if self.contains(spirv::FPFastMathMode::NOT_NAN) {
            bits.push("NotNaN")
        }
        if self.contains(spirv::FPFastMathMode::NOT_INF) {
            bits.push("NotInf")
        }
        if self.contains(spirv::FPFastMathMode::NSZ) {
            bits.push("NSZ")
        }
        if self.contains(spirv::FPFastMathMode::ALLOW_RECIP) {
            bits.push("AllowRecip")
        }
        if self.contains(spirv::FPFastMathMode::FAST) {
            bits.push("Fast")
        }
        bits.join("|")
    }
}
impl Disassemble for spirv::SelectionControl {
    fn disassemble(&self) -> String {
        if self.is_empty() {
            return "None".to_string();
        }
        let mut bits = vec![];
        if self.contains(spirv::SelectionControl::FLATTEN) {
            bits.push("Flatten")
        }
        if self.contains(spirv::SelectionControl::DONT_FLATTEN) {
            bits.push("DontFlatten")
        }
        bits.join("|")
    }
}
impl Disassemble for spirv::LoopControl {
    fn disassemble(&self) -> String {
        if self.is_empty() {
            return "None".to_string();
        }
        let mut bits = vec![];
        if self.contains(spirv::LoopControl::UNROLL) {
            bits.push("Unroll")
        }
        if self.contains(spirv::LoopControl::DONT_UNROLL) {
            bits.push("DontUnroll")
        }
        if self.contains(spirv::LoopControl::DEPENDENCY_INFINITE) {
            bits.push("DependencyInfinite")
        }
        if self.contains(spirv::LoopControl::DEPENDENCY_LENGTH) {
            bits.push("DependencyLength")
        }
        bits.join("|")
    }
}
impl Disassemble for spirv::FunctionControl {
    fn disassemble(&self) -> String {
        if self.is_empty() {
            return "None".to_string();
        }
        let mut bits = vec![];
        if self.contains(spirv::FunctionControl::INLINE) {
            bits.push("Inline")
        }
        if self.contains(spirv::FunctionControl::DONT_INLINE) {
            bits.push("DontInline")
        }
        if self.contains(spirv::FunctionControl::PURE) {
            bits.push("Pure")
        }
        if self.contains(spirv::FunctionControl::CONST) {
            bits.push("Const")
        }
        bits.join("|")
    }
}
impl Disassemble for spirv::MemorySemantics {
    fn disassemble(&self) -> String {
        if self.is_empty() {
            return "None".to_string();
        }
        let mut bits = vec![];
        if self.contains(spirv::MemorySemantics::ACQUIRE) {
            bits.push("Acquire")
        }
        if self.contains(spirv::MemorySemantics::RELEASE) {
            bits.push("Release")
        }
        if self.contains(spirv::MemorySemantics::ACQUIRE_RELEASE) {
            bits.push("AcquireRelease")
        }
        if self.contains(spirv::MemorySemantics::SEQUENTIALLY_CONSISTENT) {
            bits.push("SequentiallyConsistent")
        }
        if self.contains(spirv::MemorySemantics::UNIFORM_MEMORY) {
            bits.push("UniformMemory")
        }
        if self.contains(spirv::MemorySemantics::SUBGROUP_MEMORY) {
            bits.push("SubgroupMemory")
        }
        if self.contains(spirv::MemorySemantics::WORKGROUP_MEMORY) {
            bits.push("WorkgroupMemory")
        }
        if self.contains(spirv::MemorySemantics::CROSS_WORKGROUP_MEMORY) {
            bits.push("CrossWorkgroupMemory")
        }
        if self.contains(spirv::MemorySemantics::ATOMIC_COUNTER_MEMORY) {
            bits.push("AtomicCounterMemory")
        }
        if self.contains(spirv::MemorySemantics::IMAGE_MEMORY) {
            bits.push("ImageMemory")
        }
        if self.contains(spirv::MemorySemantics::OUTPUT_MEMORY_KHR) {
            bits.push("OutputMemoryKHR")
        }
        if self.contains(spirv::MemorySemantics::MAKE_AVAILABLE_KHR) {
            bits.push("MakeAvailableKHR")
        }
        if self.contains(spirv::MemorySemantics::MAKE_VISIBLE_KHR) {
            bits.push("MakeVisibleKHR")
        }
        bits.join("|")
    }
}
impl Disassemble for spirv::MemoryAccess {
    fn disassemble(&self) -> String {
        if self.is_empty() {
            return "None".to_string();
        }
        let mut bits = vec![];
        if self.contains(spirv::MemoryAccess::VOLATILE) {
            bits.push("Volatile")
        }
        if self.contains(spirv::MemoryAccess::ALIGNED) {
            bits.push("Aligned")
        }
        if self.contains(spirv::MemoryAccess::NONTEMPORAL) {
            bits.push("Nontemporal")
        }
        if self.contains(spirv::MemoryAccess::MAKE_POINTER_AVAILABLE_KHR) {
            bits.push("MakePointerAvailableKHR")
        }
        if self.contains(spirv::MemoryAccess::MAKE_POINTER_VISIBLE_KHR) {
            bits.push("MakePointerVisibleKHR")
        }
        if self.contains(spirv::MemoryAccess::NON_PRIVATE_POINTER_KHR) {
            bits.push("NonPrivatePointerKHR")
        }
        bits.join("|")
    }
}
impl Disassemble for spirv::KernelProfilingInfo {
    fn disassemble(&self) -> String {
        if self.is_empty() {
            return "None".to_string();
        }
        let mut bits = vec![];
        if self.contains(spirv::KernelProfilingInfo::CMD_EXEC_TIME) {
            bits.push("CmdExecTime")
        }
        bits.join("|")
    }
}

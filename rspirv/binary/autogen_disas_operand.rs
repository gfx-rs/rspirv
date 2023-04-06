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
        if self.contains(spirv::ImageOperands::MAKE_TEXEL_AVAILABLE) {
            bits.push("MakeTexelAvailable")
        }
        if self.contains(spirv::ImageOperands::MAKE_TEXEL_AVAILABLE_KHR) {
            bits.push("MakeTexelAvailableKHR")
        }
        if self.contains(spirv::ImageOperands::MAKE_TEXEL_VISIBLE) {
            bits.push("MakeTexelVisible")
        }
        if self.contains(spirv::ImageOperands::MAKE_TEXEL_VISIBLE_KHR) {
            bits.push("MakeTexelVisibleKHR")
        }
        if self.contains(spirv::ImageOperands::NON_PRIVATE_TEXEL) {
            bits.push("NonPrivateTexel")
        }
        if self.contains(spirv::ImageOperands::NON_PRIVATE_TEXEL_KHR) {
            bits.push("NonPrivateTexelKHR")
        }
        if self.contains(spirv::ImageOperands::VOLATILE_TEXEL) {
            bits.push("VolatileTexel")
        }
        if self.contains(spirv::ImageOperands::VOLATILE_TEXEL_KHR) {
            bits.push("VolatileTexelKHR")
        }
        if self.contains(spirv::ImageOperands::SIGN_EXTEND) {
            bits.push("SignExtend")
        }
        if self.contains(spirv::ImageOperands::ZERO_EXTEND) {
            bits.push("ZeroExtend")
        }
        if self.contains(spirv::ImageOperands::NONTEMPORAL) {
            bits.push("Nontemporal")
        }
        if self.contains(spirv::ImageOperands::OFFSETS) {
            bits.push("Offsets")
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
        if self.contains(spirv::FPFastMathMode::ALLOW_CONTRACT_FAST_INTEL) {
            bits.push("AllowContractFastINTEL")
        }
        if self.contains(spirv::FPFastMathMode::ALLOW_REASSOC_INTEL) {
            bits.push("AllowReassocINTEL")
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
        if self.contains(spirv::LoopControl::MIN_ITERATIONS) {
            bits.push("MinIterations")
        }
        if self.contains(spirv::LoopControl::MAX_ITERATIONS) {
            bits.push("MaxIterations")
        }
        if self.contains(spirv::LoopControl::ITERATION_MULTIPLE) {
            bits.push("IterationMultiple")
        }
        if self.contains(spirv::LoopControl::PEEL_COUNT) {
            bits.push("PeelCount")
        }
        if self.contains(spirv::LoopControl::PARTIAL_COUNT) {
            bits.push("PartialCount")
        }
        if self.contains(spirv::LoopControl::INITIATION_INTERVAL_INTEL) {
            bits.push("InitiationIntervalINTEL")
        }
        if self.contains(spirv::LoopControl::MAX_CONCURRENCY_INTEL) {
            bits.push("MaxConcurrencyINTEL")
        }
        if self.contains(spirv::LoopControl::DEPENDENCY_ARRAY_INTEL) {
            bits.push("DependencyArrayINTEL")
        }
        if self.contains(spirv::LoopControl::PIPELINE_ENABLE_INTEL) {
            bits.push("PipelineEnableINTEL")
        }
        if self.contains(spirv::LoopControl::LOOP_COALESCE_INTEL) {
            bits.push("LoopCoalesceINTEL")
        }
        if self.contains(spirv::LoopControl::MAX_INTERLEAVING_INTEL) {
            bits.push("MaxInterleavingINTEL")
        }
        if self.contains(spirv::LoopControl::SPECULATED_ITERATIONS_INTEL) {
            bits.push("SpeculatedIterationsINTEL")
        }
        if self.contains(spirv::LoopControl::NO_FUSION_INTEL) {
            bits.push("NoFusionINTEL")
        }
        if self.contains(spirv::LoopControl::LOOP_COUNT_INTEL) {
            bits.push("LoopCountINTEL")
        }
        if self.contains(spirv::LoopControl::MAX_REINVOCATION_DELAY_INTEL) {
            bits.push("MaxReinvocationDelayINTEL")
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
        if self.contains(spirv::FunctionControl::OPT_NONE_INTEL) {
            bits.push("OptNoneINTEL")
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
        if self.contains(spirv::MemorySemantics::OUTPUT_MEMORY) {
            bits.push("OutputMemory")
        }
        if self.contains(spirv::MemorySemantics::OUTPUT_MEMORY_KHR) {
            bits.push("OutputMemoryKHR")
        }
        if self.contains(spirv::MemorySemantics::MAKE_AVAILABLE) {
            bits.push("MakeAvailable")
        }
        if self.contains(spirv::MemorySemantics::MAKE_AVAILABLE_KHR) {
            bits.push("MakeAvailableKHR")
        }
        if self.contains(spirv::MemorySemantics::MAKE_VISIBLE) {
            bits.push("MakeVisible")
        }
        if self.contains(spirv::MemorySemantics::MAKE_VISIBLE_KHR) {
            bits.push("MakeVisibleKHR")
        }
        if self.contains(spirv::MemorySemantics::VOLATILE) {
            bits.push("Volatile")
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
        if self.contains(spirv::MemoryAccess::MAKE_POINTER_AVAILABLE) {
            bits.push("MakePointerAvailable")
        }
        if self.contains(spirv::MemoryAccess::MAKE_POINTER_AVAILABLE_KHR) {
            bits.push("MakePointerAvailableKHR")
        }
        if self.contains(spirv::MemoryAccess::MAKE_POINTER_VISIBLE) {
            bits.push("MakePointerVisible")
        }
        if self.contains(spirv::MemoryAccess::MAKE_POINTER_VISIBLE_KHR) {
            bits.push("MakePointerVisibleKHR")
        }
        if self.contains(spirv::MemoryAccess::NON_PRIVATE_POINTER) {
            bits.push("NonPrivatePointer")
        }
        if self.contains(spirv::MemoryAccess::NON_PRIVATE_POINTER_KHR) {
            bits.push("NonPrivatePointerKHR")
        }
        if self.contains(spirv::MemoryAccess::ALIAS_SCOPE_INTEL_MASK) {
            bits.push("AliasScopeINTELMask")
        }
        if self.contains(spirv::MemoryAccess::NO_ALIAS_INTEL_MASK) {
            bits.push("NoAliasINTELMask")
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
impl Disassemble for spirv::RayFlags {
    fn disassemble(&self) -> String {
        if self.is_empty() {
            return "None".to_string();
        }
        let mut bits = vec![];
        if self.contains(spirv::RayFlags::OPAQUE_KHR) {
            bits.push("OpaqueKHR")
        }
        if self.contains(spirv::RayFlags::NO_OPAQUE_KHR) {
            bits.push("NoOpaqueKHR")
        }
        if self.contains(spirv::RayFlags::TERMINATE_ON_FIRST_HIT_KHR) {
            bits.push("TerminateOnFirstHitKHR")
        }
        if self.contains(spirv::RayFlags::SKIP_CLOSEST_HIT_SHADER_KHR) {
            bits.push("SkipClosestHitShaderKHR")
        }
        if self.contains(spirv::RayFlags::CULL_BACK_FACING_TRIANGLES_KHR) {
            bits.push("CullBackFacingTrianglesKHR")
        }
        if self.contains(spirv::RayFlags::CULL_FRONT_FACING_TRIANGLES_KHR) {
            bits.push("CullFrontFacingTrianglesKHR")
        }
        if self.contains(spirv::RayFlags::CULL_OPAQUE_KHR) {
            bits.push("CullOpaqueKHR")
        }
        if self.contains(spirv::RayFlags::CULL_NO_OPAQUE_KHR) {
            bits.push("CullNoOpaqueKHR")
        }
        if self.contains(spirv::RayFlags::SKIP_TRIANGLES_KHR) {
            bits.push("SkipTrianglesKHR")
        }
        if self.contains(spirv::RayFlags::SKIP_AAB_BS_KHR) {
            bits.push("SkipAABBsKHR")
        }
        if self.contains(spirv::RayFlags::FORCE_OPACITY_MICROMAP2_STATE_EXT) {
            bits.push("ForceOpacityMicromap2StateEXT")
        }
        bits.join("|")
    }
}
impl Disassemble for spirv::FragmentShadingRate {
    fn disassemble(&self) -> String {
        if self.is_empty() {
            return "None".to_string();
        }
        let mut bits = vec![];
        if self.contains(spirv::FragmentShadingRate::VERTICAL2_PIXELS) {
            bits.push("Vertical2Pixels")
        }
        if self.contains(spirv::FragmentShadingRate::VERTICAL4_PIXELS) {
            bits.push("Vertical4Pixels")
        }
        if self.contains(spirv::FragmentShadingRate::HORIZONTAL2_PIXELS) {
            bits.push("Horizontal2Pixels")
        }
        if self.contains(spirv::FragmentShadingRate::HORIZONTAL4_PIXELS) {
            bits.push("Horizontal4Pixels")
        }
        bits.join("|")
    }
}

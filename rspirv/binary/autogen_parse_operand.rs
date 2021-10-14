// AUTOMATICALLY GENERATED from the SPIR-V JSON grammar:
//   external/spirv.core.grammar.json.
// DO NOT MODIFY!

impl<'c, 'd> Parser<'c, 'd> {
    fn parse_operand(&mut self, kind: GOpKind) -> Result<Vec<dr::Operand>> {
        Ok(match kind {
            GOpKind::FPFastMathMode => vec![dr::Operand::FPFastMathMode(
                self.decoder.fp_fast_math_mode()?,
            )],
            GOpKind::SelectionControl => vec![dr::Operand::SelectionControl(
                self.decoder.selection_control()?,
            )],
            GOpKind::FunctionControl => vec![dr::Operand::FunctionControl(
                self.decoder.function_control()?,
            )],
            GOpKind::MemorySemantics => vec![dr::Operand::MemorySemantics(
                self.decoder.memory_semantics()?,
            )],
            GOpKind::KernelProfilingInfo => vec![dr::Operand::KernelProfilingInfo(
                self.decoder.kernel_profiling_info()?,
            )],
            GOpKind::RayFlags => vec![dr::Operand::RayFlags(self.decoder.ray_flags()?)],
            GOpKind::FragmentShadingRate => vec![dr::Operand::FragmentShadingRate(
                self.decoder.fragment_shading_rate()?,
            )],
            GOpKind::SourceLanguage => {
                vec![dr::Operand::SourceLanguage(self.decoder.source_language()?)]
            }
            GOpKind::ExecutionModel => {
                vec![dr::Operand::ExecutionModel(self.decoder.execution_model()?)]
            }
            GOpKind::AddressingModel => vec![dr::Operand::AddressingModel(
                self.decoder.addressing_model()?,
            )],
            GOpKind::MemoryModel => vec![dr::Operand::MemoryModel(self.decoder.memory_model()?)],
            GOpKind::StorageClass => vec![dr::Operand::StorageClass(self.decoder.storage_class()?)],
            GOpKind::Dim => vec![dr::Operand::Dim(self.decoder.dim()?)],
            GOpKind::SamplerAddressingMode => vec![dr::Operand::SamplerAddressingMode(
                self.decoder.sampler_addressing_mode()?,
            )],
            GOpKind::SamplerFilterMode => vec![dr::Operand::SamplerFilterMode(
                self.decoder.sampler_filter_mode()?,
            )],
            GOpKind::ImageFormat => vec![dr::Operand::ImageFormat(self.decoder.image_format()?)],
            GOpKind::ImageChannelOrder => vec![dr::Operand::ImageChannelOrder(
                self.decoder.image_channel_order()?,
            )],
            GOpKind::ImageChannelDataType => vec![dr::Operand::ImageChannelDataType(
                self.decoder.image_channel_data_type()?,
            )],
            GOpKind::FPRoundingMode => vec![dr::Operand::FPRoundingMode(
                self.decoder.fp_rounding_mode()?,
            )],
            GOpKind::LinkageType => vec![dr::Operand::LinkageType(self.decoder.linkage_type()?)],
            GOpKind::AccessQualifier => vec![dr::Operand::AccessQualifier(
                self.decoder.access_qualifier()?,
            )],
            GOpKind::FunctionParameterAttribute => vec![dr::Operand::FunctionParameterAttribute(
                self.decoder.function_parameter_attribute()?,
            )],
            GOpKind::BuiltIn => vec![dr::Operand::BuiltIn(self.decoder.built_in()?)],
            GOpKind::Scope => vec![dr::Operand::Scope(self.decoder.scope()?)],
            GOpKind::GroupOperation => {
                vec![dr::Operand::GroupOperation(self.decoder.group_operation()?)]
            }
            GOpKind::KernelEnqueueFlags => vec![dr::Operand::KernelEnqueueFlags(
                self.decoder.kernel_enqueue_flags()?,
            )],
            GOpKind::Capability => vec![dr::Operand::Capability(self.decoder.capability()?)],
            GOpKind::RayQueryIntersection => vec![dr::Operand::RayQueryIntersection(
                self.decoder.ray_query_intersection()?,
            )],
            GOpKind::RayQueryCommittedIntersectionType => {
                vec![dr::Operand::RayQueryCommittedIntersectionType(
                    self.decoder.ray_query_committed_intersection_type()?,
                )]
            }
            GOpKind::RayQueryCandidateIntersectionType => {
                vec![dr::Operand::RayQueryCandidateIntersectionType(
                    self.decoder.ray_query_candidate_intersection_type()?,
                )]
            }
            GOpKind::IdMemorySemantics => vec![dr::Operand::IdMemorySemantics(self.decoder.id()?)],
            GOpKind::IdScope => vec![dr::Operand::IdScope(self.decoder.id()?)],
            GOpKind::IdRef => vec![dr::Operand::IdRef(self.decoder.id()?)],
            GOpKind::LiteralInteger => vec![dr::Operand::LiteralInt32(self.decoder.int32()?)],
            GOpKind::LiteralString => vec![dr::Operand::LiteralString(self.decoder.string()?)],
            GOpKind::LiteralExtInstInteger => vec![dr::Operand::LiteralExtInstInteger(
                self.decoder.ext_inst_integer()?,
            )],
            GOpKind::PairIdRefLiteralInteger => vec![
                dr::Operand::IdRef(self.decoder.id()?),
                dr::Operand::LiteralInt32(self.decoder.int32()?),
            ],
            GOpKind::PairIdRefIdRef => vec![
                dr::Operand::IdRef(self.decoder.id()?),
                dr::Operand::IdRef(self.decoder.id()?),
            ],
            GOpKind::ImageOperands => {
                let val = self.decoder.image_operands()?;
                let mut ops = vec![dr::Operand::ImageOperands(val)];
                ops.append(&mut self.parse_image_operands_arguments(val)?);
                ops
            }
            GOpKind::LoopControl => {
                let val = self.decoder.loop_control()?;
                let mut ops = vec![dr::Operand::LoopControl(val)];
                ops.append(&mut self.parse_loop_control_arguments(val)?);
                ops
            }
            GOpKind::MemoryAccess => {
                let val = self.decoder.memory_access()?;
                let mut ops = vec![dr::Operand::MemoryAccess(val)];
                ops.append(&mut self.parse_memory_access_arguments(val)?);
                ops
            }
            GOpKind::ExecutionMode => {
                let val = self.decoder.execution_mode()?;
                let mut ops = vec![dr::Operand::ExecutionMode(val)];
                ops.append(&mut self.parse_execution_mode_arguments(val)?);
                ops
            }
            GOpKind::Decoration => {
                let val = self.decoder.decoration()?;
                let mut ops = vec![dr::Operand::Decoration(val)];
                ops.append(&mut self.parse_decoration_arguments(val)?);
                ops
            }
            GOpKind::IdResultType => panic!(),
            GOpKind::IdResult => panic!(),
            GOpKind::LiteralContextDependentNumber => panic!(),
            GOpKind::LiteralSpecConstantOpInteger => panic!(),
            GOpKind::PairLiteralIntegerIdRef => panic!(),
        })
    }
    fn parse_image_operands_arguments(
        &mut self,
        image_operands: spirv::ImageOperands,
    ) -> Result<Vec<dr::Operand>> {
        let mut params = vec![];
        if image_operands.contains(spirv::ImageOperands::BIAS) {
            params.append(&mut vec![dr::Operand::IdRef(self.decoder.id()?)]);
        }
        if image_operands.contains(spirv::ImageOperands::LOD) {
            params.append(&mut vec![dr::Operand::IdRef(self.decoder.id()?)]);
        }
        if image_operands.contains(spirv::ImageOperands::GRAD) {
            params.append(&mut vec![
                dr::Operand::IdRef(self.decoder.id()?),
                dr::Operand::IdRef(self.decoder.id()?),
            ]);
        }
        if image_operands.contains(spirv::ImageOperands::CONST_OFFSET) {
            params.append(&mut vec![dr::Operand::IdRef(self.decoder.id()?)]);
        }
        if image_operands.contains(spirv::ImageOperands::OFFSET) {
            params.append(&mut vec![dr::Operand::IdRef(self.decoder.id()?)]);
        }
        if image_operands.contains(spirv::ImageOperands::CONST_OFFSETS) {
            params.append(&mut vec![dr::Operand::IdRef(self.decoder.id()?)]);
        }
        if image_operands.contains(spirv::ImageOperands::SAMPLE) {
            params.append(&mut vec![dr::Operand::IdRef(self.decoder.id()?)]);
        }
        if image_operands.contains(spirv::ImageOperands::MIN_LOD) {
            params.append(&mut vec![dr::Operand::IdRef(self.decoder.id()?)]);
        }
        if image_operands.contains(spirv::ImageOperands::MAKE_TEXEL_AVAILABLE) {
            params.append(&mut vec![dr::Operand::IdScope(self.decoder.id()?)]);
        }
        if image_operands.contains(spirv::ImageOperands::MAKE_TEXEL_VISIBLE) {
            params.append(&mut vec![dr::Operand::IdScope(self.decoder.id()?)]);
        }
        Ok(params)
    }
    fn parse_loop_control_arguments(
        &mut self,
        loop_control: spirv::LoopControl,
    ) -> Result<Vec<dr::Operand>> {
        let mut params = vec![];
        if loop_control.contains(spirv::LoopControl::DEPENDENCY_LENGTH) {
            params.append(&mut vec![dr::Operand::LiteralInt32(self.decoder.int32()?)]);
        }
        if loop_control.contains(spirv::LoopControl::MIN_ITERATIONS) {
            params.append(&mut vec![dr::Operand::LiteralInt32(self.decoder.int32()?)]);
        }
        if loop_control.contains(spirv::LoopControl::MAX_ITERATIONS) {
            params.append(&mut vec![dr::Operand::LiteralInt32(self.decoder.int32()?)]);
        }
        if loop_control.contains(spirv::LoopControl::ITERATION_MULTIPLE) {
            params.append(&mut vec![dr::Operand::LiteralInt32(self.decoder.int32()?)]);
        }
        if loop_control.contains(spirv::LoopControl::PEEL_COUNT) {
            params.append(&mut vec![dr::Operand::LiteralInt32(self.decoder.int32()?)]);
        }
        if loop_control.contains(spirv::LoopControl::PARTIAL_COUNT) {
            params.append(&mut vec![dr::Operand::LiteralInt32(self.decoder.int32()?)]);
        }
        if loop_control.contains(spirv::LoopControl::INITIATION_INTERVAL_INTEL) {
            params.append(&mut vec![dr::Operand::LiteralInt32(self.decoder.int32()?)]);
        }
        if loop_control.contains(spirv::LoopControl::MAX_CONCURRENCY_INTEL) {
            params.append(&mut vec![dr::Operand::LiteralInt32(self.decoder.int32()?)]);
        }
        if loop_control.contains(spirv::LoopControl::DEPENDENCY_ARRAY_INTEL) {
            params.append(&mut vec![dr::Operand::LiteralInt32(self.decoder.int32()?)]);
        }
        if loop_control.contains(spirv::LoopControl::PIPELINE_ENABLE_INTEL) {
            params.append(&mut vec![dr::Operand::LiteralInt32(self.decoder.int32()?)]);
        }
        if loop_control.contains(spirv::LoopControl::LOOP_COALESCE_INTEL) {
            params.append(&mut vec![dr::Operand::LiteralInt32(self.decoder.int32()?)]);
        }
        if loop_control.contains(spirv::LoopControl::MAX_INTERLEAVING_INTEL) {
            params.append(&mut vec![dr::Operand::LiteralInt32(self.decoder.int32()?)]);
        }
        if loop_control.contains(spirv::LoopControl::SPECULATED_ITERATIONS_INTEL) {
            params.append(&mut vec![dr::Operand::LiteralInt32(self.decoder.int32()?)]);
        }
        Ok(params)
    }
    fn parse_memory_access_arguments(
        &mut self,
        memory_access: spirv::MemoryAccess,
    ) -> Result<Vec<dr::Operand>> {
        let mut params = vec![];
        if memory_access.contains(spirv::MemoryAccess::ALIGNED) {
            params.append(&mut vec![dr::Operand::LiteralInt32(self.decoder.int32()?)]);
        }
        if memory_access.contains(spirv::MemoryAccess::MAKE_POINTER_AVAILABLE) {
            params.append(&mut vec![dr::Operand::IdScope(self.decoder.id()?)]);
        }
        if memory_access.contains(spirv::MemoryAccess::MAKE_POINTER_VISIBLE) {
            params.append(&mut vec![dr::Operand::IdScope(self.decoder.id()?)]);
        }
        Ok(params)
    }
    #[allow(unreachable_patterns)]
    fn parse_execution_mode_arguments(
        &mut self,
        execution_mode: spirv::ExecutionMode,
    ) -> Result<Vec<dr::Operand>> {
        Ok(match execution_mode {
            spirv::ExecutionMode::Invocations => {
                vec![dr::Operand::LiteralInt32(self.decoder.int32()?)]
            }
            spirv::ExecutionMode::LocalSize => vec![
                dr::Operand::LiteralInt32(self.decoder.int32()?),
                dr::Operand::LiteralInt32(self.decoder.int32()?),
                dr::Operand::LiteralInt32(self.decoder.int32()?),
            ],
            spirv::ExecutionMode::LocalSizeHint => vec![
                dr::Operand::LiteralInt32(self.decoder.int32()?),
                dr::Operand::LiteralInt32(self.decoder.int32()?),
                dr::Operand::LiteralInt32(self.decoder.int32()?),
            ],
            spirv::ExecutionMode::OutputVertices => {
                vec![dr::Operand::LiteralInt32(self.decoder.int32()?)]
            }
            spirv::ExecutionMode::VecTypeHint => {
                vec![dr::Operand::LiteralInt32(self.decoder.int32()?)]
            }
            spirv::ExecutionMode::SubgroupSize => {
                vec![dr::Operand::LiteralInt32(self.decoder.int32()?)]
            }
            spirv::ExecutionMode::SubgroupsPerWorkgroup => {
                vec![dr::Operand::LiteralInt32(self.decoder.int32()?)]
            }
            spirv::ExecutionMode::SubgroupsPerWorkgroupId => {
                vec![dr::Operand::IdRef(self.decoder.id()?)]
            }
            spirv::ExecutionMode::LocalSizeId => vec![
                dr::Operand::IdRef(self.decoder.id()?),
                dr::Operand::IdRef(self.decoder.id()?),
                dr::Operand::IdRef(self.decoder.id()?),
            ],
            spirv::ExecutionMode::LocalSizeHintId => vec![dr::Operand::IdRef(self.decoder.id()?)],
            spirv::ExecutionMode::DenormPreserve => {
                vec![dr::Operand::LiteralInt32(self.decoder.int32()?)]
            }
            spirv::ExecutionMode::DenormFlushToZero => {
                vec![dr::Operand::LiteralInt32(self.decoder.int32()?)]
            }
            spirv::ExecutionMode::SignedZeroInfNanPreserve => {
                vec![dr::Operand::LiteralInt32(self.decoder.int32()?)]
            }
            spirv::ExecutionMode::RoundingModeRTE => {
                vec![dr::Operand::LiteralInt32(self.decoder.int32()?)]
            }
            spirv::ExecutionMode::RoundingModeRTZ => {
                vec![dr::Operand::LiteralInt32(self.decoder.int32()?)]
            }
            spirv::ExecutionMode::OutputPrimitivesNV => {
                vec![dr::Operand::LiteralInt32(self.decoder.int32()?)]
            }
            spirv::ExecutionMode::MaxWorkgroupSizeINTEL => vec![
                dr::Operand::LiteralInt32(self.decoder.int32()?),
                dr::Operand::LiteralInt32(self.decoder.int32()?),
                dr::Operand::LiteralInt32(self.decoder.int32()?),
            ],
            spirv::ExecutionMode::MaxWorkDimINTEL => {
                vec![dr::Operand::LiteralInt32(self.decoder.int32()?)]
            }
            spirv::ExecutionMode::NumSIMDWorkitemsINTEL => {
                vec![dr::Operand::LiteralInt32(self.decoder.int32()?)]
            }
            _ => vec![],
        })
    }
    #[allow(unreachable_patterns)]
    fn parse_decoration_arguments(
        &mut self,
        decoration: spirv::Decoration,
    ) -> Result<Vec<dr::Operand>> {
        Ok(match decoration {
            spirv::Decoration::SpecId => vec![dr::Operand::LiteralInt32(self.decoder.int32()?)],
            spirv::Decoration::ArrayStride => {
                vec![dr::Operand::LiteralInt32(self.decoder.int32()?)]
            }
            spirv::Decoration::MatrixStride => {
                vec![dr::Operand::LiteralInt32(self.decoder.int32()?)]
            }
            spirv::Decoration::BuiltIn => vec![dr::Operand::BuiltIn(self.decoder.built_in()?)],
            spirv::Decoration::UniformId => vec![dr::Operand::IdScope(self.decoder.id()?)],
            spirv::Decoration::Stream => vec![dr::Operand::LiteralInt32(self.decoder.int32()?)],
            spirv::Decoration::Location => vec![dr::Operand::LiteralInt32(self.decoder.int32()?)],
            spirv::Decoration::Component => vec![dr::Operand::LiteralInt32(self.decoder.int32()?)],
            spirv::Decoration::Index => vec![dr::Operand::LiteralInt32(self.decoder.int32()?)],
            spirv::Decoration::Binding => vec![dr::Operand::LiteralInt32(self.decoder.int32()?)],
            spirv::Decoration::DescriptorSet => {
                vec![dr::Operand::LiteralInt32(self.decoder.int32()?)]
            }
            spirv::Decoration::Offset => vec![dr::Operand::LiteralInt32(self.decoder.int32()?)],
            spirv::Decoration::XfbBuffer => vec![dr::Operand::LiteralInt32(self.decoder.int32()?)],
            spirv::Decoration::XfbStride => vec![dr::Operand::LiteralInt32(self.decoder.int32()?)],
            spirv::Decoration::FuncParamAttr => vec![dr::Operand::FunctionParameterAttribute(
                self.decoder.function_parameter_attribute()?,
            )],
            spirv::Decoration::FPRoundingMode => vec![dr::Operand::FPRoundingMode(
                self.decoder.fp_rounding_mode()?,
            )],
            spirv::Decoration::FPFastMathMode => vec![dr::Operand::FPFastMathMode(
                self.decoder.fp_fast_math_mode()?,
            )],
            spirv::Decoration::LinkageAttributes => vec![
                dr::Operand::LiteralString(self.decoder.string()?),
                dr::Operand::LinkageType(self.decoder.linkage_type()?),
            ],
            spirv::Decoration::InputAttachmentIndex => {
                vec![dr::Operand::LiteralInt32(self.decoder.int32()?)]
            }
            spirv::Decoration::Alignment => vec![dr::Operand::LiteralInt32(self.decoder.int32()?)],
            spirv::Decoration::MaxByteOffset => {
                vec![dr::Operand::LiteralInt32(self.decoder.int32()?)]
            }
            spirv::Decoration::AlignmentId => vec![dr::Operand::IdRef(self.decoder.id()?)],
            spirv::Decoration::MaxByteOffsetId => vec![dr::Operand::IdRef(self.decoder.id()?)],
            spirv::Decoration::SecondaryViewportRelativeNV => {
                vec![dr::Operand::LiteralInt32(self.decoder.int32()?)]
            }
            spirv::Decoration::CounterBuffer => vec![dr::Operand::IdRef(self.decoder.id()?)],
            spirv::Decoration::UserSemantic => {
                vec![dr::Operand::LiteralString(self.decoder.string()?)]
            }
            spirv::Decoration::UserTypeGOOGLE => {
                vec![dr::Operand::LiteralString(self.decoder.string()?)]
            }
            spirv::Decoration::MemoryINTEL => {
                vec![dr::Operand::LiteralString(self.decoder.string()?)]
            }
            spirv::Decoration::NumbanksINTEL => {
                vec![dr::Operand::LiteralInt32(self.decoder.int32()?)]
            }
            spirv::Decoration::BankwidthINTEL => {
                vec![dr::Operand::LiteralInt32(self.decoder.int32()?)]
            }
            spirv::Decoration::MaxPrivateCopiesINTEL => {
                vec![dr::Operand::LiteralInt32(self.decoder.int32()?)]
            }
            spirv::Decoration::MaxReplicatesINTEL => {
                vec![dr::Operand::LiteralInt32(self.decoder.int32()?)]
            }
            spirv::Decoration::MergeINTEL => vec![
                dr::Operand::LiteralString(self.decoder.string()?),
                dr::Operand::LiteralString(self.decoder.string()?),
            ],
            spirv::Decoration::BankBitsINTEL => {
                vec![dr::Operand::LiteralInt32(self.decoder.int32()?)]
            }
            spirv::Decoration::ForcePow2DepthINTEL => {
                vec![dr::Operand::LiteralInt32(self.decoder.int32()?)]
            }
            _ => vec![],
        })
    }
}

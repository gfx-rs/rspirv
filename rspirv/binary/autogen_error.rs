// AUTOMATICALLY GENERATED from the SPIR-V JSON grammar:
//   external/spirv.core.grammar.json.
// DO NOT MODIFY!

use std::{error, fmt};
#[doc = "Decoder Error"]
#[derive(Debug, PartialEq, Eq)]
#[allow(clippy::upper_case_acronyms)]
pub enum Error {
    StreamExpected(usize),
    LimitReached(usize),
    ImageOperandsUnknown(usize, spirv::Word),
    FPFastMathModeUnknown(usize, spirv::Word),
    SelectionControlUnknown(usize, spirv::Word),
    LoopControlUnknown(usize, spirv::Word),
    FunctionControlUnknown(usize, spirv::Word),
    MemorySemanticsUnknown(usize, spirv::Word),
    MemoryAccessUnknown(usize, spirv::Word),
    KernelProfilingInfoUnknown(usize, spirv::Word),
    RayFlagsUnknown(usize, spirv::Word),
    FragmentShadingRateUnknown(usize, spirv::Word),
    SourceLanguageUnknown(usize, spirv::Word),
    ExecutionModelUnknown(usize, spirv::Word),
    AddressingModelUnknown(usize, spirv::Word),
    MemoryModelUnknown(usize, spirv::Word),
    ExecutionModeUnknown(usize, spirv::Word),
    StorageClassUnknown(usize, spirv::Word),
    DimUnknown(usize, spirv::Word),
    SamplerAddressingModeUnknown(usize, spirv::Word),
    SamplerFilterModeUnknown(usize, spirv::Word),
    ImageFormatUnknown(usize, spirv::Word),
    ImageChannelOrderUnknown(usize, spirv::Word),
    ImageChannelDataTypeUnknown(usize, spirv::Word),
    FPRoundingModeUnknown(usize, spirv::Word),
    FPDenormModeUnknown(usize, spirv::Word),
    QuantizationModesUnknown(usize, spirv::Word),
    FPOperationModeUnknown(usize, spirv::Word),
    OverflowModesUnknown(usize, spirv::Word),
    LinkageTypeUnknown(usize, spirv::Word),
    AccessQualifierUnknown(usize, spirv::Word),
    FunctionParameterAttributeUnknown(usize, spirv::Word),
    DecorationUnknown(usize, spirv::Word),
    BuiltInUnknown(usize, spirv::Word),
    ScopeUnknown(usize, spirv::Word),
    GroupOperationUnknown(usize, spirv::Word),
    KernelEnqueueFlagsUnknown(usize, spirv::Word),
    CapabilityUnknown(usize, spirv::Word),
    RayQueryIntersectionUnknown(usize, spirv::Word),
    RayQueryCommittedIntersectionTypeUnknown(usize, spirv::Word),
    RayQueryCandidateIntersectionTypeUnknown(usize, spirv::Word),
    PackedVectorFormatUnknown(usize, spirv::Word),
    #[doc = r"Failed to decode a string."]
    #[doc = r""]
    #[doc = r"For structured error handling, the second element could be"]
    #[doc = r"`string::FromUtf8Error`, but the will prohibit the compiler"]
    #[doc = r"from generating `PartialEq` for this enum."]
    DecodeStringFailed(usize, String),
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::StreamExpected(index) => {
                write!(f, "expected more bytes in the stream at index {}", index)
            }
            Error::LimitReached(index) => write!(f, "reached word limit at index {}", index),
            Error::ImageOperandsUnknown(index, word) => write!(
                f,
                "unknown value {} for operand kind ImageOperands at index {}",
                word, index
            ),
            Error::FPFastMathModeUnknown(index, word) => write!(
                f,
                "unknown value {} for operand kind FPFastMathMode at index {}",
                word, index
            ),
            Error::SelectionControlUnknown(index, word) => write!(
                f,
                "unknown value {} for operand kind SelectionControl at index {}",
                word, index
            ),
            Error::LoopControlUnknown(index, word) => write!(
                f,
                "unknown value {} for operand kind LoopControl at index {}",
                word, index
            ),
            Error::FunctionControlUnknown(index, word) => write!(
                f,
                "unknown value {} for operand kind FunctionControl at index {}",
                word, index
            ),
            Error::MemorySemanticsUnknown(index, word) => write!(
                f,
                "unknown value {} for operand kind MemorySemantics at index {}",
                word, index
            ),
            Error::MemoryAccessUnknown(index, word) => write!(
                f,
                "unknown value {} for operand kind MemoryAccess at index {}",
                word, index
            ),
            Error::KernelProfilingInfoUnknown(index, word) => write!(
                f,
                "unknown value {} for operand kind KernelProfilingInfo at index {}",
                word, index
            ),
            Error::RayFlagsUnknown(index, word) => write!(
                f,
                "unknown value {} for operand kind RayFlags at index {}",
                word, index
            ),
            Error::FragmentShadingRateUnknown(index, word) => write!(
                f,
                "unknown value {} for operand kind FragmentShadingRate at index {}",
                word, index
            ),
            Error::SourceLanguageUnknown(index, word) => write!(
                f,
                "unknown value {} for operand kind SourceLanguage at index {}",
                word, index
            ),
            Error::ExecutionModelUnknown(index, word) => write!(
                f,
                "unknown value {} for operand kind ExecutionModel at index {}",
                word, index
            ),
            Error::AddressingModelUnknown(index, word) => write!(
                f,
                "unknown value {} for operand kind AddressingModel at index {}",
                word, index
            ),
            Error::MemoryModelUnknown(index, word) => write!(
                f,
                "unknown value {} for operand kind MemoryModel at index {}",
                word, index
            ),
            Error::ExecutionModeUnknown(index, word) => write!(
                f,
                "unknown value {} for operand kind ExecutionMode at index {}",
                word, index
            ),
            Error::StorageClassUnknown(index, word) => write!(
                f,
                "unknown value {} for operand kind StorageClass at index {}",
                word, index
            ),
            Error::DimUnknown(index, word) => write!(
                f,
                "unknown value {} for operand kind Dim at index {}",
                word, index
            ),
            Error::SamplerAddressingModeUnknown(index, word) => write!(
                f,
                "unknown value {} for operand kind SamplerAddressingMode at index {}",
                word, index
            ),
            Error::SamplerFilterModeUnknown(index, word) => write!(
                f,
                "unknown value {} for operand kind SamplerFilterMode at index {}",
                word, index
            ),
            Error::ImageFormatUnknown(index, word) => write!(
                f,
                "unknown value {} for operand kind ImageFormat at index {}",
                word, index
            ),
            Error::ImageChannelOrderUnknown(index, word) => write!(
                f,
                "unknown value {} for operand kind ImageChannelOrder at index {}",
                word, index
            ),
            Error::ImageChannelDataTypeUnknown(index, word) => write!(
                f,
                "unknown value {} for operand kind ImageChannelDataType at index {}",
                word, index
            ),
            Error::FPRoundingModeUnknown(index, word) => write!(
                f,
                "unknown value {} for operand kind FPRoundingMode at index {}",
                word, index
            ),
            Error::FPDenormModeUnknown(index, word) => write!(
                f,
                "unknown value {} for operand kind FPDenormMode at index {}",
                word, index
            ),
            Error::QuantizationModesUnknown(index, word) => write!(
                f,
                "unknown value {} for operand kind QuantizationModes at index {}",
                word, index
            ),
            Error::FPOperationModeUnknown(index, word) => write!(
                f,
                "unknown value {} for operand kind FPOperationMode at index {}",
                word, index
            ),
            Error::OverflowModesUnknown(index, word) => write!(
                f,
                "unknown value {} for operand kind OverflowModes at index {}",
                word, index
            ),
            Error::LinkageTypeUnknown(index, word) => write!(
                f,
                "unknown value {} for operand kind LinkageType at index {}",
                word, index
            ),
            Error::AccessQualifierUnknown(index, word) => write!(
                f,
                "unknown value {} for operand kind AccessQualifier at index {}",
                word, index
            ),
            Error::FunctionParameterAttributeUnknown(index, word) => write!(
                f,
                "unknown value {} for operand kind FunctionParameterAttribute at index {}",
                word, index
            ),
            Error::DecorationUnknown(index, word) => write!(
                f,
                "unknown value {} for operand kind Decoration at index {}",
                word, index
            ),
            Error::BuiltInUnknown(index, word) => write!(
                f,
                "unknown value {} for operand kind BuiltIn at index {}",
                word, index
            ),
            Error::ScopeUnknown(index, word) => write!(
                f,
                "unknown value {} for operand kind Scope at index {}",
                word, index
            ),
            Error::GroupOperationUnknown(index, word) => write!(
                f,
                "unknown value {} for operand kind GroupOperation at index {}",
                word, index
            ),
            Error::KernelEnqueueFlagsUnknown(index, word) => write!(
                f,
                "unknown value {} for operand kind KernelEnqueueFlags at index {}",
                word, index
            ),
            Error::CapabilityUnknown(index, word) => write!(
                f,
                "unknown value {} for operand kind Capability at index {}",
                word, index
            ),
            Error::RayQueryIntersectionUnknown(index, word) => write!(
                f,
                "unknown value {} for operand kind RayQueryIntersection at index {}",
                word, index
            ),
            Error::RayQueryCommittedIntersectionTypeUnknown(index, word) => write!(
                f,
                "unknown value {} for operand kind RayQueryCommittedIntersectionType at index {}",
                word, index
            ),
            Error::RayQueryCandidateIntersectionTypeUnknown(index, word) => write!(
                f,
                "unknown value {} for operand kind RayQueryCandidateIntersectionType at index {}",
                word, index
            ),
            Error::PackedVectorFormatUnknown(index, word) => write!(
                f,
                "unknown value {} for operand kind PackedVectorFormat at index {}",
                word, index
            ),
            Error::DecodeStringFailed(index, ref e) => {
                write!(f, "cannot decode string at index {}: {}", index, e)
            }
        }
    }
}
impl error::Error for Error {}

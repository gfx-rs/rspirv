// AUTOMATICALLY GENERATED from the SPIR-V JSON grammar:
//   external/spirv.core.grammar.json.
// DO NOT MODIFY!

#[doc = "Extended instruction operand kinds."]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[allow(clippy::upper_case_acronyms)]
pub enum ExtOperandKind {
    DebugInfoFlags,
    BuildIdentifierFlags,
    DebugBaseTypeAttributeEncoding,
    DebugCompositeType,
    DebugTypeQualifier,
    DebugOperation,
    DebugImportedEntity,
}
static NONSEMANTIC_SHADER_DEBUGINFO_INSTRUCTIONS: &[ExtendedInstruction<'static>] = &[
    ext_inst!(
        NonsemanticShaderDebuginfo,
        NonsemanticShaderDebuginfoOp,
        DebugInfoNone,
        [],
        [],
        [],
    ),
    ext_inst!(
        NonsemanticShaderDebuginfo,
        NonsemanticShaderDebuginfoOp,
        DebugCompilationUnit,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One)
        ],
    ),
    ext_inst!(
        NonsemanticShaderDebuginfo,
        NonsemanticShaderDebuginfoOp,
        DebugTypeBasic,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, ZeroOrOne)
        ],
    ),
    ext_inst!(
        NonsemanticShaderDebuginfo,
        NonsemanticShaderDebuginfoOp,
        DebugTypePointer,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One)
        ],
    ),
    ext_inst!(
        NonsemanticShaderDebuginfo,
        NonsemanticShaderDebuginfoOp,
        DebugTypeQualifier,
        [],
        [],
        [(OperandKind::IdRef, One), (OperandKind::IdRef, One)],
    ),
    ext_inst!(
        NonsemanticShaderDebuginfo,
        NonsemanticShaderDebuginfoOp,
        DebugTypeArray,
        [],
        [],
        [(OperandKind::IdRef, One), (OperandKind::IdRef, ZeroOrMore)],
    ),
    ext_inst!(
        NonsemanticShaderDebuginfo,
        NonsemanticShaderDebuginfoOp,
        DebugTypeVector,
        [],
        [],
        [(OperandKind::IdRef, One), (OperandKind::IdRef, One)],
    ),
    ext_inst!(
        NonsemanticShaderDebuginfo,
        NonsemanticShaderDebuginfoOp,
        DebugTypedef,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One)
        ],
    ),
    ext_inst!(
        NonsemanticShaderDebuginfo,
        NonsemanticShaderDebuginfoOp,
        DebugTypeFunction,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, ZeroOrMore)
        ],
    ),
    ext_inst!(
        NonsemanticShaderDebuginfo,
        NonsemanticShaderDebuginfoOp,
        DebugTypeEnum,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::PairIdRefIdRef, ZeroOrMore)
        ],
    ),
    ext_inst!(
        NonsemanticShaderDebuginfo,
        NonsemanticShaderDebuginfoOp,
        DebugTypeComposite,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, ZeroOrMore)
        ],
    ),
    ext_inst!(
        NonsemanticShaderDebuginfo,
        NonsemanticShaderDebuginfoOp,
        DebugTypeMember,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, ZeroOrOne)
        ],
    ),
    ext_inst!(
        NonsemanticShaderDebuginfo,
        NonsemanticShaderDebuginfoOp,
        DebugTypeInheritance,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One)
        ],
    ),
    ext_inst!(
        NonsemanticShaderDebuginfo,
        NonsemanticShaderDebuginfoOp,
        DebugTypePtrToMember,
        [],
        [],
        [(OperandKind::IdRef, One), (OperandKind::IdRef, One)],
    ),
    ext_inst!(
        NonsemanticShaderDebuginfo,
        NonsemanticShaderDebuginfoOp,
        DebugTypeTemplate,
        [],
        [],
        [(OperandKind::IdRef, One), (OperandKind::IdRef, ZeroOrMore)],
    ),
    ext_inst!(
        NonsemanticShaderDebuginfo,
        NonsemanticShaderDebuginfoOp,
        DebugTypeTemplateParameter,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One)
        ],
    ),
    ext_inst!(
        NonsemanticShaderDebuginfo,
        NonsemanticShaderDebuginfoOp,
        DebugTypeTemplateTemplateParameter,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One)
        ],
    ),
    ext_inst!(
        NonsemanticShaderDebuginfo,
        NonsemanticShaderDebuginfoOp,
        DebugTypeTemplateParameterPack,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, ZeroOrMore)
        ],
    ),
    ext_inst!(
        NonsemanticShaderDebuginfo,
        NonsemanticShaderDebuginfoOp,
        DebugGlobalVariable,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, ZeroOrOne)
        ],
    ),
    ext_inst!(
        NonsemanticShaderDebuginfo,
        NonsemanticShaderDebuginfoOp,
        DebugFunctionDeclaration,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One)
        ],
    ),
    ext_inst!(
        NonsemanticShaderDebuginfo,
        NonsemanticShaderDebuginfoOp,
        DebugFunction,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, ZeroOrOne)
        ],
    ),
    ext_inst!(
        NonsemanticShaderDebuginfo,
        NonsemanticShaderDebuginfoOp,
        DebugLexicalBlock,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, ZeroOrOne)
        ],
    ),
    ext_inst!(
        NonsemanticShaderDebuginfo,
        NonsemanticShaderDebuginfoOp,
        DebugLexicalBlockDiscriminator,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One)
        ],
    ),
    ext_inst!(
        NonsemanticShaderDebuginfo,
        NonsemanticShaderDebuginfoOp,
        DebugScope,
        [],
        [],
        [(OperandKind::IdRef, One), (OperandKind::IdRef, ZeroOrOne)],
    ),
    ext_inst!(
        NonsemanticShaderDebuginfo,
        NonsemanticShaderDebuginfoOp,
        DebugNoScope,
        [],
        [],
        [],
    ),
    ext_inst!(
        NonsemanticShaderDebuginfo,
        NonsemanticShaderDebuginfoOp,
        DebugInlinedAt,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, ZeroOrOne)
        ],
    ),
    ext_inst!(
        NonsemanticShaderDebuginfo,
        NonsemanticShaderDebuginfoOp,
        DebugLocalVariable,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, ZeroOrOne)
        ],
    ),
    ext_inst!(
        NonsemanticShaderDebuginfo,
        NonsemanticShaderDebuginfoOp,
        DebugInlinedVariable,
        [],
        [],
        [(OperandKind::IdRef, One), (OperandKind::IdRef, One)],
    ),
    ext_inst!(
        NonsemanticShaderDebuginfo,
        NonsemanticShaderDebuginfoOp,
        DebugDeclare,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, ZeroOrMore)
        ],
    ),
    ext_inst!(
        NonsemanticShaderDebuginfo,
        NonsemanticShaderDebuginfoOp,
        DebugValue,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, ZeroOrMore)
        ],
    ),
    ext_inst!(
        NonsemanticShaderDebuginfo,
        NonsemanticShaderDebuginfoOp,
        DebugOperation,
        [],
        [],
        [(OperandKind::IdRef, One), (OperandKind::IdRef, ZeroOrMore)],
    ),
    ext_inst!(
        NonsemanticShaderDebuginfo,
        NonsemanticShaderDebuginfoOp,
        DebugExpression,
        [],
        [],
        [(OperandKind::IdRef, ZeroOrMore)],
    ),
    ext_inst!(
        NonsemanticShaderDebuginfo,
        NonsemanticShaderDebuginfoOp,
        DebugMacroDef,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, ZeroOrOne)
        ],
    ),
    ext_inst!(
        NonsemanticShaderDebuginfo,
        NonsemanticShaderDebuginfoOp,
        DebugMacroUndef,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One)
        ],
    ),
    ext_inst!(
        NonsemanticShaderDebuginfo,
        NonsemanticShaderDebuginfoOp,
        DebugImportedEntity,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One)
        ],
    ),
    ext_inst!(
        NonsemanticShaderDebuginfo,
        NonsemanticShaderDebuginfoOp,
        DebugSource,
        [],
        [],
        [(OperandKind::IdRef, One), (OperandKind::IdRef, ZeroOrOne)],
    ),
    ext_inst!(
        NonsemanticShaderDebuginfo,
        NonsemanticShaderDebuginfoOp,
        DebugFunctionDefinition,
        [],
        [],
        [(OperandKind::IdRef, One), (OperandKind::IdRef, One)],
    ),
    ext_inst!(
        NonsemanticShaderDebuginfo,
        NonsemanticShaderDebuginfoOp,
        DebugSourceContinued,
        [],
        [],
        [(OperandKind::IdRef, One)],
    ),
    ext_inst!(
        NonsemanticShaderDebuginfo,
        NonsemanticShaderDebuginfoOp,
        DebugLine,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One)
        ],
    ),
    ext_inst!(
        NonsemanticShaderDebuginfo,
        NonsemanticShaderDebuginfoOp,
        DebugNoLine,
        [],
        [],
        [],
    ),
    ext_inst!(
        NonsemanticShaderDebuginfo,
        NonsemanticShaderDebuginfoOp,
        DebugBuildIdentifier,
        [],
        [],
        [(OperandKind::IdRef, One), (OperandKind::IdRef, One)],
    ),
    ext_inst!(
        NonsemanticShaderDebuginfo,
        NonsemanticShaderDebuginfoOp,
        DebugStoragePath,
        [],
        [],
        [(OperandKind::IdRef, One)],
    ),
    ext_inst!(
        NonsemanticShaderDebuginfo,
        NonsemanticShaderDebuginfoOp,
        DebugEntryPoint,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One)
        ],
    ),
    ext_inst!(
        NonsemanticShaderDebuginfo,
        NonsemanticShaderDebuginfoOp,
        DebugTypeMatrix,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One)
        ],
    ),
    ext_inst!(
        NonsemanticShaderDebuginfo,
        NonsemanticShaderDebuginfoOp,
        DebugTypeVectorIdEXT,
        [],
        [],
        [(OperandKind::IdRef, One), (OperandKind::IdRef, One)],
    ),
    ext_inst!(
        NonsemanticShaderDebuginfo,
        NonsemanticShaderDebuginfoOp,
        DebugTypeCooperativeMatrixKHR,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One)
        ],
    ),
];
pub static NONSEMANTIC_SHADER_DEBUGINFO_INSTRUCTION_TABLE: InstructionTable<ExtInstOp> =
    InstructionTable(
        NONSEMANTIC_SHADER_DEBUGINFO_INSTRUCTIONS,
        std::marker::PhantomData,
    );

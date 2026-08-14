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
static NONSEMANTIC_SHADER_DEBUGINFO_100_INSTRUCTIONS: &[ExtendedInstruction<'static>] = &[
    ext_inst!(
        NonsemanticShaderDebuginfo100,
        NonsemanticShaderDebuginfo100Op,
        DebugInfoNone,
        [],
        [],
        [],
    ),
    ext_inst!(
        NonsemanticShaderDebuginfo100,
        NonsemanticShaderDebuginfo100Op,
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
        NonsemanticShaderDebuginfo100,
        NonsemanticShaderDebuginfo100Op,
        DebugTypeBasic,
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
        NonsemanticShaderDebuginfo100,
        NonsemanticShaderDebuginfo100Op,
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
        NonsemanticShaderDebuginfo100,
        NonsemanticShaderDebuginfo100Op,
        DebugTypeQualifier,
        [],
        [],
        [(OperandKind::IdRef, One), (OperandKind::IdRef, One)],
    ),
    ext_inst!(
        NonsemanticShaderDebuginfo100,
        NonsemanticShaderDebuginfo100Op,
        DebugTypeArray,
        [],
        [],
        [(OperandKind::IdRef, One), (OperandKind::IdRef, ZeroOrMore)],
    ),
    ext_inst!(
        NonsemanticShaderDebuginfo100,
        NonsemanticShaderDebuginfo100Op,
        DebugTypeVector,
        [],
        [],
        [(OperandKind::IdRef, One), (OperandKind::IdRef, One)],
    ),
    ext_inst!(
        NonsemanticShaderDebuginfo100,
        NonsemanticShaderDebuginfo100Op,
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
        NonsemanticShaderDebuginfo100,
        NonsemanticShaderDebuginfo100Op,
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
        NonsemanticShaderDebuginfo100,
        NonsemanticShaderDebuginfo100Op,
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
        NonsemanticShaderDebuginfo100,
        NonsemanticShaderDebuginfo100Op,
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
        NonsemanticShaderDebuginfo100,
        NonsemanticShaderDebuginfo100Op,
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
        NonsemanticShaderDebuginfo100,
        NonsemanticShaderDebuginfo100Op,
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
        NonsemanticShaderDebuginfo100,
        NonsemanticShaderDebuginfo100Op,
        DebugTypePtrToMember,
        [],
        [],
        [(OperandKind::IdRef, One), (OperandKind::IdRef, One)],
    ),
    ext_inst!(
        NonsemanticShaderDebuginfo100,
        NonsemanticShaderDebuginfo100Op,
        DebugTypeTemplate,
        [],
        [],
        [(OperandKind::IdRef, One), (OperandKind::IdRef, ZeroOrMore)],
    ),
    ext_inst!(
        NonsemanticShaderDebuginfo100,
        NonsemanticShaderDebuginfo100Op,
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
        NonsemanticShaderDebuginfo100,
        NonsemanticShaderDebuginfo100Op,
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
        NonsemanticShaderDebuginfo100,
        NonsemanticShaderDebuginfo100Op,
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
        NonsemanticShaderDebuginfo100,
        NonsemanticShaderDebuginfo100Op,
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
        NonsemanticShaderDebuginfo100,
        NonsemanticShaderDebuginfo100Op,
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
        NonsemanticShaderDebuginfo100,
        NonsemanticShaderDebuginfo100Op,
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
        NonsemanticShaderDebuginfo100,
        NonsemanticShaderDebuginfo100Op,
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
        NonsemanticShaderDebuginfo100,
        NonsemanticShaderDebuginfo100Op,
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
        NonsemanticShaderDebuginfo100,
        NonsemanticShaderDebuginfo100Op,
        DebugScope,
        [],
        [],
        [(OperandKind::IdRef, One), (OperandKind::IdRef, ZeroOrOne)],
    ),
    ext_inst!(
        NonsemanticShaderDebuginfo100,
        NonsemanticShaderDebuginfo100Op,
        DebugNoScope,
        [],
        [],
        [],
    ),
    ext_inst!(
        NonsemanticShaderDebuginfo100,
        NonsemanticShaderDebuginfo100Op,
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
        NonsemanticShaderDebuginfo100,
        NonsemanticShaderDebuginfo100Op,
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
        NonsemanticShaderDebuginfo100,
        NonsemanticShaderDebuginfo100Op,
        DebugInlinedVariable,
        [],
        [],
        [(OperandKind::IdRef, One), (OperandKind::IdRef, One)],
    ),
    ext_inst!(
        NonsemanticShaderDebuginfo100,
        NonsemanticShaderDebuginfo100Op,
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
        NonsemanticShaderDebuginfo100,
        NonsemanticShaderDebuginfo100Op,
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
        NonsemanticShaderDebuginfo100,
        NonsemanticShaderDebuginfo100Op,
        DebugOperation,
        [],
        [],
        [(OperandKind::IdRef, One), (OperandKind::IdRef, ZeroOrMore)],
    ),
    ext_inst!(
        NonsemanticShaderDebuginfo100,
        NonsemanticShaderDebuginfo100Op,
        DebugExpression,
        [],
        [],
        [(OperandKind::IdRef, ZeroOrMore)],
    ),
    ext_inst!(
        NonsemanticShaderDebuginfo100,
        NonsemanticShaderDebuginfo100Op,
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
        NonsemanticShaderDebuginfo100,
        NonsemanticShaderDebuginfo100Op,
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
        NonsemanticShaderDebuginfo100,
        NonsemanticShaderDebuginfo100Op,
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
        NonsemanticShaderDebuginfo100,
        NonsemanticShaderDebuginfo100Op,
        DebugSource,
        [],
        [],
        [(OperandKind::IdRef, One), (OperandKind::IdRef, ZeroOrOne)],
    ),
    ext_inst!(
        NonsemanticShaderDebuginfo100,
        NonsemanticShaderDebuginfo100Op,
        DebugFunctionDefinition,
        [],
        [],
        [(OperandKind::IdRef, One), (OperandKind::IdRef, One)],
    ),
    ext_inst!(
        NonsemanticShaderDebuginfo100,
        NonsemanticShaderDebuginfo100Op,
        DebugSourceContinued,
        [],
        [],
        [(OperandKind::IdRef, One)],
    ),
    ext_inst!(
        NonsemanticShaderDebuginfo100,
        NonsemanticShaderDebuginfo100Op,
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
        NonsemanticShaderDebuginfo100,
        NonsemanticShaderDebuginfo100Op,
        DebugNoLine,
        [],
        [],
        [],
    ),
    ext_inst!(
        NonsemanticShaderDebuginfo100,
        NonsemanticShaderDebuginfo100Op,
        DebugBuildIdentifier,
        [],
        [],
        [(OperandKind::IdRef, One), (OperandKind::IdRef, One)],
    ),
    ext_inst!(
        NonsemanticShaderDebuginfo100,
        NonsemanticShaderDebuginfo100Op,
        DebugStoragePath,
        [],
        [],
        [(OperandKind::IdRef, One)],
    ),
    ext_inst!(
        NonsemanticShaderDebuginfo100,
        NonsemanticShaderDebuginfo100Op,
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
        NonsemanticShaderDebuginfo100,
        NonsemanticShaderDebuginfo100Op,
        DebugTypeMatrix,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One)
        ],
    ),
];
pub static NONSEMANTIC_SHADER_DEBUGINFO_100_INSTRUCTION_TABLE: InstructionTable<ExtInstOp> =
    InstructionTable(
        NONSEMANTIC_SHADER_DEBUGINFO_100_INSTRUCTIONS,
        std::marker::PhantomData,
    );

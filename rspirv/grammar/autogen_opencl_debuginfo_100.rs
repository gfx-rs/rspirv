// AUTOMATICALLY GENERATED from the SPIR-V JSON grammar:
//   external/spirv.core.grammar.json.
// DO NOT MODIFY!

#[doc = "Extended instruction operand kinds."]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[allow(clippy::upper_case_acronyms)]
pub enum ExtOperandKind {
    DebugInfoFlags,
    DebugBaseTypeAttributeEncoding,
    DebugCompositeType,
    DebugTypeQualifier,
    DebugOperation,
    DebugImportedEntity,
}
static OPENCL_DEBUGINFO_100_INSTRUCTIONS: &[ExtendedInstruction<'static>] = &[
    ext_inst!(
        OpenclDebuginfo100,
        OpenclDebuginfo100Op,
        DebugInfoNone,
        [],
        [],
        [],
    ),
    ext_inst!(
        OpenclDebuginfo100,
        OpenclDebuginfo100Op,
        DebugCompilationUnit,
        [],
        [],
        [
            (OperandKind::LiteralInteger, One),
            (OperandKind::LiteralInteger, One),
            (OperandKind::IdRef, One),
            (OperandKind::SourceLanguage, One)
        ],
    ),
    ext_inst!(
        OpenclDebuginfo100,
        OpenclDebuginfo100Op,
        DebugTypeBasic,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (
                OperandKind::OpenclDebuginfo100(ExtOperandKind::DebugBaseTypeAttributeEncoding),
                One
            )
        ],
    ),
    ext_inst!(
        OpenclDebuginfo100,
        OpenclDebuginfo100Op,
        DebugTypePointer,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::StorageClass, One),
            (
                OperandKind::OpenclDebuginfo100(ExtOperandKind::DebugInfoFlags),
                One
            )
        ],
    ),
    ext_inst!(
        OpenclDebuginfo100,
        OpenclDebuginfo100Op,
        DebugTypeQualifier,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (
                OperandKind::OpenclDebuginfo100(ExtOperandKind::DebugTypeQualifier),
                One
            )
        ],
    ),
    ext_inst!(
        OpenclDebuginfo100,
        OpenclDebuginfo100Op,
        DebugTypeArray,
        [],
        [],
        [(OperandKind::IdRef, One), (OperandKind::IdRef, ZeroOrMore)],
    ),
    ext_inst!(
        OpenclDebuginfo100,
        OpenclDebuginfo100Op,
        DebugTypeVector,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::LiteralInteger, One)
        ],
    ),
    ext_inst!(
        OpenclDebuginfo100,
        OpenclDebuginfo100Op,
        DebugTypedef,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::LiteralInteger, One),
            (OperandKind::LiteralInteger, One),
            (OperandKind::IdRef, One)
        ],
    ),
    ext_inst!(
        OpenclDebuginfo100,
        OpenclDebuginfo100Op,
        DebugTypeFunction,
        [],
        [],
        [
            (
                OperandKind::OpenclDebuginfo100(ExtOperandKind::DebugInfoFlags),
                One
            ),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, ZeroOrMore)
        ],
    ),
    ext_inst!(
        OpenclDebuginfo100,
        OpenclDebuginfo100Op,
        DebugTypeEnum,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::LiteralInteger, One),
            (OperandKind::LiteralInteger, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (
                OperandKind::OpenclDebuginfo100(ExtOperandKind::DebugInfoFlags),
                One
            ),
            (OperandKind::PairIdRefIdRef, ZeroOrMore)
        ],
    ),
    ext_inst!(
        OpenclDebuginfo100,
        OpenclDebuginfo100Op,
        DebugTypeComposite,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (
                OperandKind::OpenclDebuginfo100(ExtOperandKind::DebugCompositeType),
                One
            ),
            (OperandKind::IdRef, One),
            (OperandKind::LiteralInteger, One),
            (OperandKind::LiteralInteger, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (
                OperandKind::OpenclDebuginfo100(ExtOperandKind::DebugInfoFlags),
                One
            ),
            (OperandKind::IdRef, ZeroOrMore)
        ],
    ),
    ext_inst!(
        OpenclDebuginfo100,
        OpenclDebuginfo100Op,
        DebugTypeMember,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::LiteralInteger, One),
            (OperandKind::LiteralInteger, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (
                OperandKind::OpenclDebuginfo100(ExtOperandKind::DebugInfoFlags),
                One
            ),
            (OperandKind::IdRef, ZeroOrOne)
        ],
    ),
    ext_inst!(
        OpenclDebuginfo100,
        OpenclDebuginfo100Op,
        DebugTypeInheritance,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (
                OperandKind::OpenclDebuginfo100(ExtOperandKind::DebugInfoFlags),
                One
            )
        ],
    ),
    ext_inst!(
        OpenclDebuginfo100,
        OpenclDebuginfo100Op,
        DebugTypePtrToMember,
        [],
        [],
        [(OperandKind::IdRef, One), (OperandKind::IdRef, One)],
    ),
    ext_inst!(
        OpenclDebuginfo100,
        OpenclDebuginfo100Op,
        DebugTypeTemplate,
        [],
        [],
        [(OperandKind::IdRef, One), (OperandKind::IdRef, ZeroOrMore)],
    ),
    ext_inst!(
        OpenclDebuginfo100,
        OpenclDebuginfo100Op,
        DebugTypeTemplateParameter,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::LiteralInteger, One),
            (OperandKind::LiteralInteger, One)
        ],
    ),
    ext_inst!(
        OpenclDebuginfo100,
        OpenclDebuginfo100Op,
        DebugTypeTemplateTemplateParameter,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::LiteralInteger, One),
            (OperandKind::LiteralInteger, One)
        ],
    ),
    ext_inst!(
        OpenclDebuginfo100,
        OpenclDebuginfo100Op,
        DebugTypeTemplateParameterPack,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::LiteralInteger, One),
            (OperandKind::LiteralInteger, One),
            (OperandKind::IdRef, ZeroOrMore)
        ],
    ),
    ext_inst!(
        OpenclDebuginfo100,
        OpenclDebuginfo100Op,
        DebugGlobalVariable,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::LiteralInteger, One),
            (OperandKind::LiteralInteger, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (
                OperandKind::OpenclDebuginfo100(ExtOperandKind::DebugInfoFlags),
                One
            ),
            (OperandKind::IdRef, ZeroOrOne)
        ],
    ),
    ext_inst!(
        OpenclDebuginfo100,
        OpenclDebuginfo100Op,
        DebugFunctionDeclaration,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::LiteralInteger, One),
            (OperandKind::LiteralInteger, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (
                OperandKind::OpenclDebuginfo100(ExtOperandKind::DebugInfoFlags),
                One
            )
        ],
    ),
    ext_inst!(
        OpenclDebuginfo100,
        OpenclDebuginfo100Op,
        DebugFunction,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::LiteralInteger, One),
            (OperandKind::LiteralInteger, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (
                OperandKind::OpenclDebuginfo100(ExtOperandKind::DebugInfoFlags),
                One
            ),
            (OperandKind::LiteralInteger, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, ZeroOrOne)
        ],
    ),
    ext_inst!(
        OpenclDebuginfo100,
        OpenclDebuginfo100Op,
        DebugLexicalBlock,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::LiteralInteger, One),
            (OperandKind::LiteralInteger, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, ZeroOrOne)
        ],
    ),
    ext_inst!(
        OpenclDebuginfo100,
        OpenclDebuginfo100Op,
        DebugLexicalBlockDiscriminator,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::LiteralInteger, One),
            (OperandKind::IdRef, One)
        ],
    ),
    ext_inst!(
        OpenclDebuginfo100,
        OpenclDebuginfo100Op,
        DebugScope,
        [],
        [],
        [(OperandKind::IdRef, One), (OperandKind::IdRef, ZeroOrOne)],
    ),
    ext_inst!(
        OpenclDebuginfo100,
        OpenclDebuginfo100Op,
        DebugNoScope,
        [],
        [],
        [],
    ),
    ext_inst!(
        OpenclDebuginfo100,
        OpenclDebuginfo100Op,
        DebugInlinedAt,
        [],
        [],
        [
            (OperandKind::LiteralInteger, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, ZeroOrOne)
        ],
    ),
    ext_inst!(
        OpenclDebuginfo100,
        OpenclDebuginfo100Op,
        DebugLocalVariable,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::LiteralInteger, One),
            (OperandKind::LiteralInteger, One),
            (OperandKind::IdRef, One),
            (
                OperandKind::OpenclDebuginfo100(ExtOperandKind::DebugInfoFlags),
                One
            ),
            (OperandKind::LiteralInteger, ZeroOrOne)
        ],
    ),
    ext_inst!(
        OpenclDebuginfo100,
        OpenclDebuginfo100Op,
        DebugInlinedVariable,
        [],
        [],
        [(OperandKind::IdRef, One), (OperandKind::IdRef, One)],
    ),
    ext_inst!(
        OpenclDebuginfo100,
        OpenclDebuginfo100Op,
        DebugDeclare,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One)
        ],
    ),
    ext_inst!(
        OpenclDebuginfo100,
        OpenclDebuginfo100Op,
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
        OpenclDebuginfo100,
        OpenclDebuginfo100Op,
        DebugOperation,
        [],
        [],
        [
            (
                OperandKind::OpenclDebuginfo100(ExtOperandKind::DebugOperation),
                One
            ),
            (OperandKind::LiteralInteger, ZeroOrMore)
        ],
    ),
    ext_inst!(
        OpenclDebuginfo100,
        OpenclDebuginfo100Op,
        DebugExpression,
        [],
        [],
        [(OperandKind::IdRef, ZeroOrMore)],
    ),
    ext_inst!(
        OpenclDebuginfo100,
        OpenclDebuginfo100Op,
        DebugMacroDef,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::LiteralInteger, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, ZeroOrOne)
        ],
    ),
    ext_inst!(
        OpenclDebuginfo100,
        OpenclDebuginfo100Op,
        DebugMacroUndef,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::LiteralInteger, One),
            (OperandKind::IdRef, One)
        ],
    ),
    ext_inst!(
        OpenclDebuginfo100,
        OpenclDebuginfo100Op,
        DebugImportedEntity,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (
                OperandKind::OpenclDebuginfo100(ExtOperandKind::DebugImportedEntity),
                One
            ),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::LiteralInteger, One),
            (OperandKind::LiteralInteger, One),
            (OperandKind::IdRef, One)
        ],
    ),
    ext_inst!(
        OpenclDebuginfo100,
        OpenclDebuginfo100Op,
        DebugSource,
        [],
        [],
        [(OperandKind::IdRef, One), (OperandKind::IdRef, ZeroOrOne)],
    ),
    ext_inst!(
        OpenclDebuginfo100,
        OpenclDebuginfo100Op,
        DebugModuleINTEL,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::LiteralInteger, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::LiteralInteger, One)
        ],
    ),
];
pub static OPENCL_DEBUGINFO_100_INSTRUCTION_TABLE: InstructionTable<ExtInstOp> =
    InstructionTable(OPENCL_DEBUGINFO_100_INSTRUCTIONS, std::marker::PhantomData);

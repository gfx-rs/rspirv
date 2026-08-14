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
}
static DEBUGINFO_INSTRUCTIONS: &[ExtendedInstruction<'static>] = &[
    ext_inst!(Debuginfo, DebuginfoOp, DebugInfoNone, [], [], [],),
    ext_inst!(
        Debuginfo,
        DebuginfoOp,
        DebugCompilationUnit,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::LiteralInteger, One),
            (OperandKind::LiteralInteger, One)
        ],
    ),
    ext_inst!(
        Debuginfo,
        DebuginfoOp,
        DebugTypeBasic,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (
                OperandKind::Debuginfo(ExtOperandKind::DebugBaseTypeAttributeEncoding),
                One
            )
        ],
    ),
    ext_inst!(
        Debuginfo,
        DebuginfoOp,
        DebugTypePointer,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::StorageClass, One),
            (OperandKind::Debuginfo(ExtOperandKind::DebugInfoFlags), One)
        ],
    ),
    ext_inst!(
        Debuginfo,
        DebuginfoOp,
        DebugTypeQualifier,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (
                OperandKind::Debuginfo(ExtOperandKind::DebugTypeQualifier),
                One
            )
        ],
    ),
    ext_inst!(
        Debuginfo,
        DebuginfoOp,
        DebugTypeArray,
        [],
        [],
        [(OperandKind::IdRef, One), (OperandKind::IdRef, ZeroOrMore)],
    ),
    ext_inst!(
        Debuginfo,
        DebuginfoOp,
        DebugTypeVector,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::LiteralInteger, One)
        ],
    ),
    ext_inst!(
        Debuginfo,
        DebuginfoOp,
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
        Debuginfo,
        DebuginfoOp,
        DebugTypeFunction,
        [],
        [],
        [(OperandKind::IdRef, One), (OperandKind::IdRef, ZeroOrMore)],
    ),
    ext_inst!(
        Debuginfo,
        DebuginfoOp,
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
            (OperandKind::Debuginfo(ExtOperandKind::DebugInfoFlags), One),
            (OperandKind::PairIdRefIdRef, ZeroOrMore)
        ],
    ),
    ext_inst!(
        Debuginfo,
        DebuginfoOp,
        DebugTypeComposite,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (
                OperandKind::Debuginfo(ExtOperandKind::DebugCompositeType),
                One
            ),
            (OperandKind::IdRef, One),
            (OperandKind::LiteralInteger, One),
            (OperandKind::LiteralInteger, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::Debuginfo(ExtOperandKind::DebugInfoFlags), One),
            (OperandKind::IdRef, ZeroOrMore)
        ],
    ),
    ext_inst!(
        Debuginfo,
        DebuginfoOp,
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
            (OperandKind::Debuginfo(ExtOperandKind::DebugInfoFlags), One),
            (OperandKind::IdRef, ZeroOrOne)
        ],
    ),
    ext_inst!(
        Debuginfo,
        DebuginfoOp,
        DebugTypeInheritance,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::Debuginfo(ExtOperandKind::DebugInfoFlags), One)
        ],
    ),
    ext_inst!(
        Debuginfo,
        DebuginfoOp,
        DebugTypePtrToMember,
        [],
        [],
        [(OperandKind::IdRef, One), (OperandKind::IdRef, One)],
    ),
    ext_inst!(
        Debuginfo,
        DebuginfoOp,
        DebugTypeTemplate,
        [],
        [],
        [(OperandKind::IdRef, One), (OperandKind::IdRef, ZeroOrMore)],
    ),
    ext_inst!(
        Debuginfo,
        DebuginfoOp,
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
        Debuginfo,
        DebuginfoOp,
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
        Debuginfo,
        DebuginfoOp,
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
        Debuginfo,
        DebuginfoOp,
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
            (OperandKind::Debuginfo(ExtOperandKind::DebugInfoFlags), One),
            (OperandKind::IdRef, ZeroOrOne)
        ],
    ),
    ext_inst!(
        Debuginfo,
        DebuginfoOp,
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
            (OperandKind::Debuginfo(ExtOperandKind::DebugInfoFlags), One)
        ],
    ),
    ext_inst!(
        Debuginfo,
        DebuginfoOp,
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
            (OperandKind::Debuginfo(ExtOperandKind::DebugInfoFlags), One),
            (OperandKind::LiteralInteger, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, ZeroOrOne)
        ],
    ),
    ext_inst!(
        Debuginfo,
        DebuginfoOp,
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
        Debuginfo,
        DebuginfoOp,
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
        Debuginfo,
        DebuginfoOp,
        DebugScope,
        [],
        [],
        [(OperandKind::IdRef, One), (OperandKind::IdRef, ZeroOrOne)],
    ),
    ext_inst!(Debuginfo, DebuginfoOp, DebugNoScope, [], [], [],),
    ext_inst!(
        Debuginfo,
        DebuginfoOp,
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
        Debuginfo,
        DebuginfoOp,
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
            (OperandKind::LiteralInteger, ZeroOrOne)
        ],
    ),
    ext_inst!(
        Debuginfo,
        DebuginfoOp,
        DebugInlinedVariable,
        [],
        [],
        [(OperandKind::IdRef, One), (OperandKind::IdRef, One)],
    ),
    ext_inst!(
        Debuginfo,
        DebuginfoOp,
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
        Debuginfo,
        DebuginfoOp,
        DebugValue,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, One),
            (OperandKind::IdRef, ZeroOrMore)
        ],
    ),
    ext_inst!(
        Debuginfo,
        DebuginfoOp,
        DebugOperation,
        [],
        [],
        [
            (OperandKind::Debuginfo(ExtOperandKind::DebugOperation), One),
            (OperandKind::LiteralInteger, ZeroOrMore)
        ],
    ),
    ext_inst!(
        Debuginfo,
        DebuginfoOp,
        DebugExpression,
        [],
        [],
        [(OperandKind::IdRef, ZeroOrMore)],
    ),
    ext_inst!(
        Debuginfo,
        DebuginfoOp,
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
        Debuginfo,
        DebuginfoOp,
        DebugMacroUndef,
        [],
        [],
        [
            (OperandKind::IdRef, One),
            (OperandKind::LiteralInteger, One),
            (OperandKind::IdRef, One)
        ],
    ),
];
pub static DEBUGINFO_INSTRUCTION_TABLE: InstructionTable<ExtInstOp> =
    InstructionTable(DEBUGINFO_INSTRUCTIONS, std::marker::PhantomData);

// AUTOMATICALLY GENERATED from the SPIR-V JSON grammar:
//   external/spirv.core.grammar.json.
// DO NOT MODIFY!

static NONSEMANTIC_SHADER_DEBUGINFO_100_INSTRUCTION_TABLE: &[ExtendedInstruction<'static>] = &[
    ext_inst!(DebugInfoNone, 0u32, [], [], []),
    ext_inst!(
        DebugCompilationUnit,
        1u32,
        [],
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        DebugTypeBasic,
        2u32,
        [],
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        DebugTypePointer,
        3u32,
        [],
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        DebugTypeQualifier,
        4u32,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        DebugTypeArray,
        5u32,
        [],
        [],
        [(IdRef, One), (IdRef, ZeroOrMore)]
    ),
    ext_inst!(DebugTypeVector, 6u32, [], [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(
        DebugTypedef,
        7u32,
        [],
        [],
        [
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    ext_inst!(
        DebugTypeFunction,
        8u32,
        [],
        [],
        [(IdRef, One), (IdRef, One), (IdRef, ZeroOrMore)]
    ),
    ext_inst!(
        DebugTypeEnum,
        9u32,
        [],
        [],
        [
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (PairIdRefIdRef, ZeroOrMore)
        ]
    ),
    ext_inst!(
        DebugTypeComposite,
        10u32,
        [],
        [],
        [
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, ZeroOrMore)
        ]
    ),
    ext_inst!(
        DebugTypeMember,
        11u32,
        [],
        [],
        [
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, ZeroOrOne)
        ]
    ),
    ext_inst!(
        DebugTypeInheritance,
        12u32,
        [],
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        DebugTypePtrToMember,
        13u32,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        DebugTypeTemplate,
        14u32,
        [],
        [],
        [(IdRef, One), (IdRef, ZeroOrMore)]
    ),
    ext_inst!(
        DebugTypeTemplateParameter,
        15u32,
        [],
        [],
        [
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    ext_inst!(
        DebugTypeTemplateTemplateParameter,
        16u32,
        [],
        [],
        [
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    ext_inst!(
        DebugTypeTemplateParameterPack,
        17u32,
        [],
        [],
        [
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, ZeroOrMore)
        ]
    ),
    ext_inst!(
        DebugGlobalVariable,
        18u32,
        [],
        [],
        [
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, ZeroOrOne)
        ]
    ),
    ext_inst!(
        DebugFunctionDeclaration,
        19u32,
        [],
        [],
        [
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    ext_inst!(
        DebugFunction,
        20u32,
        [],
        [],
        [
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, ZeroOrOne)
        ]
    ),
    ext_inst!(
        DebugLexicalBlock,
        21u32,
        [],
        [],
        [
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, ZeroOrOne)
        ]
    ),
    ext_inst!(
        DebugLexicalBlockDiscriminator,
        22u32,
        [],
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        DebugScope,
        23u32,
        [],
        [],
        [(IdRef, One), (IdRef, ZeroOrOne)]
    ),
    ext_inst!(DebugNoScope, 24u32, [], [], []),
    ext_inst!(
        DebugInlinedAt,
        25u32,
        [],
        [],
        [(IdRef, One), (IdRef, One), (IdRef, ZeroOrOne)]
    ),
    ext_inst!(
        DebugLocalVariable,
        26u32,
        [],
        [],
        [
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, ZeroOrOne)
        ]
    ),
    ext_inst!(
        DebugInlinedVariable,
        27u32,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        DebugDeclare,
        28u32,
        [],
        [],
        [
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, ZeroOrMore)
        ]
    ),
    ext_inst!(
        DebugValue,
        29u32,
        [],
        [],
        [
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, ZeroOrMore)
        ]
    ),
    ext_inst!(
        DebugOperation,
        30u32,
        [],
        [],
        [(IdRef, One), (IdRef, ZeroOrMore)]
    ),
    ext_inst!(DebugExpression, 31u32, [], [], [(IdRef, ZeroOrMore)]),
    ext_inst!(
        DebugMacroDef,
        32u32,
        [],
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One), (IdRef, ZeroOrOne)]
    ),
    ext_inst!(
        DebugMacroUndef,
        33u32,
        [],
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        DebugImportedEntity,
        34u32,
        [],
        [],
        [
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    ext_inst!(
        DebugSource,
        35u32,
        [],
        [],
        [(IdRef, One), (IdRef, ZeroOrOne)]
    ),
    ext_inst!(
        DebugFunctionDefinition,
        101u32,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(DebugSourceContinued, 102u32, [], [], [(IdRef, One)]),
    ext_inst!(
        DebugLine,
        103u32,
        [],
        [],
        [
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (IdRef, One)
        ]
    ),
    ext_inst!(DebugNoLine, 104u32, [], [], []),
    ext_inst!(
        DebugBuildIdentifier,
        105u32,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(DebugStoragePath, 106u32, [], [], [(IdRef, One)]),
    ext_inst!(
        DebugEntryPoint,
        107u32,
        [],
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        DebugTypeMatrix,
        108u32,
        [],
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
];

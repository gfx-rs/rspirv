// AUTOMATICALLY GENERATED from the SPIR-V JSON grammar:
//   external/spirv.core.grammar.json.
// DO NOT MODIFY!

static TOSA_001000_1_INSTRUCTIONS: &[ExtendedInstruction<'static>] = &[
    ext_inst!(
        Tosa0010001,
        Tosa0010001Op,
        ARGMAX,
        [],
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        Tosa0010001,
        Tosa0010001Op,
        AVG_POOL2D,
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
        Tosa0010001,
        Tosa0010001Op,
        CONV2D,
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
            (IdRef, One)
        ]
    ),
    ext_inst!(
        Tosa0010001,
        Tosa0010001Op,
        CONV3D,
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
            (IdRef, One)
        ]
    ),
    ext_inst!(
        Tosa0010001,
        Tosa0010001Op,
        DEPTHWISE_CONV2D,
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
            (IdRef, One)
        ]
    ),
    ext_inst!(
        Tosa0010001,
        Tosa0010001Op,
        FFT2D,
        [],
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        Tosa0010001,
        Tosa0010001Op,
        MATMUL,
        [],
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        Tosa0010001,
        Tosa0010001Op,
        MAX_POOL2D,
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
        Tosa0010001,
        Tosa0010001Op,
        RFFT2D,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        Tosa0010001,
        Tosa0010001Op,
        TRANSPOSE_CONV2D,
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
            (IdRef, One)
        ]
    ),
    ext_inst!(
        Tosa0010001,
        Tosa0010001Op,
        CLAMP,
        [],
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(Tosa0010001, Tosa0010001Op, ERF, [], [], [(IdRef, One)]),
    ext_inst!(Tosa0010001, Tosa0010001Op, SIGMOID, [], [], [(IdRef, One)]),
    ext_inst!(Tosa0010001, Tosa0010001Op, TANH, [], [], [(IdRef, One)]),
    ext_inst!(
        Tosa0010001,
        Tosa0010001Op,
        ADD,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        Tosa0010001,
        Tosa0010001Op,
        ARITHMETIC_RIGHT_SHIFT,
        [],
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        Tosa0010001,
        Tosa0010001Op,
        BITWISE_AND,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        Tosa0010001,
        Tosa0010001Op,
        BITWISE_OR,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        Tosa0010001,
        Tosa0010001Op,
        BITWISE_XOR,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        Tosa0010001,
        Tosa0010001Op,
        INTDIV,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        Tosa0010001,
        Tosa0010001Op,
        LOGICAL_AND,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        Tosa0010001,
        Tosa0010001Op,
        LOGICAL_LEFT_SHIFT,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        Tosa0010001,
        Tosa0010001Op,
        LOGICAL_RIGHT_SHIFT,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        Tosa0010001,
        Tosa0010001Op,
        LOGICAL_OR,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        Tosa0010001,
        Tosa0010001Op,
        LOGICAL_XOR,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        Tosa0010001,
        Tosa0010001Op,
        MAXIMUM,
        [],
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        Tosa0010001,
        Tosa0010001Op,
        MINIMUM,
        [],
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        Tosa0010001,
        Tosa0010001Op,
        MUL,
        [],
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        Tosa0010001,
        Tosa0010001Op,
        POW,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        Tosa0010001,
        Tosa0010001Op,
        SUB,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        Tosa0010001,
        Tosa0010001Op,
        TABLE,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(Tosa0010001, Tosa0010001Op, ABS, [], [], [(IdRef, One)]),
    ext_inst!(
        Tosa0010001,
        Tosa0010001Op,
        BITWISE_NOT,
        [],
        [],
        [(IdRef, One)]
    ),
    ext_inst!(Tosa0010001, Tosa0010001Op, CEIL, [], [], [(IdRef, One)]),
    ext_inst!(Tosa0010001, Tosa0010001Op, CLZ, [], [], [(IdRef, One)]),
    ext_inst!(Tosa0010001, Tosa0010001Op, COS, [], [], [(IdRef, One)]),
    ext_inst!(Tosa0010001, Tosa0010001Op, EXP, [], [], [(IdRef, One)]),
    ext_inst!(Tosa0010001, Tosa0010001Op, FLOOR, [], [], [(IdRef, One)]),
    ext_inst!(Tosa0010001, Tosa0010001Op, LOG, [], [], [(IdRef, One)]),
    ext_inst!(
        Tosa0010001,
        Tosa0010001Op,
        LOGICAL_NOT,
        [],
        [],
        [(IdRef, One)]
    ),
    ext_inst!(
        Tosa0010001,
        Tosa0010001Op,
        NEGATE,
        [],
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        Tosa0010001,
        Tosa0010001Op,
        RECIPROCAL,
        [],
        [],
        [(IdRef, One)]
    ),
    ext_inst!(Tosa0010001, Tosa0010001Op, RSQRT, [], [], [(IdRef, One)]),
    ext_inst!(Tosa0010001, Tosa0010001Op, SIN, [], [], [(IdRef, One)]),
    ext_inst!(
        Tosa0010001,
        Tosa0010001Op,
        SELECT,
        [],
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        Tosa0010001,
        Tosa0010001Op,
        EQUAL,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        Tosa0010001,
        Tosa0010001Op,
        GREATER,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        Tosa0010001,
        Tosa0010001Op,
        GREATER_EQUAL,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        Tosa0010001,
        Tosa0010001Op,
        REDUCE_ALL,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        Tosa0010001,
        Tosa0010001Op,
        REDUCE_ANY,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        Tosa0010001,
        Tosa0010001Op,
        REDUCE_MAX,
        [],
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        Tosa0010001,
        Tosa0010001Op,
        REDUCE_MIN,
        [],
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        Tosa0010001,
        Tosa0010001Op,
        REDUCE_PRODUCT,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        Tosa0010001,
        Tosa0010001Op,
        REDUCE_SUM,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        Tosa0010001,
        Tosa0010001Op,
        CONCAT,
        [],
        [],
        [(IdRef, One), (IdRef, ZeroOrMore)]
    ),
    ext_inst!(
        Tosa0010001,
        Tosa0010001Op,
        PAD,
        [],
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        Tosa0010001,
        Tosa0010001Op,
        RESHAPE,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        Tosa0010001,
        Tosa0010001Op,
        REVERSE,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        Tosa0010001,
        Tosa0010001Op,
        SLICE,
        [],
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        Tosa0010001,
        Tosa0010001Op,
        TILE,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        Tosa0010001,
        Tosa0010001Op,
        TRANSPOSE,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        Tosa0010001,
        Tosa0010001Op,
        GATHER,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        Tosa0010001,
        Tosa0010001Op,
        SCATTER,
        [],
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        Tosa0010001,
        Tosa0010001Op,
        RESIZE,
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
    ext_inst!(Tosa0010001, Tosa0010001Op, CAST, [], [], [(IdRef, One)]),
    ext_inst!(
        Tosa0010001,
        Tosa0010001Op,
        RESCALE,
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
            (IdRef, One)
        ]
    ),
];
pub static TOSA_001000_1_INSTRUCTION_TABLE: InstructionTable<ExtInstOp> =
    InstructionTable(TOSA_001000_1_INSTRUCTIONS, std::marker::PhantomData);

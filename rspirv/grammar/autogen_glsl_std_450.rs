// AUTOMATICALLY GENERATED from the SPIR-V JSON grammar:
//   external/spirv.core.grammar.json.
// DO NOT MODIFY!

static GLSL_STD_450_INSTRUCTIONS: &[ExtendedInstruction<'static>] = &[
    ext_inst!(GlslStd450, GlslStd450Op, Round, [], [], [(IdRef, One)]),
    ext_inst!(GlslStd450, GlslStd450Op, RoundEven, [], [], [(IdRef, One)]),
    ext_inst!(GlslStd450, GlslStd450Op, Trunc, [], [], [(IdRef, One)]),
    ext_inst!(GlslStd450, GlslStd450Op, FAbs, [], [], [(IdRef, One)]),
    ext_inst!(GlslStd450, GlslStd450Op, SAbs, [], [], [(IdRef, One)]),
    ext_inst!(GlslStd450, GlslStd450Op, FSign, [], [], [(IdRef, One)]),
    ext_inst!(GlslStd450, GlslStd450Op, SSign, [], [], [(IdRef, One)]),
    ext_inst!(GlslStd450, GlslStd450Op, Floor, [], [], [(IdRef, One)]),
    ext_inst!(GlslStd450, GlslStd450Op, Ceil, [], [], [(IdRef, One)]),
    ext_inst!(GlslStd450, GlslStd450Op, Fract, [], [], [(IdRef, One)]),
    ext_inst!(GlslStd450, GlslStd450Op, Radians, [], [], [(IdRef, One)]),
    ext_inst!(GlslStd450, GlslStd450Op, Degrees, [], [], [(IdRef, One)]),
    ext_inst!(GlslStd450, GlslStd450Op, Sin, [], [], [(IdRef, One)]),
    ext_inst!(GlslStd450, GlslStd450Op, Cos, [], [], [(IdRef, One)]),
    ext_inst!(GlslStd450, GlslStd450Op, Tan, [], [], [(IdRef, One)]),
    ext_inst!(GlslStd450, GlslStd450Op, Asin, [], [], [(IdRef, One)]),
    ext_inst!(GlslStd450, GlslStd450Op, Acos, [], [], [(IdRef, One)]),
    ext_inst!(GlslStd450, GlslStd450Op, Atan, [], [], [(IdRef, One)]),
    ext_inst!(GlslStd450, GlslStd450Op, Sinh, [], [], [(IdRef, One)]),
    ext_inst!(GlslStd450, GlslStd450Op, Cosh, [], [], [(IdRef, One)]),
    ext_inst!(GlslStd450, GlslStd450Op, Tanh, [], [], [(IdRef, One)]),
    ext_inst!(GlslStd450, GlslStd450Op, Asinh, [], [], [(IdRef, One)]),
    ext_inst!(GlslStd450, GlslStd450Op, Acosh, [], [], [(IdRef, One)]),
    ext_inst!(GlslStd450, GlslStd450Op, Atanh, [], [], [(IdRef, One)]),
    ext_inst!(
        GlslStd450,
        GlslStd450Op,
        Atan2,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        GlslStd450,
        GlslStd450Op,
        Pow,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(GlslStd450, GlslStd450Op, Exp, [], [], [(IdRef, One)]),
    ext_inst!(GlslStd450, GlslStd450Op, Log, [], [], [(IdRef, One)]),
    ext_inst!(GlslStd450, GlslStd450Op, Exp2, [], [], [(IdRef, One)]),
    ext_inst!(GlslStd450, GlslStd450Op, Log2, [], [], [(IdRef, One)]),
    ext_inst!(GlslStd450, GlslStd450Op, Sqrt, [], [], [(IdRef, One)]),
    ext_inst!(
        GlslStd450,
        GlslStd450Op,
        InverseSqrt,
        [],
        [],
        [(IdRef, One)]
    ),
    ext_inst!(
        GlslStd450,
        GlslStd450Op,
        Determinant,
        [],
        [],
        [(IdRef, One)]
    ),
    ext_inst!(
        GlslStd450,
        GlslStd450Op,
        MatrixInverse,
        [],
        [],
        [(IdRef, One)]
    ),
    ext_inst!(
        GlslStd450,
        GlslStd450Op,
        Modf,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(GlslStd450, GlslStd450Op, ModfStruct, [], [], [(IdRef, One)]),
    ext_inst!(
        GlslStd450,
        GlslStd450Op,
        FMin,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        GlslStd450,
        GlslStd450Op,
        UMin,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        GlslStd450,
        GlslStd450Op,
        SMin,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        GlslStd450,
        GlslStd450Op,
        FMax,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        GlslStd450,
        GlslStd450Op,
        UMax,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        GlslStd450,
        GlslStd450Op,
        SMax,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        GlslStd450,
        GlslStd450Op,
        FClamp,
        [],
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        GlslStd450,
        GlslStd450Op,
        UClamp,
        [],
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        GlslStd450,
        GlslStd450Op,
        SClamp,
        [],
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        GlslStd450,
        GlslStd450Op,
        FMix,
        [],
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        GlslStd450,
        GlslStd450Op,
        IMix,
        [],
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        GlslStd450,
        GlslStd450Op,
        Step,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        GlslStd450,
        GlslStd450Op,
        SmoothStep,
        [],
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        GlslStd450,
        GlslStd450Op,
        Fma,
        [],
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        GlslStd450,
        GlslStd450Op,
        Frexp,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        GlslStd450,
        GlslStd450Op,
        FrexpStruct,
        [],
        [],
        [(IdRef, One)]
    ),
    ext_inst!(
        GlslStd450,
        GlslStd450Op,
        Ldexp,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        GlslStd450,
        GlslStd450Op,
        PackSnorm4x8,
        [],
        [],
        [(IdRef, One)]
    ),
    ext_inst!(
        GlslStd450,
        GlslStd450Op,
        PackUnorm4x8,
        [],
        [],
        [(IdRef, One)]
    ),
    ext_inst!(
        GlslStd450,
        GlslStd450Op,
        PackSnorm2x16,
        [],
        [],
        [(IdRef, One)]
    ),
    ext_inst!(
        GlslStd450,
        GlslStd450Op,
        PackUnorm2x16,
        [],
        [],
        [(IdRef, One)]
    ),
    ext_inst!(
        GlslStd450,
        GlslStd450Op,
        PackHalf2x16,
        [],
        [],
        [(IdRef, One)]
    ),
    ext_inst!(
        GlslStd450,
        GlslStd450Op,
        PackDouble2x32,
        [Float64],
        [],
        [(IdRef, One)]
    ),
    ext_inst!(
        GlslStd450,
        GlslStd450Op,
        UnpackSnorm2x16,
        [],
        [],
        [(IdRef, One)]
    ),
    ext_inst!(
        GlslStd450,
        GlslStd450Op,
        UnpackUnorm2x16,
        [],
        [],
        [(IdRef, One)]
    ),
    ext_inst!(
        GlslStd450,
        GlslStd450Op,
        UnpackHalf2x16,
        [],
        [],
        [(IdRef, One)]
    ),
    ext_inst!(
        GlslStd450,
        GlslStd450Op,
        UnpackSnorm4x8,
        [],
        [],
        [(IdRef, One)]
    ),
    ext_inst!(
        GlslStd450,
        GlslStd450Op,
        UnpackUnorm4x8,
        [],
        [],
        [(IdRef, One)]
    ),
    ext_inst!(
        GlslStd450,
        GlslStd450Op,
        UnpackDouble2x32,
        [Float64],
        [],
        [(IdRef, One)]
    ),
    ext_inst!(GlslStd450, GlslStd450Op, Length, [], [], [(IdRef, One)]),
    ext_inst!(
        GlslStd450,
        GlslStd450Op,
        Distance,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        GlslStd450,
        GlslStd450Op,
        Cross,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(GlslStd450, GlslStd450Op, Normalize, [], [], [(IdRef, One)]),
    ext_inst!(
        GlslStd450,
        GlslStd450Op,
        FaceForward,
        [],
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        GlslStd450,
        GlslStd450Op,
        Reflect,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        GlslStd450,
        GlslStd450Op,
        Refract,
        [],
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(GlslStd450, GlslStd450Op, FindILsb, [], [], [(IdRef, One)]),
    ext_inst!(GlslStd450, GlslStd450Op, FindSMsb, [], [], [(IdRef, One)]),
    ext_inst!(GlslStd450, GlslStd450Op, FindUMsb, [], [], [(IdRef, One)]),
    ext_inst!(
        GlslStd450,
        GlslStd450Op,
        InterpolateAtCentroid,
        [InterpolationFunction],
        [],
        [(IdRef, One)]
    ),
    ext_inst!(
        GlslStd450,
        GlslStd450Op,
        InterpolateAtSample,
        [InterpolationFunction],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        GlslStd450,
        GlslStd450Op,
        InterpolateAtOffset,
        [InterpolationFunction],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        GlslStd450,
        GlslStd450Op,
        NMin,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        GlslStd450,
        GlslStd450Op,
        NMax,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        GlslStd450,
        GlslStd450Op,
        NClamp,
        [],
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
];
pub static GLSL_STD_450_INSTRUCTION_TABLE: InstructionTable<ExtInstOp> =
    InstructionTable(GLSL_STD_450_INSTRUCTIONS, std::marker::PhantomData);

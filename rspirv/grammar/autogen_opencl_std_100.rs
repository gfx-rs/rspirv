// AUTOMATICALLY GENERATED from the SPIR-V JSON grammar:
//   external/spirv.core.grammar.json.
// DO NOT MODIFY!

static OPENCL_STD_100_INSTRUCTIONS: &[ExtendedInstruction<'static>] = &[
    ext_inst!(OpenclStd100, OpenclStd100Op, acos, [], [], [(IdRef, One)]),
    ext_inst!(OpenclStd100, OpenclStd100Op, acosh, [], [], [(IdRef, One)]),
    ext_inst!(OpenclStd100, OpenclStd100Op, acospi, [], [], [(IdRef, One)]),
    ext_inst!(OpenclStd100, OpenclStd100Op, asin, [], [], [(IdRef, One)]),
    ext_inst!(OpenclStd100, OpenclStd100Op, asinh, [], [], [(IdRef, One)]),
    ext_inst!(OpenclStd100, OpenclStd100Op, asinpi, [], [], [(IdRef, One)]),
    ext_inst!(OpenclStd100, OpenclStd100Op, atan, [], [], [(IdRef, One)]),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        atan2,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(OpenclStd100, OpenclStd100Op, atanh, [], [], [(IdRef, One)]),
    ext_inst!(OpenclStd100, OpenclStd100Op, atanpi, [], [], [(IdRef, One)]),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        atan2pi,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(OpenclStd100, OpenclStd100Op, cbrt, [], [], [(IdRef, One)]),
    ext_inst!(OpenclStd100, OpenclStd100Op, ceil, [], [], [(IdRef, One)]),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        copysign,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(OpenclStd100, OpenclStd100Op, cos, [], [], [(IdRef, One)]),
    ext_inst!(OpenclStd100, OpenclStd100Op, cosh, [], [], [(IdRef, One)]),
    ext_inst!(OpenclStd100, OpenclStd100Op, cospi, [], [], [(IdRef, One)]),
    ext_inst!(OpenclStd100, OpenclStd100Op, erfc, [], [], [(IdRef, One)]),
    ext_inst!(OpenclStd100, OpenclStd100Op, erf, [], [], [(IdRef, One)]),
    ext_inst!(OpenclStd100, OpenclStd100Op, exp, [], [], [(IdRef, One)]),
    ext_inst!(OpenclStd100, OpenclStd100Op, exp2, [], [], [(IdRef, One)]),
    ext_inst!(OpenclStd100, OpenclStd100Op, exp10, [], [], [(IdRef, One)]),
    ext_inst!(OpenclStd100, OpenclStd100Op, expm1, [], [], [(IdRef, One)]),
    ext_inst!(OpenclStd100, OpenclStd100Op, fabs, [], [], [(IdRef, One)]),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        fdim,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(OpenclStd100, OpenclStd100Op, floor, [], [], [(IdRef, One)]),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        fma,
        [],
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        fmax,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        fmin,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        fmod,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        fract,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        frexp,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        hypot,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(OpenclStd100, OpenclStd100Op, ilogb, [], [], [(IdRef, One)]),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        ldexp,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(OpenclStd100, OpenclStd100Op, lgamma, [], [], [(IdRef, One)]),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        lgamma_r,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(OpenclStd100, OpenclStd100Op, log, [], [], [(IdRef, One)]),
    ext_inst!(OpenclStd100, OpenclStd100Op, log2, [], [], [(IdRef, One)]),
    ext_inst!(OpenclStd100, OpenclStd100Op, log10, [], [], [(IdRef, One)]),
    ext_inst!(OpenclStd100, OpenclStd100Op, log1p, [], [], [(IdRef, One)]),
    ext_inst!(OpenclStd100, OpenclStd100Op, logb, [], [], [(IdRef, One)]),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        mad,
        [],
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        maxmag,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        minmag,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        modf,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(OpenclStd100, OpenclStd100Op, nan, [], [], [(IdRef, One)]),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        nextafter,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        pow,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        pown,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        powr,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        remainder,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        remquo,
        [],
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(OpenclStd100, OpenclStd100Op, rint, [], [], [(IdRef, One)]),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        rootn,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(OpenclStd100, OpenclStd100Op, round, [], [], [(IdRef, One)]),
    ext_inst!(OpenclStd100, OpenclStd100Op, rsqrt, [], [], [(IdRef, One)]),
    ext_inst!(OpenclStd100, OpenclStd100Op, sin, [], [], [(IdRef, One)]),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        sincos,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(OpenclStd100, OpenclStd100Op, sinh, [], [], [(IdRef, One)]),
    ext_inst!(OpenclStd100, OpenclStd100Op, sinpi, [], [], [(IdRef, One)]),
    ext_inst!(OpenclStd100, OpenclStd100Op, sqrt, [], [], [(IdRef, One)]),
    ext_inst!(OpenclStd100, OpenclStd100Op, tan, [], [], [(IdRef, One)]),
    ext_inst!(OpenclStd100, OpenclStd100Op, tanh, [], [], [(IdRef, One)]),
    ext_inst!(OpenclStd100, OpenclStd100Op, tanpi, [], [], [(IdRef, One)]),
    ext_inst!(OpenclStd100, OpenclStd100Op, tgamma, [], [], [(IdRef, One)]),
    ext_inst!(OpenclStd100, OpenclStd100Op, trunc, [], [], [(IdRef, One)]),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        half_cos,
        [],
        [],
        [(IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        half_divide,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        half_exp,
        [],
        [],
        [(IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        half_exp2,
        [],
        [],
        [(IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        half_exp10,
        [],
        [],
        [(IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        half_log,
        [],
        [],
        [(IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        half_log2,
        [],
        [],
        [(IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        half_log10,
        [],
        [],
        [(IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        half_powr,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        half_recip,
        [],
        [],
        [(IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        half_rsqrt,
        [],
        [],
        [(IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        half_sin,
        [],
        [],
        [(IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        half_sqrt,
        [],
        [],
        [(IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        half_tan,
        [],
        [],
        [(IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        native_cos,
        [],
        [],
        [(IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        native_divide,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        native_exp,
        [],
        [],
        [(IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        native_exp2,
        [],
        [],
        [(IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        native_exp10,
        [],
        [],
        [(IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        native_log,
        [],
        [],
        [(IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        native_log2,
        [],
        [],
        [(IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        native_log10,
        [],
        [],
        [(IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        native_powr,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        native_recip,
        [],
        [],
        [(IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        native_rsqrt,
        [],
        [],
        [(IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        native_sin,
        [],
        [],
        [(IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        native_sqrt,
        [],
        [],
        [(IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        native_tan,
        [],
        [],
        [(IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        fclamp,
        [],
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        degrees,
        [],
        [],
        [(IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        fmax_common,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        fmin_common,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        mix,
        [],
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        radians,
        [],
        [],
        [(IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        step,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        smoothstep,
        [],
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(OpenclStd100, OpenclStd100Op, sign, [], [], [(IdRef, One)]),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        cross,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        distance,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(OpenclStd100, OpenclStd100Op, length, [], [], [(IdRef, One)]),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        normalize,
        [],
        [],
        [(IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        fast_distance,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        fast_length,
        [],
        [],
        [(IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        fast_normalize,
        [],
        [],
        [(IdRef, One)]
    ),
    ext_inst!(OpenclStd100, OpenclStd100Op, s_abs, [], [], [(IdRef, One)]),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        s_abs_diff,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        s_add_sat,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        u_add_sat,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        s_hadd,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        u_hadd,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        s_rhadd,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        u_rhadd,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        s_clamp,
        [],
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        u_clamp,
        [],
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(OpenclStd100, OpenclStd100Op, clz, [], [], [(IdRef, One)]),
    ext_inst!(OpenclStd100, OpenclStd100Op, ctz, [], [], [(IdRef, One)]),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        s_mad_hi,
        [],
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        u_mad_sat,
        [],
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        s_mad_sat,
        [],
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        s_max,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        u_max,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        s_min,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        u_min,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        s_mul_hi,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        rotate,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        s_sub_sat,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        u_sub_sat,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        u_upsample,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        s_upsample,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        popcount,
        [],
        [],
        [(IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        s_mad24,
        [],
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        u_mad24,
        [],
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        s_mul24,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        u_mul24,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        vloadn,
        [],
        [],
        [(IdRef, One), (IdRef, One), (LiteralInteger, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        vstoren,
        [],
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        vload_half,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        vload_halfn,
        [],
        [],
        [(IdRef, One), (IdRef, One), (LiteralInteger, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        vstore_half,
        [],
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        vstore_half_r,
        [],
        [],
        [
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (FPRoundingMode, One)
        ]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        vstore_halfn,
        [],
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        vstore_halfn_r,
        [],
        [],
        [
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (FPRoundingMode, One)
        ]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        vloada_halfn,
        [],
        [],
        [(IdRef, One), (IdRef, One), (LiteralInteger, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        vstorea_halfn,
        [],
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        vstorea_halfn_r,
        [],
        [],
        [
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (FPRoundingMode, One)
        ]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        shuffle,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        shuffle2,
        [],
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        printf,
        [],
        [],
        [(IdRef, One), (IdRef, ZeroOrMore)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        prefetch,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        bitselect,
        [],
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        select,
        [],
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(OpenclStd100, OpenclStd100Op, u_abs, [], [], [(IdRef, One)]),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        u_abs_diff,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        u_mul_hi,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        OpenclStd100,
        OpenclStd100Op,
        u_mad_hi,
        [],
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
];
pub static OPENCL_STD_100_INSTRUCTION_TABLE: InstructionTable<ExtInstOp> =
    InstructionTable(OPENCL_STD_100_INSTRUCTIONS, std::marker::PhantomData);

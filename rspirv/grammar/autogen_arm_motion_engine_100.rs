// AUTOMATICALLY GENERATED from the SPIR-V JSON grammar:
//   external/spirv.core.grammar.json.
// DO NOT MODIFY!

static ARM_MOTION_ENGINE_100_INSTRUCTIONS: &[ExtendedInstruction<'static>] = &[
    ext_inst!(
        ArmMotionEngine100,
        ArmMotionEngine100Op,
        MIN_SAD,
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
        ArmMotionEngine100,
        ArmMotionEngine100Op,
        MIN_SAD_COST,
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
        ArmMotionEngine100,
        ArmMotionEngine100Op,
        RAW_SAD,
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
];
pub static ARM_MOTION_ENGINE_100_INSTRUCTION_TABLE: InstructionTable<ExtInstOp> =
    InstructionTable(ARM_MOTION_ENGINE_100_INSTRUCTIONS, std::marker::PhantomData);

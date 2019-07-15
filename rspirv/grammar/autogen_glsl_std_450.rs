// Copyright 2016 Google Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// AUTOMATICALLY GENERATED from the SPIR-V JSON grammar:
//   external/spirv.core.grammar.json.
// DO NOT MODIFY!

#[cfg_attr(rustfmt, rustfmt_skip)]
static GLSL_STD_450_INSTRUCTION_TABLE: &'static [ExtendedInstruction<'static>] = &[
    ext_inst!(Round, 1, [], [(IdRef, One)]),
    ext_inst!(RoundEven, 2, [], [(IdRef, One)]),
    ext_inst!(Trunc, 3, [], [(IdRef, One)]),
    ext_inst!(FAbs, 4, [], [(IdRef, One)]),
    ext_inst!(SAbs, 5, [], [(IdRef, One)]),
    ext_inst!(FSign, 6, [], [(IdRef, One)]),
    ext_inst!(SSign, 7, [], [(IdRef, One)]),
    ext_inst!(Floor, 8, [], [(IdRef, One)]),
    ext_inst!(Ceil, 9, [], [(IdRef, One)]),
    ext_inst!(Fract, 10, [], [(IdRef, One)]),
    ext_inst!(Radians, 11, [], [(IdRef, One)]),
    ext_inst!(Degrees, 12, [], [(IdRef, One)]),
    ext_inst!(Sin, 13, [], [(IdRef, One)]),
    ext_inst!(Cos, 14, [], [(IdRef, One)]),
    ext_inst!(Tan, 15, [], [(IdRef, One)]),
    ext_inst!(Asin, 16, [], [(IdRef, One)]),
    ext_inst!(Acos, 17, [], [(IdRef, One)]),
    ext_inst!(Atan, 18, [], [(IdRef, One)]),
    ext_inst!(Sinh, 19, [], [(IdRef, One)]),
    ext_inst!(Cosh, 20, [], [(IdRef, One)]),
    ext_inst!(Tanh, 21, [], [(IdRef, One)]),
    ext_inst!(Asinh, 22, [], [(IdRef, One)]),
    ext_inst!(Acosh, 23, [], [(IdRef, One)]),
    ext_inst!(Atanh, 24, [], [(IdRef, One)]),
    ext_inst!(Atan2, 25, [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(Pow, 26, [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(Exp, 27, [], [(IdRef, One)]),
    ext_inst!(Log, 28, [], [(IdRef, One)]),
    ext_inst!(Exp2, 29, [], [(IdRef, One)]),
    ext_inst!(Log2, 30, [], [(IdRef, One)]),
    ext_inst!(Sqrt, 31, [], [(IdRef, One)]),
    ext_inst!(InverseSqrt, 32, [], [(IdRef, One)]),
    ext_inst!(Determinant, 33, [], [(IdRef, One)]),
    ext_inst!(MatrixInverse, 34, [], [(IdRef, One)]),
    ext_inst!(Modf, 35, [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(ModfStruct, 36, [], [(IdRef, One)]),
    ext_inst!(FMin, 37, [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(UMin, 38, [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(SMin, 39, [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(FMax, 40, [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(UMax, 41, [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(SMax, 42, [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(FClamp, 43, [], [(IdRef, One), (IdRef, One), (IdRef, One)]),
    ext_inst!(UClamp, 44, [], [(IdRef, One), (IdRef, One), (IdRef, One)]),
    ext_inst!(SClamp, 45, [], [(IdRef, One), (IdRef, One), (IdRef, One)]),
    ext_inst!(FMix, 46, [], [(IdRef, One), (IdRef, One), (IdRef, One)]),
    ext_inst!(IMix, 47, [], [(IdRef, One), (IdRef, One), (IdRef, One)]),
    ext_inst!(Step, 48, [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(SmoothStep, 49, [], [(IdRef, One), (IdRef, One), (IdRef, One)]),
    ext_inst!(Fma, 50, [], [(IdRef, One), (IdRef, One), (IdRef, One)]),
    ext_inst!(Frexp, 51, [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(FrexpStruct, 52, [], [(IdRef, One)]),
    ext_inst!(Ldexp, 53, [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(PackSnorm4x8, 54, [], [(IdRef, One)]),
    ext_inst!(PackUnorm4x8, 55, [], [(IdRef, One)]),
    ext_inst!(PackSnorm2x16, 56, [], [(IdRef, One)]),
    ext_inst!(PackUnorm2x16, 57, [], [(IdRef, One)]),
    ext_inst!(PackHalf2x16, 58, [], [(IdRef, One)]),
    ext_inst!(PackDouble2x32, 59, [Float64], [(IdRef, One)]),
    ext_inst!(UnpackSnorm2x16, 60, [], [(IdRef, One)]),
    ext_inst!(UnpackUnorm2x16, 61, [], [(IdRef, One)]),
    ext_inst!(UnpackHalf2x16, 62, [], [(IdRef, One)]),
    ext_inst!(UnpackSnorm4x8, 63, [], [(IdRef, One)]),
    ext_inst!(UnpackUnorm4x8, 64, [], [(IdRef, One)]),
    ext_inst!(UnpackDouble2x32, 65, [Float64], [(IdRef, One)]),
    ext_inst!(Length, 66, [], [(IdRef, One)]),
    ext_inst!(Distance, 67, [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(Cross, 68, [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(Normalize, 69, [], [(IdRef, One)]),
    ext_inst!(FaceForward, 70, [], [(IdRef, One), (IdRef, One), (IdRef, One)]),
    ext_inst!(Reflect, 71, [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(Refract, 72, [], [(IdRef, One), (IdRef, One), (IdRef, One)]),
    ext_inst!(FindILsb, 73, [], [(IdRef, One)]),
    ext_inst!(FindSMsb, 74, [], [(IdRef, One)]),
    ext_inst!(FindUMsb, 75, [], [(IdRef, One)]),
    ext_inst!(InterpolateAtCentroid, 76, [InterpolationFunction], [(IdRef, One)]),
    ext_inst!(InterpolateAtSample, 77, [InterpolationFunction], [(IdRef, One), (IdRef, One)]),
    ext_inst!(InterpolateAtOffset, 78, [InterpolationFunction], [(IdRef, One), (IdRef, One)]),
    ext_inst!(NMin, 79, [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(NMax, 80, [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(NClamp, 81, [], [(IdRef, One), (IdRef, One), (IdRef, One)]),
];

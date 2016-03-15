#!/usr/bin/env python
"""A script for generating operand_type.rs from libspirv.h."""

from __future__ import print_function

import argparse
import os.path
import re

import clang.cindex as libclang


def stylify_name(name):
    """Change name from SOURCE_STYLE to DestinationStyle."""
    return ''.join([str.capitalize(s) for s in name.split('_')])


def gen_enum_definition(node, enum_class, remove_last=None):
    """Generates enum definition from a clang EnumDecl AST node.

    Assumes the AST subtree has the following structure:
      ENUM_DECL
        ENUM_CONSTANT_DECL
        ENUM_CONSTANT_DECL
        ...
    """
    assert node.kind is libclang.CursorKind.ENUM_DECL
    case_names = []

    for case in node.get_children():
        assert case.kind is libclang.CursorKind.ENUM_CONSTANT_DECL
        case_names.append(case.spelling)  # Enumerant name

    if case_names[-1] == remove_last:
        case_names = case_names[:-1]

    # Remove the common prefix from all enumerants,
    # except those with prefix trimmed will start with numbers.
    common_prefix = os.path.commonprefix(case_names)
    trimmed_case_names = [c[len(common_prefix):] for c in case_names]
    if not any([re.match(r'\d', c) for c in trimmed_case_names]):
        case_names = trimmed_case_names

    case_names = [stylify_name(s) for s in case_names]

    cases = ['    {},'.format(c) for c in case_names]

    return '{attribute}\npub enum {enum_class} {{\n{enumerants}\n}}'.format(
        attribute='#[derive(Clone, Copy, Debug)]',
        enum_class=enum_class,
        enumerants='\n'.join(cases))


def generate_operand_type_rs(libspirv_h_path):
    assert libspirv_h_path.endswith('libspirv.h')
    index = libclang.Index.create()
    tu = index.parse(libspirv_h_path, ['-x', 'c'])

    enums = [node for node in tu.cursor.get_children()
             if node.kind is libclang.CursorKind.ENUM_DECL and
             node.spelling == 'spv_operand_type_t']
    assert len(enums) == 1
    operand_type = gen_enum_definition(
        enums[0], 'OperandType', '_spv_operand_type_t')

    return '{allows}\n\n{enums}'.format(
        allows='#![allow(dead_code)]', enums=operand_type)

if __name__ == '__main__':
    parser = argparse.ArgumentParser(
        description='Generate operand_type.rs from libspirv.h')
    parser.add_argument('path', type=str, metavar='<path>',
                        help='Path to libspirv.h')
    args = parser.parse_args()

    print('// This rust module is automatically generated from ',
          'SPIRV-Tools libspirv.h:')
    print('//   https://github.com/KhronosGroup/SPIRV-Tools'
          '/blob/master/include/spirv-tools/libspirv.h\n')
    print(generate_operand_type_rs(args.path))

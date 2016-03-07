#!/usr/bin/env python
"""A script for generating spirv.rs from spirv.hpp."""

from __future__ import print_function

import argparse
import itertools
import os.path
import re

import clang.cindex as libclang


def gen_variable_definition(node):
    """Generates variable definition from a clang VarDecl AST node.

    Assumes the AST subtree has the following structure:
      VAR_DECL
        UNEXPOSED_EXPR
          INTEGER_LITERAL (with two tokens)
    """
    assert node.kind is libclang.CursorKind.VAR_DECL
    assert node.type.spelling == 'const unsigned int'
    variable = node.spelling

    # Convert variable to the RUST_CONSTANT_STYLE.
    variable = re.sub(r'([a-z])([A-Z])', r'\1_\2', variable).upper()

    children = list(node.get_children())
    assert len(children) == 1
    assert children[0].kind is libclang.CursorKind.UNEXPOSED_EXPR

    children = list(children[0].get_children())
    assert len(children) == 1
    assert children[0].kind is libclang.CursorKind.INTEGER_LITERAL

    tokens = list(children[0].get_tokens())
    assert len(tokens) == 2
    assert tokens[1].spelling == ';'
    value = tokens[0].spelling

    return 'const {variable}: usize = {value};'.format(
        variable=variable, value=value)


def gen_enum_definition(node):
    """Generates enum definition from a clang EnumDecl AST node.

    Assumes the AST subtree has the following structure:
      ENUM_DECL
        ENUM_CONSTANT_DECL
          UNEXPOSED_EXPR
            INTEGER_LITERAL (with two tokens)
        ENUM_CONSTANT_DECL
          UNEXPOSED_EXPR
            INTEGER_LITERAL (with two tokens)
        ...
    """
    assert node.kind is libclang.CursorKind.ENUM_DECL
    enum_class = node.spelling
    case_names = []
    case_values = []

    for case in node.get_children():
        assert case.kind is libclang.CursorKind.ENUM_CONSTANT_DECL
        case_names.append(case.spelling)  # Enumerant name

        children = list(case.get_children())
        assert len(children) == 1
        assert children[0].kind is libclang.CursorKind.UNEXPOSED_EXPR

        children = list(children[0].get_children())
        assert len(children) == 1
        assert children[0].kind is libclang.CursorKind.INTEGER_LITERAL

        tokens = list(children[0].get_tokens())
        assert len(tokens) == 2
        assert tokens[1].spelling == ','
        case_values.append(tokens[0].spelling)  # Enumerant value

    # Remove the common prefix (with the enum class) from all enumerants,
    # except those with prefix trimmed will start with numbers.
    common_prefix = os.path.commonprefix(case_names + [enum_class])
    trimmed_case_names = [c[len(common_prefix):] for c in case_names]
    if not any([re.match(r'\d', c) for c in trimmed_case_names]):
        case_names = trimmed_case_names

    cases = itertools.imap('    {} = {},'.format, case_names, case_values)

    return '{attribute}\npub enum {enum_class} {{\n{enumerants}\n}}'.format(
        attribute='#[derive(Clone, Copy, Debug)]',
        enum_class=enum_class,
        enumerants='\n'.join(cases))


def generate_spirv_rs(spirv_hpp_path):
    assert spirv_hpp_path.endswith('spirv.hpp')
    index = libclang.Index.create()
    tu = index.parse(spirv_hpp_path, ['-x', 'c++'])

    namespace = list(tu.cursor.get_children())
    assert len(namespace) == 1
    assert namespace[0].kind is libclang.CursorKind.NAMESPACE

    consts = []
    enums = []

    for node in namespace[0].get_children():
        if node.kind is libclang.CursorKind.VAR_DECL:
            consts.append(gen_variable_definition(node))
        elif node.kind is libclang.CursorKind.ENUM_DECL:
            enums.append(gen_enum_definition(node))
    return '{allows}\n\n{types}\n\n{consts}\n\n{enums}'.format(
        allows='#![allow(dead_code)]\n#![allow(non_camel_case_types)]',
        types='pub type Word = u32;\npub type Id = u32;',
        consts='\n'.join(consts),
        enums='\n\n'.join(enums))

if __name__ == '__main__':
    parser = argparse.ArgumentParser(
        description='Generate spirv.rs from spirv.hpp')
    parser.add_argument('path', type=str, metavar='<path>',
                        help='Path to spirv.hpp')
    args = parser.parse_args()

    print('// This rust module is automatically generated from ',
          'SPIR-V C++ header file:')
    print('//   https://www.khronos.org/registry/spir-v/api/1.0/spirv.hpp\n')
    print(generate_spirv_rs(args.path))

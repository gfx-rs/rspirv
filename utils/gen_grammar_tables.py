#!/usr/bin/env python
# Copyright 2016 Google Inc.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#      http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

"""Generates various info tables from the SPIR-V JSON grammar."""

from __future__ import print_function

import json
import re

COPYRIGHT = '''// Copyright 2016 Google Inc.
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
'''

AUTO_GEN_COMMENT = ('// This rust module is automatically generated '
                    'from the SPIR-V JSON grammar:')

SPIRV_GRAMMAR_URL = ('//   https://github.com/KhronosGroup/SPIRV-Headers/\n'
                     '//           blob/master/include/spirv/1.1/'
                     'spirv.core.grammar.json')


def convert_symbol_to_snake_case(variable):
    return re.sub(r'([a-z])([A-Z])', r'\1_\2', variable).lower()


def get_decode_method(kind):
    assert not kind.startswith('Pair')
    if kind.startswith('Id'):
        return 'id'
    elif kind.startswith('Literal'):
        kind = kind[len('Literal'):]
    return convert_symbol_to_snake_case(kind)


def generate_operand_param_parse_methods(enums):
    """Returns the implementation of various operand param parsing methods."""
    # These are operand kinds that may have additional parameters.
    # So we need additional parsing methods for them to parse their
    # additional arguments.
    further_parse_methods = []
    for kind in enums:
        enumerants = []
        if 'enumerants' not in kind:
            continue
        for e in kind['enumerants']:
            ename = e['enumerant']
            if 'parameters' in e:
                eparam = [p['kind'] for p in e['parameters']]
                enumerants.append((ename, eparam))
        if len(enumerants) != 0:
            if kind['category'] == 'BitEnum':
                template = [
                    '    #[allow(unused_variables)]',
                    '    fn parse_{k}_arguments(&mut self, {k}: spirv::{kind})'
                    ' -> Result<Vec<mr::Operand>> {{',
                    '        unimplemented!()',
                    '    }}']
                decode_method = '\n'.join(template).format(
                    k=convert_symbol_to_snake_case(kind['kind']),
                    kind=kind['kind'])
                further_parse_methods.append((kind['kind'], decode_method))
            else:
                template = [
                    '    fn parse_{k}_arguments(&mut self, {k}: spirv::{kind})'
                    ' -> Result<Vec<mr::Operand>> {{',
                    '        Ok(match {k} {{',
                    '            {cases},',
                    '            {default}',
                    '        }})',
                    '    }}']
                cases = [
                    'spirv::{kind}::{ename} => vec![{eparams}]'.format(
                        kind=kind['kind'],
                        ename=name,
                        eparams=', '.join([
                            'mr::Operand::{kind}'
                            '(try_decode!(self.decoder.{decode}()))'.format(
                                kind=p, decode=get_decode_method(p))
                            for p in params]))
                    for name, params in enumerants]
                decode_method = '\n'.join(template).format(
                    k=convert_symbol_to_snake_case(kind['kind']),
                    kind=kind['kind'],
                    cases=',\n            '.join(cases),
                    default='_ => vec![]')
                further_parse_methods.append((kind['kind'], decode_method))
    return further_parse_methods


def generate_operand_parse_methods(enums):
    """Returns the implementation of various operand parsing methods."""
    further_parse_methods = generate_operand_param_parse_methods(enums)

    all_kinds = [e['kind'] for e in enums]

    further_parse_kinds = [e[0] for e in further_parse_methods]
    case_template = [
        'GOpKind::{kind} => {{',
        '                let val = try_decode!(self.decoder.{decode}());',
        '                let mut ops = vec![mr::Operand::{kind}(val)];',
        '                ops.append(&mut try!(self.parse_{k}_arguments(val)));',
        '                ops',
        '            }}']
    further_parse_cases = [
        '\n'.join(case_template).format(
            k=convert_symbol_to_snake_case(k),
            kind=k,
            decode=get_decode_method(k))
        for k in further_parse_kinds]

    # Logic operands that do not expand to concrete operand pairs,
    # that is, those operand kinds without 'Pair' name prefix.
    non_pair_cases = ['GOpKind::{kind} => vec![mr::Operand::{kind}'
                      '(try_decode!(self.decoder.{decode}()))]'.format(
                          kind=k, decode=get_decode_method(k))
                      for k in all_kinds if not (k in further_parse_kinds or
                                                 k.startswith('Pair'))]

    # Logic operands that expand to concrete operand pairs,
    # that is, those operand kinds with 'Pair' name prefix.
    # We only have three cases. So hard code it.
    pair_kinds = [('LiteralInteger', 'IdRef'),
                  ('IdRef', 'LiteralInteger'),
                  ('IdRef', 'IdRef')]
    case_template = [
        'GOpKind::{kind} => {{',
        '                vec!['
        'mr::Operand::{kind0}(try_decode!(self.decoder.{decode0}())), '
        'mr::Operand::{kind1}(try_decode!(self.decoder.{decode1}()))'
        ']',
        '            }}']
    pair_cases = [
        '\n'.join(case_template).format(
            kind='Pair{}{}'.format(k0, k1),
            kind0=k0, decode0=get_decode_method(k0),
            kind1=k1, decode1=get_decode_method(k1))
        for (k0, k1) in pair_kinds]

    # Parser method for requesting values of a specific operand kind and
    # converting it to mr::Operand.
    parser = ['{no_format}',
              "impl<'a> Parser<'a> {{",
              '    fn parse_operand(&mut self, kind: GOpKind) '
              '-> Result<Vec<mr::Operand>> {{',
              '        Ok(match kind {{',
              '            {non_pair_cases},',
              '            {pair_cases}',
              '            {further_parse_cases}',
              '        }})',
              '    }}\n',
              '{functions}',
              '}}']

    return '\n'.join(parser).format(
        no_format='',
        non_pair_cases=',\n            '.join(non_pair_cases),
        pair_cases='\n            '.join(pair_cases),
        further_parse_cases='\n            '.join(further_parse_cases),
        functions='\n\n'.join([e[1] for e in further_parse_methods]))


def update(grammar_input, operand_parse_output):
    """Updates all generated tables using the JSON grammar."""
    with open(grammar_input) as json_file:
        grammar = json.loads(json_file.read())

        operand_parse_output = open(operand_parse_output, 'w')

        print(COPYRIGHT, file=operand_parse_output)
        print(AUTO_GEN_COMMENT, file=operand_parse_output)
        print('{}\n'.format(SPIRV_GRAMMAR_URL), file=operand_parse_output)
        print(generate_operand_parse_methods(grammar['operand_kinds']),
              file=operand_parse_output)


def main():
    import argparse
    parser = argparse.ArgumentParser(
        description='Generate SPIR-V grammar tables')
    parser.add_argument('-i', '--input', metavar='<path>',
                        type=str, required=True,
                        help='input JSON file for core SPIR-V grammar')
    parser.add_argument('-d', '--operand-decode-output', metavar='<path>',
                        type=str, required=True,
                        help='output file for SPIR-V operand decoding methods')
    parser.add_argument('-p', '--operand-parse-output', metavar='<path>',
                        type=str, required=True,
                        help='output file for SPIR-V operand parsing methods')
    args = parser.parse_args()

    update(args.input, args.operand_parse_output)

if __name__ == '__main__':
    main()

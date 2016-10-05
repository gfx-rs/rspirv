#!/usr/bin/env python2
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

"""Updates various tables generated from SPIR-V grammar."""

import os
import imp
import sys

if len(sys.argv) != 2:
    print '{} /path/to/spirv/grammar/json'.format(sys.argv[0])
    exit(1)

grammar_input = sys.argv[1]

directory = os.path.dirname(os.path.realpath(__file__))

script = imp.load_source('gen_tables',
                         os.path.join(directory, 'gen_grammar_tables.py'))

src = os.path.join(directory, '..', 'src')
inst_table_output = os.path.join(src, 'grammar/table.rs')
operand_decode_output = os.path.join(src, 'binary/decode_operand.rs')
operand_error_output = os.path.join(src, 'binary/error.rs')
operand_parse_output = os.path.join(src, 'binary/parse_operand.rs')
mr_operand_output = os.path.join(src, 'mr/operand.rs')

script.update(grammar_input, inst_table_output, operand_decode_output,
              operand_error_output, operand_parse_output, mr_operand_output)

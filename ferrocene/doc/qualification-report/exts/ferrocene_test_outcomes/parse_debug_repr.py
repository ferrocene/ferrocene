# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

# The build metrics don't contain structured information about the parameters
# of a step (compiler, target, tested crates, etc), but only the stringified
# debug representation. This is a simple parser for that debug representation
# that we can use to extract what we want to render.

import re
from dataclasses import dataclass, field
from typing import Dict


# Regex passed by the tokenizer to the re.split() function. Chars matched by
# this regex will be used to identify where tokens should be split.
#
# Note that by default the re.split() function does not emit the delimiters.
# That behavior can be overridden by wrapping everything in a capture group
# (aka parentheses), which is what we're doing right now.
_TOKENIZER_RE = re.compile(r'([\s:\[\],"])')


class DebugReprParser:
    def __init__(self, input):
        self.input = input
        self.tokens = list(tokenize(input))
        self.idx = 0

    def parse_item(self):
        value = self._next()

        if value == "[":
            return self.parse_list()
        elif self._peek() == "{":
            self._next()  # Consume the '{'
            return self.parse_struct(value)
        elif value == '"':
            value = self._next()
            self._expect('"')
            return value
        else:
            return value

    def parse_list(self):
        contents = list()

        # Handle empty lists
        if self._peek() == "]":
            self._next()  # Consume the ]
            return contents

        while True:
            contents.append(self.parse_item())

            if self._peek() == "]":
                self._next()  # Consume the ]
                return contents
            self._expect(",")

    def parse_struct(self, name):
        struct = Struct(name)
        while True:
            field = self._next()
            self._expect(":")
            value = self.parse_item()
            struct.fields[field] = value

            if self._peek() == "}":
                self._next()  # Consume the }
                return struct
            self._expect(",")

    def _expect(self, token):
        next = self._next()
        if next != token:
            raise self._error(f"expected {repr(token)}, found {repr(next)}")

    def _next(self):
        try:
            token = self.tokens[self.idx]
        except IndexError:
            raise self._error("expected a token, found end of input")
        self.idx += 1
        return token

    def _peek(self, nth=0):
        try:
            return self.tokens[self.idx + nth]
        except IndexError:
            return None

    def _error(self, message):
        return ParserError(
            f"{message} at token {self.idx} while parsing {repr(self.input)}"
        )


@dataclass
class Struct:
    name: str
    fields: Dict[str, object] = field(default_factory=dict)


class ParserError(Exception):
    pass


def tokenize(input):
    for token in _TOKENIZER_RE.split(input):
        token = token.strip()
        if not token:
            continue
        yield token

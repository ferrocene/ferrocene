# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: Critical Section GmbH

from .definitions import DefIdNode, DefRefNode
from docutils import nodes
from sphinx.directives import SphinxDirective


class SyntaxDirective(SphinxDirective):
    has_content = True

    def run(self):
        node = nodes.literal_block("", "")
        node["classes"].append("spec-syntax")

        for child in Parser("\n".join(self.content), self.env.docname).parse():
            node += child

        return [node]


class Parser:
    def __init__(self, content, document_name):
        self.document_name = document_name
        self.lexer = Lexer(content).lex()
        self.peek_buffer = []

    def parse(self):
        while True:
            token = self.next()
            if token is None:
                return

            if token.kind == "literal":
                node = nodes.strong("", token.content)
                node["classes"].append("spec-syntax-literal")
                yield node

            elif token.kind == "identifier" and is_syntax_identifier(token.content):

                def peek_kind(kind, nth=0):
                    peeked = self.peek(nth)
                    return peeked is not None and peeked.kind == kind

                if peek_kind("definition", int(peek_kind("whitespace"))):
                    yield DefIdNode("syntaxes", token.content)
                else:
                    yield DefRefNode("syntaxes", self.document_name, token.content)

            else:
                yield nodes.Text(token.content)

    def next(self):
        if self.peek_buffer:
            return self.peek_buffer.pop(0)
        else:
            return next(self.lexer, None)

    def peek(self, nth=0):
        while len(self.peek_buffer) <= nth:
            token = next(self.lexer, None)
            if token is None:
                return None
            self.peek_buffer.append(token)
        return self.peek_buffer[nth]


class Lexer:
    def __init__(self, content):
        self.content = content
        self.pos = 0

    def lex(self):
        while True:
            char = self.next()
            if char is None:
                return

            if char == "$" and self.peek() == "$":
                self.next()  # Consume the second "$"

                # Collect all the chars until the next `$$`
                buffer = ""
                while True:
                    peeked = self.peek()
                    if peeked is None:
                        break
                    elif peeked == "$" and self.peek(1) == "$":
                        self.next()  # Consume the first "$"
                        self.next()  # Consume the second "$"
                        break
                    else:
                        buffer += self.next()
                yield Token("literal", buffer)

            elif char == ":" and self.peek() == ":" and self.peek(1) == "=":
                self.next()  # Consume the second ":"
                self.next()  # Consume the "="
                yield Token("definition", "::=")

            elif char.isalpha():
                buffer = char
                while True:
                    peeked = self.peek()
                    if peeked is None or not peeked.isalpha():
                        break
                    buffer += self.next()
                yield Token("identifier", buffer)

            elif char.isspace():
                buffer = char
                while True:
                    peeked = self.peek()
                    if peeked is None or not peeked.isspace():
                        break
                    buffer += self.next()
                yield Token("whitespace", buffer)

            else:
                yield Token("other", char)

    def peek(self, nth=0):
        try:
            return self.content[self.pos + nth]
        except IndexError:
            return None

    def next(self):
        try:
            char = self.content[self.pos]
        except IndexError:
            return None
        self.pos += 1
        return char


def is_syntax_identifier(identifier):
    EXPECT_ANY = 0
    EXPECT_UPPER = 1
    EXPECT_LOWER = 2

    expected = EXPECT_UPPER
    for char in identifier:
        if expected == EXPECT_UPPER:
            if not char.isupper():
                return False
            expected = EXPECT_LOWER
        elif expected == EXPECT_LOWER:
            if char.isupper():
                return False
            expected = EXPECT_ANY

    return True


class Token:
    def __init__(self, kind, content=None):
        self.kind = kind
        self.content = content

# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: Critical Section GmbH

from .definitions import DefIdNode, DefRefNode
from docutils import nodes
from sphinx.directives import SphinxDirective


class SyntaxDirective(SphinxDirective):
    has_content = True

    def run(self):
        # The first argument of creating any docutils node is the "raw source",
        # which in theory is completely optional for our needs. That's why all
        # the nodes we create pass an empty string (the default) to it.
        #
        # Still, this block is fairly special: to decide whether to perform
        # syntax highlighting on a literal block, Sphinx compares the raw
        # source with the output of the astext() method, and it highlights only
        # if the two are equal.
        #
        # We had problems in the past with the heuristic randomly causing the
        # syntax blocks to be highlighted by Sphinx (thus losing all our custom
        # formatting). Thus, to avoid problems we set the raw source to
        # something that will never appear in the astext() output (byte zero).
        node = nodes.literal_block("\0")

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
                    # We check that the third peek is not a "$" to perform a
                    # greedy parsing. This way, $$$$$ is parsed as "<b>$</b>"
                    # rather than "<b></b>$".
                    elif peeked == "$" and self.peek(1) == "$" and self.peek(2) != "$":
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
                    if peeked is None or not (peeked.isalpha() or peeked == "_"):
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

    # Some of the identifier referring to Unicode categories are called
    # XID_Category. The problem is that the XID_ portion is not a valid
    # identifier based on our rules. This code special-case that by ignoring
    # the problematic part of the identifier.
    if identifier.startswith("XID_"):
        identifier = identifier[len("XID_") :]

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

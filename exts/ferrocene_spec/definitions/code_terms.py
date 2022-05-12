from dataclasses import dataclass
from docutils import nodes
import sphinx


ROLE = "codeterm"
NAME = "code_terms"
PRETTY_NAME = "programmatic construct"


@dataclass
class CodeTerm:
    id: str
    document: str

    def anchor(self):
        return f"codeterm_{self.id.lower().replace(' ', '_')}"

    def search_name(self, env):
        return self.id


def collect_items_in_document(app, nodes):
    for node in nodes:
        yield CodeTerm(node.def_id, app.env.docname)


def replace_id_node(app, node, term):
    new = nodes.emphasis("", "")
    new["ids"].append(term.anchor())
    new += nodes.literal("", term.id)
    node.replace_self(new)


def create_ref_node(env, term, make_link):
    return make_link(
        term.document,
        term.anchor(),
        nodes.literal("", term.id),
    )

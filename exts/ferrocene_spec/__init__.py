from . import definitions, syntax_directive
from sphinx.domains import Domain
from sphinx.roles import XRefRole
import os


class SpecDomain(Domain):
    name = "spec"
    label = "Specification"
    roles = {
        "ref": XRefRole(),
        **definitions.get_roles(),
    }
    directives = {
        "syntax": syntax_directive.SyntaxDirective,
    }
    object_types = definitions.get_object_types()
    indices = {}

    def resolve_xref(self, env, fromdocname, builder, typ, target, node, contnode):
        return definitions.handle_ref(builder, env, fromdocname, target)

    def get_objects(self):
        return definitions.get_objects(self.env)


def setup(app):
    app.add_domain(SpecDomain)
    definitions.setup(app)

    # This works because `ext/ferrocene_spec/static` is added to the list of
    # HTML static paths in `conf.py`. Unfortunately there is no non-hackish way
    # to add a stylesheet from an extension.
    app.add_css_file("spec.css")

    return {
        "version": "0",
        "parallel_read_safe": True,
        "parallel_write_safe": True,
    }

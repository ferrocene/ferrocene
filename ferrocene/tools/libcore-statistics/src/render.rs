use rustdoc_types::{
    Abi, Crate, GenericArg, GenericArgs, GenericBound, GenericParamDef, GenericParamDefKind, Impl,
    Path, Term, TraitBoundModifier, Type, TypeBindingKind, WherePredicate,
};

pub(crate) fn render_impl(buf: &mut String, crate_: &Crate, impl_: &Impl) {
    if impl_.is_unsafe {
        buf.push_str("unsafe ");
    }
    buf.push_str("impl");
    if !impl_.generics.params.is_empty() {
        render_generic_params(buf, crate_, &impl_.generics.params);
    }
    buf.push(' ');
    if let Some(trait_) = &impl_.trait_ {
        if impl_.negative {
            buf.push('!');
        }
        render_path(buf, crate_, trait_);
        buf.push_str(" for ");
    }
    render_type(buf, crate_, &impl_.for_);

    if !impl_.generics.where_predicates.is_empty() {
        buf.push_str(" where ");
        let mut separator = Separator::new(", ");
        for predicate in &impl_.generics.where_predicates {
            separator.insert(buf);
            match predicate {
                WherePredicate::BoundPredicate {
                    type_,
                    bounds,
                    generic_params,
                } => {
                    if !generic_params.is_empty() {
                        buf.push_str("for");
                        render_generic_params(buf, crate_, generic_params);
                        buf.push(' ');
                    }
                    render_type(buf, crate_, type_);
                    buf.push_str(": ");
                    render_generic_bounds(buf, crate_, bounds);
                }
                WherePredicate::RegionPredicate { lifetime, bounds } => {
                    buf.push_str(lifetime);
                    buf.push_str(": ");
                    render_generic_bounds(buf, crate_, bounds);
                }
                WherePredicate::EqPredicate { lhs, rhs } => {
                    render_type(buf, crate_, lhs);
                    buf.push_str(" = ");
                    render_term(buf, crate_, rhs);
                }
            }
        }
    }
}

fn render_generic_params(buf: &mut String, crate_: &Crate, params: &[GenericParamDef]) {
    buf.push('<');
    let mut separator = Separator::new(", ");
    for param in params {
        if let GenericParamDefKind::Type {
            synthetic: true, ..
        } = &param.kind
        {
            continue;
        }
        separator.insert(buf);
        render_generic_param(buf, crate_, param);
    }
    buf.push('>');
}

fn render_generic_param(buf: &mut String, crate_: &Crate, param: &GenericParamDef) {
    match &param.kind {
        GenericParamDefKind::Lifetime { outlives } => {
            buf.push_str(&param.name);
            if !outlives.is_empty() {
                buf.push_str(": ");
                buf.push_str(&outlives.join(" + "));
            }
        }
        GenericParamDefKind::Type {
            bounds,
            default,
            synthetic: _,
        } => {
            buf.push_str(&param.name);
            if !bounds.is_empty() {
                buf.push_str(": ");
                render_generic_bounds(buf, crate_, bounds);
            }
            if let Some(default) = default {
                buf.push_str(" = ");
                render_type(buf, crate_, default);
            }
        }
        GenericParamDefKind::Const { type_, default } => {
            buf.push_str("const ");
            buf.push_str(&param.name);
            buf.push_str(": ");
            render_type(buf, crate_, type_);
            if let Some(default) = default {
                buf.push_str(" = ");
                buf.push_str(&default);
            }
        }
    }
}

fn render_generic_bounds(buf: &mut String, crate_: &Crate, bounds: &[GenericBound]) {
    let mut separator = Separator::new(" + ");
    for bound in bounds {
        separator.insert(buf);
        match bound {
            GenericBound::TraitBound {
                trait_,
                generic_params: bound_generic_params,
                modifier,
            } => {
                if !bound_generic_params.is_empty() {
                    buf.push_str("for");
                    render_generic_params(buf, crate_, bound_generic_params);
                    buf.push(' ');
                }
                match modifier {
                    TraitBoundModifier::None | TraitBoundModifier::MaybeConst => {}
                    TraitBoundModifier::Maybe => buf.push('?'),
                }
                render_path(buf, crate_, trait_);
            }
            GenericBound::Outlives(lifetime) => buf.push_str(&lifetime),
        }
    }
}

fn render_path(buf: &mut String, crate_: &Crate, path: &Path) {
    if path.name.is_empty() {
        let item = crate_.index.get(&path.id).unwrap();
        if let Some(name) = &item.name {
            buf.push_str(name);
        }
    } else {
        buf.push_str(&path.name);
    }
    if let Some(args) = &path.args {
        render_generic_args(buf, crate_, &**args)
    }
}

fn render_generic_args(buf: &mut String, crate_: &Crate, args: &GenericArgs) {
    match args {
        GenericArgs::AngleBracketed { args, bindings } => {
            if args.is_empty() && bindings.is_empty() {
                return;
            }

            buf.push('<');
            let mut separator = Separator::new(", ");
            for arg in args {
                separator.insert(buf);
                match arg {
                    GenericArg::Lifetime(lifetime) => buf.push_str(lifetime),
                    GenericArg::Type(type_) => render_type(buf, crate_, type_),
                    GenericArg::Const(constant) => buf.push_str(&constant.expr),
                    GenericArg::Infer => buf.push('_'),
                }
            }
            for binding in bindings {
                separator.insert(buf);
                buf.push_str(&binding.name);
                render_generic_args(buf, crate_, &binding.args);
                match &binding.binding {
                    TypeBindingKind::Equality(term) => {
                        buf.push_str(" = ");
                        render_term(buf, crate_, term);
                    }
                    TypeBindingKind::Constraint(bounds) => {
                        buf.push_str(": ");
                        render_generic_bounds(buf, crate_, bounds);
                    }
                }
            }
            buf.push('>');
        }
        GenericArgs::Parenthesized { inputs, output } => {
            buf.push('(');
            let mut separator = Separator::new(", ");
            for input in inputs {
                separator.insert(buf);
                render_type(buf, crate_, input);
            }
            buf.push(')');
            if let Some(output) = output {
                buf.push_str(" -> ");
                render_type(buf, crate_, output);
            }
        }
    }
}

fn render_term(buf: &mut String, crate_: &Crate, term: &Term) {
    match term {
        Term::Type(type_) => render_type(buf, crate_, type_),
        Term::Constant(constant) => buf.push_str(&constant.expr),
    }
}

fn render_type(buf: &mut String, crate_: &Crate, type_: &Type) {
    match type_ {
        Type::ResolvedPath(path) => render_path(buf, crate_, path),
        Type::Generic(generic) => buf.push_str(generic),
        Type::Primitive(primitive) => buf.push_str(primitive),

        Type::Slice(inner) => {
            buf.push('[');
            render_type(buf, crate_, inner);
            buf.push(']');
        }
        Type::Array { type_, len } => {
            buf.push('[');
            render_type(buf, crate_, type_);
            buf.push_str("; ");
            buf.push_str(len);
            buf.push(']');
        }
        Type::ImplTrait(bounds) => {
            buf.push_str("impl ");
            render_generic_bounds(buf, crate_, bounds);
        }
        Type::Infer => buf.push('_'),
        Type::RawPointer { mutable, type_ } => {
            buf.push('*');
            if *mutable {
                buf.push_str("mut ");
            }
            render_type(buf, crate_, type_);
        }

        Type::BorrowedRef {
            lifetime,
            mutable,
            type_,
        } => {
            buf.push('&');
            if let Some(lifetime) = lifetime {
                buf.push_str(lifetime);
                buf.push(' ');
            }
            if *mutable {
                buf.push_str("mut ");
            }
            render_type(buf, crate_, type_);
        }

        Type::QualifiedPath {
            name,
            args,
            self_type,
            trait_,
        } => {
            buf.push_str("<");
            render_type(buf, crate_, &self_type);
            render_generic_args(buf, crate_, &**args);
            buf.push_str(" as ");
            render_path(buf, crate_, trait_);
            buf.push_str(">::");
            buf.push_str(name);
        }

        Type::DynTrait(dyn_trait) => {
            buf.push_str("dyn ");
            let mut separator = Separator::new(" + ");
            for trait_ in &dyn_trait.traits {
                separator.insert(buf);
                if !trait_.generic_params.is_empty() {
                    buf.push_str("for");
                    render_generic_params(buf, crate_, &trait_.generic_params);
                    buf.push(' ');
                }
                render_path(buf, crate_, &trait_.trait_);
            }
            if let Some(lifetime) = &dyn_trait.lifetime {
                buf.push_str(" + ");
                buf.push_str(lifetime);
            }
        }

        Type::Tuple(members) => {
            buf.push('(');
            let mut separator = Separator::new(", ");
            for member in members {
                separator.insert(buf);
                render_type(buf, crate_, member);
            }
            buf.push(')');
        }

        Type::FunctionPointer(function) => {
            if !function.generic_params.is_empty() {
                buf.push_str("for");
                render_generic_params(buf, crate_, &function.generic_params);
                buf.push(' ');
            }
            if function.header.const_ {
                buf.push_str("const ");
            }
            if function.header.unsafe_ {
                buf.push_str("unsafe ");
            }
            if function.header.async_ {
                buf.push_str("async ");
            }
            let abi = match &function.header.abi {
                Abi::Rust => None,
                Abi::C { unwind } => Some(("C", unwind)),
                Abi::Cdecl { unwind } => Some(("cdecl", unwind)),
                Abi::Stdcall { unwind } => Some(("stdcall", unwind)),
                Abi::Fastcall { unwind } => Some(("fastcall", unwind)),
                Abi::Aapcs { unwind } => Some(("aapcs", unwind)),
                Abi::Win64 { unwind } => Some(("win64", unwind)),
                Abi::SysV64 { unwind } => Some(("sysv64", unwind)),
                Abi::System { unwind } => Some(("system", unwind)),
                Abi::Other(other) => Some((other.as_str(), &false)),
            };
            if let Some((extern_, unwind)) = abi {
                buf.push_str("extern \"");
                buf.push_str(extern_);
                if *unwind {
                    buf.push_str("-unwind");
                }
                buf.push_str("\" ");
            }

            buf.push_str("fn(");
            let mut separator = Separator::new(", ");
            for input in &function.decl.inputs {
                separator.insert(buf);
                buf.push_str(&input.0);
                buf.push_str(": ");
                render_type(buf, crate_, &input.1);
            }
            if function.decl.c_variadic {
                separator.insert(buf);
                buf.push_str("..");
            }
            buf.push_str(")");
            if let Some(output) = &function.decl.output {
                buf.push_str(" -> ");
                render_type(buf, crate_, output);
            }
        }
    }
}

struct Separator {
    separator: &'static str,
    first: bool,
}

impl Separator {
    fn new(separator: &'static str) -> Self {
        Self {
            separator,
            first: true,
        }
    }

    fn insert(&mut self, buf: &mut String) {
        if self.first {
            self.first = false;
        } else {
            buf.push_str(self.separator);
        }
    }
}

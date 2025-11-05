//! A formatter for types from `rustdoc_types`, used to convert
//! `rustdoc_types` types to strings for use in handlebars templates.
use std::fmt::Write;

/// A formatter to format `rustdoc_types` types to valid rust code
pub trait FormatRustdoc {
    fn fmt<W: Write>(&self, w: &mut W) -> std::fmt::Result;
    fn format_rustdoc(&self) -> std::result::Result<String, std::fmt::Error> {
        let mut buf = String::new();
        self.fmt(&mut buf)?;
        Ok(buf)
    }
}

impl FormatRustdoc for rustdoc_types::Constant {
    fn fmt<W: Write>(&self, w: &mut W) -> std::fmt::Result {
        write!(w, "{}", self.expr)
    }
}

impl FormatRustdoc for Vec<rustdoc_types::Type> {
    fn fmt<W: Write>(&self, w: &mut W) -> std::fmt::Result {
        let mut iter = self.iter();
        if let Some(type_) = iter.next() {
            type_.fmt(w)?;
            for type_ in iter {
                write!(w, ", ")?;
                type_.fmt(w)?;
            }
        }
        Ok(())
    }
}

impl FormatRustdoc for rustdoc_types::Type {
    fn fmt<W: Write>(&self, w: &mut W) -> std::fmt::Result {
        match self {
            Self::ResolvedPath(path) => path.fmt(w),
            Self::DynTrait(dyn_trait) => dyn_trait.fmt(w),
            Self::Generic(generic) => write!(w, "{generic}"),
            Self::Primitive(primitive) => write!(w, "{primitive}"),
            Self::FunctionPointer(function_pointer) => function_pointer.fmt(w),
            Self::Tuple(tuple) => {
                write!(w, "(")?;
                tuple.fmt(w)?;
                write!(w, ")")
            }
            Self::Slice(slice) => {
                write!(w, "[")?;
                slice.fmt(w)?;
                write!(w, "]")
            }
            Self::Array { type_, len } => {
                write!(w, "[")?;
                type_.fmt(w)?;
                write!(w, "; {len}]")
            }
            Self::Pat { .. } => {
                todo!("rustdoc_types::Type::Pat is not yet supported by FormatRustdoc")
            }
            Self::ImplTrait(bounds) => {
                write!(w, "impl ")?;
                bounds.fmt(w)
            }
            Self::Infer => write!(w, "_"),
            Self::RawPointer { is_mutable, type_ } => {
                if *is_mutable {
                    write!(w, "*mut ")?;
                } else {
                    write!(w, "*const ")?;
                }
                type_.fmt(w)
            }
            Self::BorrowedRef {
                lifetime,
                is_mutable,
                type_,
            } => {
                if let Some(lifetime) = lifetime {
                    if *is_mutable {
                        write!(w, "&{lifetime} mut ")?;
                    } else {
                        write!(w, "&{lifetime} ")?;
                    }
                } else if *is_mutable {
                    write!(w, "&mut ")?;
                } else {
                    write!(w, "& ")?;
                }

                type_.fmt(w)
            }
            Self::QualifiedPath {
                name,
                args,
                self_type,
                trait_,
            } => {
                if trait_.is_some() {
                    write!(w, "<")?;
                }
                self_type.fmt(w)?;
                if let Some(trait_) = trait_ {
                    trait_.fmt(w)?;
                    write!(w, ">")?;
                }
                write!(w, "::{name}")?;
                if let Some(args) = args {
                    args.fmt(w)?;
                }
                Ok(())
            }
        }
    }
}

impl FormatRustdoc for rustdoc_types::Path {
    fn fmt<W: Write>(&self, w: &mut W) -> std::fmt::Result {
        write!(w, "{}", self.path)?;
        if let Some(args) = &self.args {
            args.fmt(w)
        } else {
            Ok(())
        }
    }
}

impl FormatRustdoc for rustdoc_types::DynTrait {
    fn fmt<W: Write>(&self, w: &mut W) -> std::fmt::Result {
        write!(w, "dyn ")?;
        self.traits.fmt(w)?;
        if let Some(lifetime) = &self.lifetime {
            write!(w, " + {lifetime}")?;
        }
        Ok(())
    }
}

impl FormatRustdoc for rustdoc_types::FunctionPointer {
    fn fmt<W: Write>(&self, w: &mut W) -> std::fmt::Result {
        self.header.fmt(w)?;
        if !self.generic_params.is_empty() {
            write!(w, "for<")?;
            self.generic_params.fmt(w)?;
            write!(w, "> ")?;
        }
        self.sig.fmt(w)
    }
}

impl FormatRustdoc for rustdoc_types::GenericArgs {
    fn fmt<W: Write>(&self, w: &mut W) -> std::fmt::Result {
        match self {
            Self::AngleBracketed { args, constraints } => {
                write!(w, "<")?;
                args.fmt(w)?;
                if !args.is_empty() && !constraints.is_empty() {
                    write!(w, ", ")?;
                }
                constraints.fmt(w)?;
                write!(w, ">")
            }
            Self::Parenthesized { inputs, output } => {
                write!(w, "(")?;
                inputs.fmt(w)?;
                write!(w, ")")?;
                if let Some(output) = output {
                    write!(w, " -> ")?;
                    output.fmt(w)?;
                }
                Ok(())
            }
            Self::ReturnTypeNotation => todo!(
                "rustdoc_types::GenericArgs::ReturnTypeNotation is not yet supported by FormatRustdoc"
            ),
        }
    }
}

impl FormatRustdoc for Vec<rustdoc_types::GenericArg> {
    fn fmt<W: Write>(&self, w: &mut W) -> std::fmt::Result {
        let mut iter = self.iter();
        if let Some(arg) = iter.next() {
            arg.fmt(w)?;
            for arg in iter {
                write!(w, ", ")?;
                arg.fmt(w)?;
            }
        }
        Ok(())
    }
}

impl FormatRustdoc for rustdoc_types::GenericArg {
    fn fmt<W: Write>(&self, w: &mut W) -> std::fmt::Result {
        match self {
            Self::Lifetime(lifetime) => write!(w, "{}", lifetime),
            Self::Type(arg_type) => arg_type.fmt(w),
            Self::Const(constant) => constant.fmt(w),
            Self::Infer => write!(w, "_"),
        }
    }
}

impl FormatRustdoc for Vec<rustdoc_types::AssocItemConstraint> {
    fn fmt<W: Write>(&self, w: &mut W) -> std::fmt::Result {
        let mut iter = self.iter();
        if let Some(constraint) = iter.next() {
            constraint.fmt(w)?;
            for constraint in iter {
                write!(w, ", ")?;
                constraint.fmt(w)?;
            }
        }
        Ok(())
    }
}

impl FormatRustdoc for rustdoc_types::AssocItemConstraint {
    fn fmt<W: Write>(&self, w: &mut W) -> std::fmt::Result {
        write!(w, "{}", self.name)?;
        if let Some(args) = &self.args {
            args.fmt(w)?;
        }
        match &self.binding {
            rustdoc_types::AssocItemConstraintKind::Equality(term) => {
                write!(w, " = ")?;
                term.fmt(w)
            }
            rustdoc_types::AssocItemConstraintKind::Constraint(bounds) => {
                write!(w, ": ")?;
                bounds.fmt(w)
            }
        }
    }
}

impl FormatRustdoc for rustdoc_types::Term {
    fn fmt<W: Write>(&self, w: &mut W) -> std::fmt::Result {
        match self {
            Self::Type(term_type) => term_type.fmt(w),
            Self::Constant(constant) => constant.fmt(w),
        }
    }
}

impl FormatRustdoc for Vec<rustdoc_types::GenericBound> {
    fn fmt<W: Write>(&self, w: &mut W) -> std::fmt::Result {
        let mut iter = self.iter();
        if let Some(bound) = iter.next() {
            bound.fmt(w)?;
            for bound in iter {
                write!(w, " + ")?;
                bound.fmt(w)?;
            }
        }
        Ok(())
    }
}

impl FormatRustdoc for rustdoc_types::GenericBound {
    fn fmt<W: Write>(&self, w: &mut W) -> std::fmt::Result {
        match self {
            Self::TraitBound {
                trait_,
                generic_params,
                modifier,
            } => {
                if !generic_params.is_empty() {
                    write!(w, "for<")?;
                    generic_params.fmt(w)?;
                    write!(w, "> ")?;
                }
                trait_.fmt(w)?;
                match modifier {
                    rustdoc_types::TraitBoundModifier::None => (),
                    rustdoc_types::TraitBoundModifier::Maybe => write!(w, "?")?,
                    rustdoc_types::TraitBoundModifier::MaybeConst => todo!(
                        "rustdoc_types::TraitBoundModifier::MaybeConst is not yet supported by FormatRustdoc"
                    ),
                }
                Ok(())
            }
            Self::Outlives(lifetime) => write!(w, "{lifetime}"),
            Self::Use(precise_args) => {
                write!(w, "use<")?;
                precise_args.fmt(w)?;
                write!(w, ">")
            }
        }
    }
}

impl FormatRustdoc for Vec<rustdoc_types::GenericParamDef> {
    fn fmt<W: Write>(&self, w: &mut W) -> std::fmt::Result {
        let mut iter = self.iter();
        if let Some(param_def) = iter.next() {
            param_def.fmt(w)?;
            for param_def in iter {
                write!(w, " + ")?;
                param_def.fmt(w)?;
            }
        }
        Ok(())
    }
}

impl FormatRustdoc for rustdoc_types::GenericParamDef {
    fn fmt<W: Write>(&self, w: &mut W) -> std::fmt::Result {
        write!(w, "{}", self.name)?;
        match &self.kind {
            rustdoc_types::GenericParamDefKind::Lifetime { outlives } => {
                let mut iter = outlives.iter();
                if let Some(outlives) = iter.next() {
                    write!(w, ": {outlives}")?;
                    for outlives in iter {
                        write!(w, ", {outlives}")?;
                    }
                }
                Ok(())
            }
            rustdoc_types::GenericParamDefKind::Type {
                bounds,
                default,
                is_synthetic,
            } => {
                if *is_synthetic {
                    todo!(
                        "Synthetic rustdoc::GenericParamDefKind::Type is not yet supported by FormatRustdoc"
                    );
                } else {
                    if !bounds.is_empty() {
                        write!(w, ": ")?;
                        bounds.fmt(w)?;
                        if let Some(default_type) = default {
                            write!(w, " = ")?;
                            default_type.fmt(w)?;
                        }
                    }
                    Ok(())
                }
            }

            rustdoc_types::GenericParamDefKind::Const { type_, default } => {
                write!(w, "const ")?;
                type_.fmt(w)?;
                if let Some(default_val) = default {
                    write!(w, " = {default_val}")?;
                }
                Ok(())
            }
        }
    }
}

impl FormatRustdoc for Vec<rustdoc_types::PreciseCapturingArg> {
    fn fmt<W: Write>(&self, w: &mut W) -> std::fmt::Result {
        let mut iter = self.iter();
        if let Some(arg) = iter.next() {
            arg.fmt(w)?;
            for arg in iter {
                write!(w, ", ")?;
                arg.fmt(w)?;
            }
        }
        Ok(())
    }
}

impl FormatRustdoc for rustdoc_types::PreciseCapturingArg {
    fn fmt<W: Write>(&self, w: &mut W) -> std::fmt::Result {
        match self {
            Self::Lifetime(lifetime) => write!(w, "{lifetime}"),
            Self::Param(param) => write!(w, "{param}"),
        }
    }
}

impl FormatRustdoc for Vec<rustdoc_types::PolyTrait> {
    fn fmt<W: Write>(&self, w: &mut W) -> std::fmt::Result {
        let mut iter = self.iter();
        if let Some(poly) = iter.next() {
            poly.fmt(w)?;
            for poly in iter {
                write!(w, ", ")?;
                poly.fmt(w)?;
            }
        }
        Ok(())
    }
}

impl FormatRustdoc for rustdoc_types::PolyTrait {
    fn fmt<W: Write>(&self, w: &mut W) -> std::fmt::Result {
        if !self.generic_params.is_empty() {
            write!(w, "for<")?;
            self.generic_params.fmt(w)?;
            write!(w, "> ")?;
        }
        self.trait_.fmt(w)
    }
}

impl FormatRustdoc for rustdoc_types::FunctionHeader {
    fn fmt<W: Write>(&self, w: &mut W) -> std::fmt::Result {
        if self.is_const {
            write!(w, "const ")?;
        }
        if self.is_unsafe {
            write!(w, "unsafe ")?;
        }
        if self.is_async {
            write!(w, "async ")?;
        }
        if self.abi != rustdoc_types::Abi::Rust {
            todo!(
                "Non-Rust ABI for rustdoc_types::FunctionPointer is not yet supported by FormatRustdoc"
            );
        }
        Ok(())
    }
}

impl FormatRustdoc for rustdoc_types::FunctionSignature {
    fn fmt<W: Write>(&self, w: &mut W) -> std::fmt::Result {
        write!(w, "(")?;
        let mut iter = self.inputs.iter();
        if let Some((name, type_)) = iter.next() {
            write!(w, "{name}: ")?;
            type_.fmt(w)?;
            for (name, type_) in iter {
                write!(w, ", {name}: ")?;
                type_.fmt(w)?;
            }
        }
        if self.is_c_variadic {
            if !self.inputs.is_empty() {
                write!(w, ", ")?;
            }
            write!(w, "...")?;
        }
        write!(w, ")")?;
        if let Some(output) = &self.output {
            write!(w, " -> ")?;
            output.fmt(w)?;
        }
        Ok(())
    }
}

use std::{collections::BTreeSet, sync::Arc};

use rustdoc_types::{
    Crate, Enum, Function, Id, Impl, Item, ItemEnum, Method, Span, Struct, Type, Variant,
};
use trustfall_core::{
    interpreter::{Adapter, DataContext, InterpretedQuery},
    ir::{EdgeParameters, Eid, FieldValue, Vid},
    schema::Schema,
};

pub struct RustdocAdapter<'a> {
    current_crate: &'a Crate,
    previous_crate: Option<&'a Crate>,
}

impl<'a> RustdocAdapter<'a> {
    pub fn new(current_crate: &'a Crate, previous_crate: Option<&'a Crate>) -> Self {
        Self {
            current_crate,
            previous_crate,
        }
    }

    pub fn schema() -> Schema {
        Schema::parse(include_str!("rustdoc_schema.graphql")).expect("schema not valid")
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Origin {
    CurrentCrate,
    PreviousCrate,
}

impl Origin {
    fn make_item_token<'a>(&self, item: &'a Item) -> Token<'a> {
        Token {
            origin: *self,
            kind: item.into(),
        }
    }

    fn make_span_token<'a>(&self, span: &'a Span) -> Token<'a> {
        Token {
            origin: *self,
            kind: span.into(),
        }
    }

    fn make_path_token<'a>(&self, path: &'a [String]) -> Token<'a> {
        Token {
            origin: *self,
            kind: TokenKind::Path(path),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token<'a> {
    origin: Origin,
    kind: TokenKind<'a>,
}

impl<'a> Token<'a> {
    fn new_crate(origin: Origin, crate_: &'a Crate) -> Self {
        Self {
            origin,
            kind: TokenKind::Crate(crate_),
        }
    }
}

#[derive(Debug, Clone)]
pub enum TokenKind<'a> {
    CrateDiff((&'a Crate, &'a Crate)),
    Crate(&'a Crate),
    Item(&'a Item),
    Span(&'a Span),
    Path(&'a [String]),
}

#[allow(dead_code)]
impl<'a> Token<'a> {
    /// The name of the actual runtime type of this token,
    /// intended to fulfill resolution requests for the __typename property.
    #[inline]
    fn typename(&self) -> &'static str {
        match self.kind {
            TokenKind::Item(item) => match &item.inner {
                rustdoc_types::ItemEnum::Struct(..) => "Struct",
                rustdoc_types::ItemEnum::Enum(..) => "Enum",
                rustdoc_types::ItemEnum::Function(..) => "Function",
                rustdoc_types::ItemEnum::Method(..) => "Method",
                rustdoc_types::ItemEnum::Variant(Variant::Plain) => "PlainVariant",
                rustdoc_types::ItemEnum::Variant(Variant::Tuple(..)) => "TupleVariant",
                rustdoc_types::ItemEnum::Variant(Variant::Struct(..)) => "StructVariant",
                rustdoc_types::ItemEnum::StructField(..) => "StructField",
                rustdoc_types::ItemEnum::Impl(..) => "Impl",
                _ => unreachable!("unexpected item.inner for item: {item:?}"),
            },
            TokenKind::Span(..) => "Span",
            TokenKind::Path(..) => "Path",
            TokenKind::Crate(..) => "Crate",
            TokenKind::CrateDiff(..) => "CrateDiff",
        }
    }

    fn as_crate_diff(&self) -> Option<(&'a Crate, &'a Crate)> {
        match &self.kind {
            TokenKind::CrateDiff(tuple) => Some(*tuple),
            _ => None,
        }
    }

    fn as_crate(&self) -> Option<&'a Crate> {
        match self.kind {
            TokenKind::Crate(c) => Some(c),
            _ => None,
        }
    }

    fn as_item(&self) -> Option<&'a Item> {
        match self.kind {
            TokenKind::Item(item) => Some(item),
            _ => None,
        }
    }

    fn as_struct_item(&self) -> Option<(&'a Item, &'a Struct)> {
        self.as_item().and_then(|item| match &item.inner {
            rustdoc_types::ItemEnum::Struct(s) => Some((item, s)),
            _ => None,
        })
    }

    fn as_struct_field_item(&self) -> Option<(&'a Item, &'a Type)> {
        self.as_item().and_then(|item| match &item.inner {
            rustdoc_types::ItemEnum::StructField(s) => Some((item, s)),
            _ => None,
        })
    }

    fn as_span(&self) -> Option<&'a Span> {
        match self.kind {
            TokenKind::Span(s) => Some(s),
            _ => None,
        }
    }

    fn as_enum(&self) -> Option<&'a Enum> {
        self.as_item().and_then(|item| match &item.inner {
            rustdoc_types::ItemEnum::Enum(e) => Some(e),
            _ => None,
        })
    }

    fn as_variant(&self) -> Option<&'a Variant> {
        self.as_item().and_then(|item| match &item.inner {
            rustdoc_types::ItemEnum::Variant(v) => Some(v),
            _ => None,
        })
    }

    fn as_path(&self) -> Option<&'a [String]> {
        match &self.kind {
            TokenKind::Path(path) => Some(*path),
            _ => None,
        }
    }

    fn as_function(&self) -> Option<&'a Function> {
        self.as_item().and_then(|item| match &item.inner {
            rustdoc_types::ItemEnum::Function(func) => Some(func),
            _ => None,
        })
    }

    fn as_method(&self) -> Option<&'a Method> {
        self.as_item().and_then(|item| match &item.inner {
            rustdoc_types::ItemEnum::Method(func) => Some(func),
            _ => None,
        })
    }

    fn as_impl(&self) -> Option<&'a Impl> {
        self.as_item().and_then(|item| match &item.inner {
            rustdoc_types::ItemEnum::Impl(x) => Some(x),
            _ => None,
        })
    }
}

impl<'a> From<&'a Item> for TokenKind<'a> {
    fn from(item: &'a Item) -> Self {
        Self::Item(item)
    }
}

impl<'a> From<&'a Crate> for TokenKind<'a> {
    fn from(c: &'a Crate) -> Self {
        Self::Crate(c)
    }
}

impl<'a> From<&'a Span> for TokenKind<'a> {
    fn from(s: &'a Span) -> Self {
        Self::Span(s)
    }
}

fn get_crate_property(crate_token: &Token, field_name: &str) -> FieldValue {
    let crate_item = crate_token.as_crate().expect("token was not a Crate");
    match field_name {
        "root" => (&crate_item.root.0).into(),
        "crate_version" => (&crate_item.crate_version).into(),
        "includes_private" => crate_item.includes_private.into(),
        "format_version" => crate_item.format_version.into(),
        _ => unreachable!("Crate property {field_name}"),
    }
}

fn get_item_property(item_token: &Token, field_name: &str) -> FieldValue {
    let item = item_token.as_item().expect("token was not an Item");
    match field_name {
        "id" => (&item.id.0).into(),
        "crate_id" => (&item.crate_id).into(),
        "name" => (&item.name).into(),
        "docs" => (&item.docs).into(),
        "attrs" => item.attrs.clone().into(),
        "visibility_limit" => match &item.visibility {
            rustdoc_types::Visibility::Public => "public".into(),
            rustdoc_types::Visibility::Default => "default".into(),
            rustdoc_types::Visibility::Crate => "crate".into(),
            rustdoc_types::Visibility::Restricted { parent: _, path } => {
                format!("restricted ({path})").into()
            }
        },
        _ => unreachable!("Item property {field_name}"),
    }
}

fn get_struct_property(item_token: &Token, field_name: &str) -> FieldValue {
    let (_, struct_item) = item_token.as_struct_item().expect("token was not a Struct");
    match field_name {
        "struct_type" => match struct_item.struct_type {
            rustdoc_types::StructType::Plain => "plain",
            rustdoc_types::StructType::Tuple => "tuple",
            rustdoc_types::StructType::Unit => "unit",
        }
        .into(),
        "fields_stripped" => struct_item.fields_stripped.into(),
        _ => unreachable!("Struct property {field_name}"),
    }
}

fn get_span_property(item_token: &Token, field_name: &str) -> FieldValue {
    let span = item_token.as_span().expect("token was not a Span");
    match field_name {
        "filename" => span
            .filename
            .to_str()
            .expect("non-representable path")
            .into(),
        "begin_line" => (span.begin.0 as u64).into(),
        "begin_column" => (span.begin.1 as u64).into(),
        "end_line" => (span.end.0 as u64).into(),
        "end_column" => (span.end.1 as u64).into(),
        _ => unreachable!("Span property {field_name}"),
    }
}

fn get_enum_property(item_token: &Token, field_name: &str) -> FieldValue {
    let enum_item = item_token.as_enum().expect("token was not an Enum");
    match field_name {
        "variants_stripped" => enum_item.variants_stripped.into(),
        _ => unreachable!("Enum property {field_name}"),
    }
}

fn get_path_property(token: &Token, field_name: &str) -> FieldValue {
    let path_token = token.as_path().expect("token was not a Path");
    match field_name {
        "path" => path_token.into(),
        _ => unreachable!("Path property {field_name}"),
    }
}

fn get_function_like_property(token: &Token, field_name: &str) -> FieldValue {
    let maybe_function = token.as_function();
    let maybe_method = token.as_method();

    let (header, _decl) = maybe_function
        .map(|func| (&func.header, &func.decl))
        .unwrap_or_else(|| {
            let method = maybe_method.unwrap_or_else(|| {
                unreachable!("token was neither a function nor a method: {token:?}")
            });
            (&method.header, &method.decl)
        });

    match field_name {
        "const" => header.const_.into(),
        "async" => header.async_.into(),
        "unsafe" => header.unsafe_.into(),
        _ => unreachable!("FunctionLike property {field_name}"),
    }
}

fn get_impl_property(token: &Token, field_name: &str) -> FieldValue {
    let impl_token = token.as_impl().expect("token was not an Impl");
    match field_name {
        "unsafe" => impl_token.is_unsafe.into(),
        "negative" => impl_token.negative.into(),
        "synthetic" => impl_token.synthetic.into(),
        _ => unreachable!("Impl property {field_name}"),
    }
}

fn property_mapper<'a>(
    ctx: DataContext<Token<'a>>,
    field_name: &str,
    property_getter: fn(&Token<'a>, &str) -> FieldValue,
) -> (DataContext<Token<'a>>, FieldValue) {
    let value = match &ctx.current_token {
        Some(token) => property_getter(token, field_name),
        None => FieldValue::Null,
    };
    (ctx, value)
}

impl<'a> Adapter<'a> for RustdocAdapter<'a> {
    type DataToken = Token<'a>;

    fn get_starting_tokens(
        &mut self,
        edge: Arc<str>,
        _parameters: Option<Arc<EdgeParameters>>,
        _query_hint: InterpretedQuery,
        _vertex_hint: Vid,
    ) -> Box<dyn Iterator<Item = Self::DataToken> + 'a> {
        match edge.as_ref() {
            "Crate" => Box::new(std::iter::once(Token::new_crate(
                Origin::CurrentCrate,
                self.current_crate,
            ))),
            "CrateDiff" => {
                let previous_crate = self.previous_crate.expect("no previous crate provided");
                Box::new(std::iter::once(Token {
                    origin: Origin::CurrentCrate,
                    kind: TokenKind::CrateDiff((self.current_crate, previous_crate)),
                }))
            }
            _ => unreachable!("{edge}"),
        }
    }

    fn project_property(
        &mut self,
        data_contexts: Box<dyn Iterator<Item = DataContext<Self::DataToken>> + 'a>,
        current_type_name: Arc<str>,
        field_name: Arc<str>,
        _query_hint: InterpretedQuery,
        _vertex_hint: Vid,
    ) -> Box<dyn Iterator<Item = (DataContext<Self::DataToken>, FieldValue)> + 'a> {
        if field_name.as_ref() == "__typename" {
            Box::new(data_contexts.map(|ctx| match &ctx.current_token {
                Some(token) => {
                    let value = token.typename().into();
                    (ctx, value)
                }
                None => (ctx, FieldValue::Null),
            }))
        } else {
            match current_type_name.as_ref() {
                "Crate" => {
                    Box::new(data_contexts.map(move |ctx| {
                        property_mapper(ctx, field_name.as_ref(), get_crate_property)
                    }))
                }
                "Item" => {
                    Box::new(data_contexts.map(move |ctx| {
                        property_mapper(ctx, field_name.as_ref(), get_item_property)
                    }))
                }
                "ImplOwner" | "Struct" | "StructField" | "Enum" | "Variant" | "PlainVariant"
                | "TupleVariant" | "StructVariant" | "Function" | "Method" | "Impl"
                    if matches!(
                        field_name.as_ref(),
                        "id" | "crate_id" | "name" | "docs" | "attrs" | "visibility_limit"
                    ) =>
                {
                    // properties inherited from Item, accesssed on Item subtypes
                    Box::new(data_contexts.map(move |ctx| {
                        property_mapper(ctx, field_name.as_ref(), get_item_property)
                    }))
                }
                "Struct" => Box::new(data_contexts.map(move |ctx| {
                    property_mapper(ctx, field_name.as_ref(), get_struct_property)
                })),
                "Enum" => {
                    Box::new(data_contexts.map(move |ctx| {
                        property_mapper(ctx, field_name.as_ref(), get_enum_property)
                    }))
                }
                "Span" => {
                    Box::new(data_contexts.map(move |ctx| {
                        property_mapper(ctx, field_name.as_ref(), get_span_property)
                    }))
                }
                "Path" => {
                    Box::new(data_contexts.map(move |ctx| {
                        property_mapper(ctx, field_name.as_ref(), get_path_property)
                    }))
                }
                "FunctionLike" | "Function" | "Method"
                    if matches!(field_name.as_ref(), "const" | "unsafe" | "async") =>
                {
                    Box::new(data_contexts.map(move |ctx| {
                        property_mapper(ctx, field_name.as_ref(), get_function_like_property)
                    }))
                }
                "Impl" => {
                    Box::new(data_contexts.map(move |ctx| {
                        property_mapper(ctx, field_name.as_ref(), get_impl_property)
                    }))
                }
                _ => unreachable!("project_property {current_type_name} {field_name}"),
            }
        }
    }

    fn project_neighbors(
        &mut self,
        data_contexts: Box<dyn Iterator<Item = DataContext<Self::DataToken>> + 'a>,
        current_type_name: Arc<str>,
        edge_name: Arc<str>,
        parameters: Option<Arc<EdgeParameters>>,
        _query_hint: InterpretedQuery,
        _vertex_hint: Vid,
        _edge_hint: Eid,
    ) -> Box<
        dyn Iterator<
                Item = (
                    DataContext<Self::DataToken>,
                    Box<dyn Iterator<Item = Self::DataToken> + 'a>,
                ),
            > + 'a,
    > {
        match current_type_name.as_ref() {
            "CrateDiff" => match edge_name.as_ref() {
                "current" => Box::new(data_contexts.map(move |ctx| {
                    let neighbors: Box<dyn Iterator<Item = Self::DataToken> + 'a> = match &ctx
                        .current_token
                    {
                        None => Box::new(std::iter::empty()),
                        Some(token) => {
                            let crate_tuple =
                                token.as_crate_diff().expect("token was not a CrateDiff");
                            let neighbor = Token::new_crate(Origin::CurrentCrate, crate_tuple.0);
                            Box::new(std::iter::once(neighbor))
                        }
                    };

                    (ctx, neighbors)
                })),
                "baseline" => Box::new(data_contexts.map(move |ctx| {
                    let neighbors: Box<dyn Iterator<Item = Self::DataToken> + 'a> = match &ctx
                        .current_token
                    {
                        None => Box::new(std::iter::empty()),
                        Some(token) => {
                            let crate_tuple =
                                token.as_crate_diff().expect("token was not a CrateDiff");
                            let neighbor = Token::new_crate(Origin::PreviousCrate, crate_tuple.1);
                            Box::new(std::iter::once(neighbor))
                        }
                    };

                    (ctx, neighbors)
                })),
                _ => {
                    unreachable!("project_neighbors {current_type_name} {edge_name} {parameters:?}")
                }
            },
            "Crate" => {
                match edge_name.as_ref() {
                    "item" => Box::new(data_contexts.map(move |ctx| {
                        let neighbors: Box<dyn Iterator<Item = Self::DataToken> + 'a> = match &ctx
                            .current_token
                        {
                            None => Box::new(std::iter::empty()),
                            Some(token) => {
                                let origin = token.origin;
                                let crate_token = token.as_crate().expect("token was not a Crate");
                                let iter = crate_token
                                    .index
                                    .values()
                                    .filter(|item| {
                                        // Filter out item types that are not currently supported.
                                        matches!(
                                            item.inner,
                                            rustdoc_types::ItemEnum::Struct(..)
                                                | rustdoc_types::ItemEnum::StructField(..)
                                                | rustdoc_types::ItemEnum::Enum(..)
                                                | rustdoc_types::ItemEnum::Variant(..)
                                                | rustdoc_types::ItemEnum::Function(..)
                                                | rustdoc_types::ItemEnum::Method(..)
                                                | rustdoc_types::ItemEnum::Impl(..)
                                        )
                                    })
                                    .map(move |value| origin.make_item_token(value));
                                Box::new(iter)
                            }
                        };

                        (ctx, neighbors)
                    })),
                    _ => unreachable!(
                        "project_neighbors {current_type_name} {edge_name} {parameters:?}"
                    ),
                }
            }
            "Importable" | "ImplOwner" | "Struct" | "Enum" | "Function"
                if edge_name.as_ref() == "path" =>
            {
                let current_crate = self.current_crate;
                let previous_crate = self.previous_crate;

                Box::new(data_contexts.map(move |ctx| {
                    let neighbors: Box<dyn Iterator<Item = Self::DataToken> + 'a> =
                        match &ctx.current_token {
                            None => Box::new(std::iter::empty()),
                            Some(token) => {
                                let origin = token.origin;
                                let item = token.as_item().expect("token was not an Item");
                                let item_id = &item.id;

                                if let Some(path) = match origin {
                                    Origin::CurrentCrate => {
                                        current_crate.paths.get(item_id).map(|x| &x.path)
                                    }
                                    Origin::PreviousCrate => previous_crate
                                        .expect("no baseline provided")
                                        .paths
                                        .get(item_id)
                                        .map(|x| &x.path),
                                } {
                                    Box::new(std::iter::once(origin.make_path_token(path)))
                                } else {
                                    Box::new(std::iter::empty())
                                }
                            }
                        };

                    (ctx, neighbors)
                }))
            }
            "Item" | "ImplOwner" | "Struct" | "StructField" | "Enum" | "Variant"
            | "PlainVariant" | "TupleVariant" | "StructVariant" | "Function" | "Method"
            | "Impl"
                if edge_name.as_ref() == "span" =>
            {
                Box::new(data_contexts.map(move |ctx| {
                    let neighbors: Box<dyn Iterator<Item = Self::DataToken> + 'a> =
                        match &ctx.current_token {
                            None => Box::new(std::iter::empty()),
                            Some(token) => {
                                let origin = token.origin;
                                let item = token.as_item().expect("token was not an Item");
                                if let Some(span) = &item.span {
                                    Box::new(std::iter::once(origin.make_span_token(span)))
                                } else {
                                    Box::new(std::iter::empty())
                                }
                            }
                        };

                    (ctx, neighbors)
                }))
            }
            "ImplOwner" | "Struct" | "Enum"
                if matches!(edge_name.as_ref(), "impl" | "inherent_impl") =>
            {
                let current_crate = self.current_crate;
                let previous_crate = self.previous_crate;
                let inherent_impls_only = edge_name.as_ref() == "inherent_impl";
                Box::new(data_contexts.map(move |ctx| {
                    let neighbors: Box<dyn Iterator<Item = Self::DataToken> + 'a> =
                        match &ctx.current_token {
                            None => Box::new(std::iter::empty()),
                            Some(token) => {
                                let origin = token.origin;
                                let item_index = match origin {
                                    Origin::CurrentCrate => &current_crate.index,
                                    Origin::PreviousCrate => {
                                        &previous_crate.expect("no previous crate provided").index
                                    }
                                };

                                // Get the IDs of all the impl blocks.
                                // Relies on the fact that only structs and enums can have impls,
                                // so we know that the token must represent either a struct or an enum.
                                let impl_ids = token
                                    .as_struct_item()
                                    .map(|(_, s)| &s.impls)
                                    .or_else(|| token.as_enum().map(|e| &e.impls))
                                    .expect("token was neither a struct nor an enum");

                                Box::new(impl_ids.iter().filter_map(move |item_id| {
                                    let next_item = item_index.get(item_id);
                                    next_item.and_then(|next_item| match &next_item.inner {
                                        rustdoc_types::ItemEnum::Impl(imp) => {
                                            if !inherent_impls_only || imp.trait_.is_none() {
                                                Some(origin.make_item_token(next_item))
                                            } else {
                                                None
                                            }
                                        }
                                        _ => None,
                                    })
                                }))
                            }
                        };

                    (ctx, neighbors)
                }))
            }
            "Struct" => match edge_name.as_ref() {
                "field" => {
                    let current_crate = self.current_crate;
                    let previous_crate = self.previous_crate;
                    Box::new(data_contexts.map(move |ctx| {
                        let neighbors: Box<dyn Iterator<Item = Self::DataToken> + 'a> = match &ctx
                            .current_token
                        {
                            None => Box::new(std::iter::empty()),
                            Some(token) => {
                                let origin = token.origin;
                                let (_, struct_item) =
                                    token.as_struct_item().expect("token was not a Struct");

                                let item_index = match origin {
                                    Origin::CurrentCrate => &current_crate.index,
                                    Origin::PreviousCrate => {
                                        &previous_crate.expect("no previous crate provided").index
                                    }
                                };
                                Box::new(struct_item.fields.clone().into_iter().map(
                                    move |field_id| {
                                        origin.make_item_token(
                                            item_index.get(&field_id).expect("missing item"),
                                        )
                                    },
                                ))
                            }
                        };

                        (ctx, neighbors)
                    }))
                }
                _ => {
                    unreachable!("project_neighbors {current_type_name} {edge_name} {parameters:?}")
                }
            },
            "Enum" => match edge_name.as_ref() {
                "variant" => {
                    let current_crate = self.current_crate;
                    let previous_crate = self.previous_crate;
                    Box::new(data_contexts.map(move |ctx| {
                        let neighbors: Box<dyn Iterator<Item = Self::DataToken> + 'a> = match &ctx
                            .current_token
                        {
                            None => Box::new(std::iter::empty()),
                            Some(token) => {
                                let origin = token.origin;
                                let enum_item = token.as_enum().expect("token was not a Enum");

                                let item_index = match origin {
                                    Origin::CurrentCrate => &current_crate.index,
                                    Origin::PreviousCrate => {
                                        &previous_crate.expect("no previous crate provided").index
                                    }
                                };
                                Box::new(enum_item.variants.iter().map(move |field_id| {
                                    origin.make_item_token(
                                        item_index.get(field_id).expect("missing item"),
                                    )
                                }))
                            }
                        };

                        (ctx, neighbors)
                    }))
                }
                _ => {
                    unreachable!("project_neighbors {current_type_name} {edge_name} {parameters:?}")
                }
            },
            "Impl" => match edge_name.as_ref() {
                "method" => {
                    let current_crate = self.current_crate;
                    let previous_crate = self.previous_crate;
                    Box::new(data_contexts.map(move |ctx| {
                        let neighbors: Box<dyn Iterator<Item = Self::DataToken> + 'a> = match &ctx
                            .current_token
                        {
                            None => Box::new(std::iter::empty()),
                            Some(token) => {
                                let origin = token.origin;
                                let item_index = match origin {
                                    Origin::CurrentCrate => &current_crate.index,
                                    Origin::PreviousCrate => {
                                        &previous_crate.expect("no previous crate provided").index
                                    }
                                };

                                let impl_token = token.as_impl().expect("not an Impl token");
                                let provided_methods: Box<dyn Iterator<Item = &Id>> = if impl_token.provided_trait_methods.is_empty() {
                                    Box::new(std::iter::empty())
                                } else {
                                    let method_names: BTreeSet<&str> = impl_token.provided_trait_methods.iter().map(|x| x.as_str()).collect();

                                    let trait_type = impl_token.trait_.as_ref().expect("no trait but provided_trait_methods was non-empty");
                                    let trait_item = match trait_type {
                                        Type::ResolvedPath { name: _, id, args: _, param_names: _ } => {
                                            item_index.get(id)
                                        }
                                        _ => unimplemented!("found provided_trait_methods when the trait was not a ResolvedPath: {trait_type:?}"),
                                    };

                                    if let Some(trait_item) = trait_item {
                                        if let ItemEnum::Trait(trait_item) = &trait_item.inner {
                                            Box::new(trait_item.items.iter().filter(move |item_id| {
                                                let next_item = &item_index.get(item_id);
                                                if let Some(name) = next_item.and_then(|x| x.name.as_deref()) {
                                                    method_names.contains(name)
                                                } else {
                                                    false
                                                }
                                            }))
                                        } else {
                                            unreachable!("found a non-trait type {trait_item:?}");
                                        }
                                    } else {
                                        Box::new(std::iter::empty())
                                    }
                                };
                                Box::new(provided_methods.chain(impl_token.items.iter()).filter_map(move |item_id| {
                                    let next_item = &item_index.get(item_id);
                                    if let Some(next_item) = next_item {
                                        match &next_item.inner {
                                            rustdoc_types::ItemEnum::Method(..) => {
                                                Some(origin.make_item_token(next_item))
                                            }
                                            _ => None,
                                        }
                                    } else {
                                        None
                                    }
                                }))
                            }
                        };

                        (ctx, neighbors)
                    }))
                }
                _ => {
                    unreachable!("project_neighbors {current_type_name} {edge_name} {parameters:?}")
                }
            },
            _ => unreachable!("project_neighbors {current_type_name} {edge_name} {parameters:?}"),
        }
    }

    fn can_coerce_to_type(
        &mut self,
        data_contexts: Box<dyn Iterator<Item = DataContext<Self::DataToken>> + 'a>,
        current_type_name: Arc<str>,
        coerce_to_type_name: Arc<str>,
        _query_hint: InterpretedQuery,
        _vertex_hint: Vid,
    ) -> Box<dyn Iterator<Item = (DataContext<Self::DataToken>, bool)> + 'a> {
        match current_type_name.as_ref() {
            "Item" | "Variant" | "FunctionLike" | "Importable" | "ImplOwner" => {
                Box::new(data_contexts.map(move |ctx| {
                    let can_coerce = match &ctx.current_token {
                        None => false,
                        Some(token) => {
                            let actual_type_name = token.typename();

                            match coerce_to_type_name.as_ref() {
                                "Variant" => matches!(
                                    actual_type_name,
                                    "PlainVariant" | "TupleVariant" | "StructVariant"
                                ),
                                "ImplOwner" => matches!(actual_type_name, "Struct" | "Enum"),
                                _ => {
                                    // The remaining types are final (don't have any subtypes)
                                    // so we can just compare the actual type name to
                                    // the type we are attempting to coerce to.
                                    actual_type_name == coerce_to_type_name.as_ref()
                                }
                            }
                        }
                    };

                    (ctx, can_coerce)
                }))
            }
            _ => unreachable!("can_coerce_to_type {current_type_name} {coerce_to_type_name}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, collections::BTreeMap, rc::Rc, sync::Arc};

    use anyhow::Context;
    use trustfall_core::{frontend::parse, interpreter::execution::interpret_ir, ir::FieldValue};

    use crate::{query::SemverQuery, util::load_rustdoc_from_file};

    use super::RustdocAdapter;

    fn check_query_execution(query_name: &str) {
        // Ensure the rustdocs JSON outputs have been regenerated.
        let baseline = load_rustdoc_from_file("./localdata/test_data/baseline.json")
            .with_context(|| "Could not load localdata/test_data/baseline.json file, did you forget to run ./scripts/regenerate_test_rustdocs.sh ?")
            .expect("failed to load baseline rustdoc");
        let current =
            load_rustdoc_from_file(&format!("./localdata/test_data/{}.json", query_name))
            .with_context(|| format!("Could not load localdata/test_data/{}.json file, did you forget to run ./scripts/regenerate_test_rustdocs.sh ?", query_name))
            .expect("failed to load rustdoc under test");

        let query_text =
            std::fs::read_to_string(&format!("./src/queries/{}.ron", query_name)).unwrap();
        let semver_query: SemverQuery = ron::from_str(&query_text).unwrap();

        let expected_result_text =
            std::fs::read_to_string(&format!("./src/test_data/{}.output.ron", query_name))
            .with_context(|| format!("Could not load src/test_data/{}.output.ron expected-outputs file, did you forget to add it?", query_name))
            .expect("failed to load expected outputs");
        let mut expected_results: Vec<BTreeMap<String, FieldValue>> =
            ron::from_str(&expected_result_text)
                .expect("could not parse expected outputs as ron format");

        let schema = RustdocAdapter::schema();
        let adapter = Rc::new(RefCell::new(RustdocAdapter::new(&current, Some(&baseline))));

        let parsed_query = parse(&schema, &semver_query.query).unwrap();
        let args = Arc::new(
            semver_query
                .arguments
                .iter()
                .map(|(k, v)| (Arc::from(k.clone()), v.clone().into()))
                .collect(),
        );
        let results_iter = interpret_ir(adapter.clone(), parsed_query, args).unwrap();

        let mut actual_results: Vec<BTreeMap<_, _>> = results_iter
            .map(|res| res.into_iter().map(|(k, v)| (k.to_string(), v)).collect())
            .collect();

        // Reorder both vectors of results into a deterministic order that will compensate for
        // nondeterminism in how the results are ordered.
        let key_func = |elem: &BTreeMap<String, FieldValue>| {
            (
                elem["span_filename"].as_str().unwrap().to_owned(),
                elem["span_begin_line"].as_usize().unwrap(),
            )
        };
        expected_results.sort_unstable_by_key(key_func);
        actual_results.sort_unstable_by_key(key_func);

        assert_eq!(expected_results, actual_results);
    }

    macro_rules! query_execution_tests {
        ($($name:ident,)*) => {
            $(
                #[test]
                fn $name() {
                    check_query_execution(stringify!($name))
                }
            )*
        }
    }

    query_execution_tests!(
        enum_missing,
        enum_repr_c_removed,
        enum_variant_added,
        enum_variant_missing,
        function_missing,
        inherent_method_missing,
        struct_marked_non_exhaustive,
        struct_missing,
        struct_pub_field_missing,
        struct_repr_c_removed,
        struct_repr_transparent_removed,
        unit_struct_changed_kind,
        variant_marked_non_exhaustive,
    );
}

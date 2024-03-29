use super::{ConstantDeclaration, FunctionDeclaration, TraitTypeDeclaration};
use crate::{
    decl_engine::parsed_id::ParsedDeclId, language::CallPath, type_system::TypeArgument,
    TypeParameter,
};

use sway_types::span::Span;

#[derive(Debug, Clone)]
pub enum ImplItem {
    Fn(ParsedDeclId<FunctionDeclaration>),
    Constant(ParsedDeclId<ConstantDeclaration>),
    Type(ParsedDeclId<TraitTypeDeclaration>),
}

#[derive(Debug, Clone)]
pub struct ImplTrait {
    pub impl_type_parameters: Vec<TypeParameter>,
    pub trait_name: CallPath,
    pub trait_type_arguments: Vec<TypeArgument>,
    pub implementing_for: TypeArgument,
    pub items: Vec<ImplItem>,
    // the span of the whole impl trait and block
    pub(crate) block_span: Span,
}

/// An impl of methods without a trait
/// like `impl MyType { fn foo { .. } }`
#[derive(Debug, Clone)]
pub struct ImplSelf {
    pub impl_type_parameters: Vec<TypeParameter>,
    pub implementing_for: TypeArgument,
    pub items: Vec<ImplItem>,
    // the span of the whole impl trait and block
    pub(crate) block_span: Span,
}

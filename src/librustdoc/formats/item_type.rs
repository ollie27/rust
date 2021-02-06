//! Item types.

use std::fmt;

use serde::{Serialize, Serializer};

use rustc_hir as hir;
use rustc_span::hygiene::MacroKind;

use crate::clean;

/// Item type. Corresponds to `clean::ItemEnum` variants.
///
/// The search index uses item types encoded as smaller numbers which equal to
/// discriminants. JavaScript then is used to decode them into the original value.
/// Consequently, every change to this type should be synchronized to
/// the `itemTypes` mapping table in `html/static/main.js`.
///
/// In addition, code in `html::render` uses this enum to generate CSS classes, page prefixes, and
/// module headings. If you are adding to this enum and want to ensure that the sidebar also prints
/// a heading, edit the listing in `html/render.rs`, function `sidebar_module`. This uses an
/// ordering based on a helper function inside `item_module`, in the same file.
#[derive(Copy, PartialEq, Eq, Clone, Debug, PartialOrd, Ord)]
crate enum ItemType {
    Module = 0,
    ExternCrate = 1,
    Import = 2,
    Struct = 3,
    Enum = 4,
    Function = 5,
    Typedef = 6,
    Static = 7,
    Trait = 8,
    Impl = 9,
    TyMethod = 10,
    Method = 11,
    StructField = 12,
    Variant = 13,
    Macro = 14,
    Primitive = 15,
    AssocType = 16,
    Constant = 17,
    AssocConst = 18,
    Union = 19,
    ForeignType = 20,
    Keyword = 21,
    OpaqueTy = 22,
    ProcAttribute = 23,
    ProcDerive = 24,
    TraitAlias = 25,
}

impl Serialize for ItemType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (*self as u8).serialize(serializer)
    }
}

impl<'a> From<&'a clean::Item> for ItemType {
    fn from(item: &'a clean::Item) -> ItemType {
        let kind = match *item.kind {
            clean::StrippedItem(box ref item) => item,
            ref kind => kind,
        };

        match *kind {
            clean::ModuleItem(..) => ItemType::Module,
            clean::ExternCrateItem(..) => ItemType::ExternCrate,
            clean::ImportItem(..) => ItemType::Import,
            clean::StructItem(..) => ItemType::Struct,
            clean::UnionItem(..) => ItemType::Union,
            clean::EnumItem(..) => ItemType::Enum,
            clean::FunctionItem(..) => ItemType::Function,
            clean::TypedefItem(..) => ItemType::Typedef,
            clean::OpaqueTyItem(..) => ItemType::OpaqueTy,
            clean::StaticItem(..) => ItemType::Static,
            clean::ConstantItem(..) => ItemType::Constant,
            clean::TraitItem(..) => ItemType::Trait,
            clean::ImplItem(..) => ItemType::Impl,
            clean::TyMethodItem(..) => ItemType::TyMethod,
            clean::MethodItem(..) => ItemType::Method,
            clean::StructFieldItem(..) => ItemType::StructField,
            clean::VariantItem(..) => ItemType::Variant,
            clean::ForeignFunctionItem(..) => ItemType::Function, // no ForeignFunction
            clean::ForeignStaticItem(..) => ItemType::Static,     // no ForeignStatic
            clean::MacroItem(..) => ItemType::Macro,
            clean::PrimitiveItem(..) => ItemType::Primitive,
            clean::AssocConstItem(..) => ItemType::AssocConst,
            clean::AssocTypeItem(..) => ItemType::AssocType,
            clean::ForeignTypeItem => ItemType::ForeignType,
            clean::KeywordItem(..) => ItemType::Keyword,
            clean::TraitAliasItem(..) => ItemType::TraitAlias,
            clean::ProcMacroItem(ref mac) => match mac.kind {
                MacroKind::Bang => ItemType::Macro,
                MacroKind::Attr => ItemType::ProcAttribute,
                MacroKind::Derive => ItemType::ProcDerive,
            },
            clean::StrippedItem(..) => unreachable!(),
        }
    }
}

impl<'a> From<&'a clean::Type> for ItemType {
    fn from(item: &'a clean::Type) -> ItemType {
        match item {
            clean::Type::ResolvedPath { path, .. } => {
                match path.res {
                    hir::def::Res::Def(def_kind, ..) => match def_kind {
                        hir::def::DefKind::Mod => ItemType::Module,
                        hir::def::DefKind::Struct => ItemType::Struct,
                        hir::def::DefKind::Union => ItemType::Union,
                        hir::def::DefKind::Enum => ItemType::Enum,
                        hir::def::DefKind::Trait => ItemType::Trait,
                        hir::def::DefKind::TyAlias => ItemType::Typedef,
                        hir::def::DefKind::ForeignTy => ItemType::ForeignType,
                        hir::def::DefKind::TraitAlias => ItemType::TraitAlias,
                        hir::def::DefKind::Fn => ItemType::Function,
                        hir::def::DefKind::Const => ItemType::Constant,
                        hir::def::DefKind::Static => ItemType::Static,
                        hir::def::DefKind::Macro(_) => ItemType::Macro,
                        _ => ItemType::ForeignType,
                    },
                    hir::def::Res::PrimTy(..) => ItemType::Primitive,
                    // Res::Err => ItemType::Primitive, // TODO: wrong
                    _ => panic!("{:?}", item),
                }
            }
            clean::Type::Generic(_) => ItemType::Primitive, // TODO: wrong
            clean::Type::Primitive(_) => ItemType::Primitive,
            clean::Type::BareFunction(_) => ItemType::Function,
            clean::Type::Tuple(_) => ItemType::Primitive,
            clean::Type::Slice(_) => ItemType::Primitive,
            clean::Type::Array(_, _) => ItemType::Primitive,
            clean::Type::Never => ItemType::Primitive,
            clean::Type::RawPointer(_, _) => ItemType::Primitive,
            clean::Type::BorrowedRef { type_: box clean::Generic(..), .. } => ItemType::Primitive,
            clean::Type::BorrowedRef { type_, .. } => ItemType::from(&**type_),
            clean::Type::QPath { self_type, .. } => ItemType::from(&**self_type),
            clean::Type::Infer => ItemType::Primitive, // TODO: wrong
            clean::Type::ImplTrait(_) => ItemType::Primitive, // TODO: wrong
        }
    }
}

impl From<clean::TypeKind> for ItemType {
    fn from(kind: clean::TypeKind) -> ItemType {
        match kind {
            clean::TypeKind::Struct => ItemType::Struct,
            clean::TypeKind::Union => ItemType::Union,
            clean::TypeKind::Enum => ItemType::Enum,
            clean::TypeKind::Function => ItemType::Function,
            clean::TypeKind::Trait => ItemType::Trait,
            clean::TypeKind::Module => ItemType::Module,
            clean::TypeKind::Static => ItemType::Static,
            clean::TypeKind::Const => ItemType::Constant,
            clean::TypeKind::Typedef => ItemType::Typedef,
            clean::TypeKind::Foreign => ItemType::ForeignType,
            clean::TypeKind::Macro => ItemType::Macro,
            clean::TypeKind::Attr => ItemType::ProcAttribute,
            clean::TypeKind::Derive => ItemType::ProcDerive,
            clean::TypeKind::TraitAlias => ItemType::TraitAlias,
        }
    }
}

impl ItemType {
    crate fn as_str(&self) -> &'static str {
        match *self {
            ItemType::Module => "mod",
            ItemType::ExternCrate => "externcrate",
            ItemType::Import => "import",
            ItemType::Struct => "struct",
            ItemType::Union => "union",
            ItemType::Enum => "enum",
            ItemType::Function => "fn",
            ItemType::Typedef => "type",
            ItemType::Static => "static",
            ItemType::Trait => "trait",
            ItemType::Impl => "impl",
            ItemType::TyMethod => "tymethod",
            ItemType::Method => "method",
            ItemType::StructField => "structfield",
            ItemType::Variant => "variant",
            ItemType::Macro => "macro",
            ItemType::Primitive => "primitive",
            ItemType::AssocType => "associatedtype",
            ItemType::Constant => "constant",
            ItemType::AssocConst => "associatedconstant",
            ItemType::ForeignType => "foreigntype",
            ItemType::Keyword => "keyword",
            ItemType::OpaqueTy => "opaque",
            ItemType::ProcAttribute => "attr",
            ItemType::ProcDerive => "derive",
            ItemType::TraitAlias => "traitalias",
        }
    }
}

impl fmt::Display for ItemType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

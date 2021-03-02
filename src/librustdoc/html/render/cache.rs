use std::collections::BTreeMap;
use std::path::Path;

use rustc_data_structures::fx::{FxHashMap, FxHashSet};
use rustc_span::symbol::{sym, Symbol};
use serde::Serialize;

use crate::clean::types::{
    FnDecl, FnRetTy, GenericBound, Generics, GetDefId, Type, WherePredicate,
};
use crate::clean::{self, AttributesExt};
use crate::formats::cache::Cache;
use crate::formats::item_type::ItemType;
use crate::html::markdown::short_markdown_summary;
use crate::html::render::{IndexItem, IndexItemFunctionType, TypeWithKind};

/// Indicates where an external crate can be found.
crate enum ExternalLocation {
    /// Remote URL root of the external crate
    Remote(String),
    /// This external crate can be found in the local doc/ folder
    Local,
    /// The external crate could not be found.
    Unknown,
}

/// Attempts to find where an external crate is located, given that we're
/// rendering in to the specified source destination.
crate fn extern_location(
    e: &clean::ExternalCrate,
    extern_url: Option<&str>,
    dst: &Path,
) -> ExternalLocation {
    use ExternalLocation::*;
    // See if there's documentation generated into the local directory
    let local_location = dst.join(&*e.name.as_str());
    if local_location.is_dir() {
        return Local;
    }

    if let Some(url) = extern_url {
        let mut url = url.to_string();
        if !url.ends_with('/') {
            url.push('/');
        }
        return Remote(url);
    }

    // Failing that, see if there's an attribute specifying where to find this
    // external crate
    e.attrs
        .lists(sym::doc)
        .filter(|a| a.has_name(sym::html_root_url))
        .filter_map(|a| a.value_str())
        .map(|url| {
            let mut url = url.to_string();
            if !url.ends_with('/') {
                url.push('/')
            }
            Remote(url)
        })
        .next()
        .unwrap_or(Unknown) // Well, at least we tried.
}

/// Builds the search index from the collected metadata
crate fn build_index(krate: &clean::Crate, cache: &mut Cache) -> String {
    let mut defid_to_pathid = FxHashMap::default();
    let mut crate_items = Vec::with_capacity(cache.search_index.len());
    let mut crate_paths = vec![];

    // Attach all orphan items to the type's definition if the type
    // has since been learned.
    for &(did, ref item) in &cache.orphan_impl_items {
        if let Some(&(ref fqp, _)) = cache.paths.get(&did) {
            cache.search_index.push(IndexItem {
                ty: item.type_(),
                name: item.name.unwrap().to_string(),
                path: fqp[..fqp.len() - 1].join("::"),
                desc: item.doc_value().map_or_else(String::new, |s| short_markdown_summary(&s)),
                parent: Some(did),
                parent_idx: None,
                search_type: get_index_search_type(&item),
            });
            for alias in item.attrs.get_doc_aliases() {
                cache
                    .aliases
                    .entry(alias.to_lowercase())
                    .or_insert(Vec::new())
                    .push(cache.search_index.len() - 1);
            }
        }
    }

    let Cache { ref mut search_index, ref paths, ref mut aliases, .. } = *cache;

    // Reduce `DefId` in paths into smaller sequential numbers,
    // and prune the paths that do not appear in the index.
    let mut lastpath = String::new();
    let mut lastpathid = 0usize;

    for item in search_index {
        item.parent_idx = item.parent.and_then(|defid| {
            if defid_to_pathid.contains_key(&defid) {
                defid_to_pathid.get(&defid).copied()
            } else {
                let pathid = lastpathid;
                defid_to_pathid.insert(defid, pathid);
                lastpathid += 1;

                if let Some(&(ref fqp, short)) = paths.get(&defid) {
                    crate_paths.push((short, fqp.last().unwrap().clone()));
                    Some(pathid)
                } else {
                    None
                }
            }
        });

        // Omit the parent path if it is same to that of the prior item.
        if lastpath == item.path {
            item.path.clear();
        } else {
            lastpath = item.path.clone();
        }
        crate_items.push(&*item);
    }

    let crate_doc = krate
        .module
        .as_ref()
        .map(|module| module.doc_value().map_or_else(String::new, |s| short_markdown_summary(&s)))
        .unwrap_or_default();

    #[derive(Serialize)]
    struct CrateData<'a> {
        doc: String,
        #[serde(rename = "i")]
        items: Vec<&'a IndexItem>,
        #[serde(rename = "p")]
        paths: Vec<(ItemType, String)>,
        // The String is alias name and the vec is the list of the elements with this alias.
        //
        // To be noted: the `usize` elements are indexes to `items`.
        #[serde(rename = "a")]
        #[serde(skip_serializing_if = "BTreeMap::is_empty")]
        aliases: &'a BTreeMap<String, Vec<usize>>,
    }

    // Collect the index into a string
    format!(
        r#""{}":{}"#,
        krate.name,
        serde_json::to_string(&CrateData {
            doc: crate_doc,
            items: crate_items,
            paths: crate_paths,
            aliases,
        })
        .expect("failed serde conversion")
        // All these `replace` calls are because we have to go through JS string for JSON content.
        .replace(r"\", r"\\")
        .replace("'", r"\'")
        // We need to escape double quotes for the JSON.
        .replace("\\\"", "\\\\\"")
    )
}

crate fn get_index_search_type(item: &clean::Item) -> Option<IndexItemFunctionType> {
    let (generics, decl) = match *item.kind {
        clean::FunctionItem(ref f) => (&f.generics, &f.decl),
        clean::MethodItem(ref m, _) => (&m.generics, &m.decl),
        clean::TyMethodItem(ref m) => (&m.generics, &m.decl),
        _ => return None,
    };

    let (inputs, output) = get_all_types(generics, decl);

    let inputs = inputs
        .into_iter()
        .filter_map(|ty| {
            let name = get_index_type_name(ty)?.as_str().to_ascii_lowercase();
            Some(TypeWithKind { name, kind: ItemType::from(ty) })
        })
        .collect();

    let output = output
        .into_iter()
        .filter_map(|ty| {
            let name = get_index_type_name(ty)?.as_str().to_ascii_lowercase();
            Some(TypeWithKind { name, kind: ItemType::from(ty) })
        })
        .collect();

    Some(IndexItemFunctionType { inputs, output })
}

fn get_index_type_name(clean_type: &clean::Type) -> Option<Symbol> {
    match *clean_type {
        clean::ResolvedPath { ref path, .. } => {
            let segments = &path.segments;
            let path_segment = segments.iter().last().unwrap_or_else(|| {
                panic!("get_index_type_name(clean_type: {:?}) had length zero path", clean_type)
            });
            Some(path_segment.name)
        }
        clean::Primitive(ref p) => Some(p.as_sym()),
        clean::BorrowedRef { ref type_, .. } => get_index_type_name(type_),
        clean::Generic(_)
        | clean::BareFunction(_)
        | clean::Tuple(_)
        | clean::Slice(_)
        | clean::Array(_, _)
        | clean::Never
        | clean::RawPointer(_, _)
        | clean::QPath { .. }
        | clean::Infer
        | clean::ImplTrait(_) => None,
    }
}

/// The point of this function is to replace bounds with types.
///
/// i.e. `[T, U]` when you have the following bounds: `T: Display, U: Option<T>` will return
/// `[Display, Option]` (we just returns the list of the types, we don't care about the
/// wrapped types in here).
fn get_real_types<'a>(
    generics: &'a Generics,
    arg: &'a Type,
    recurse: i32,
    res: &mut FxHashSet<&'a Type>,
) {
    if recurse >= 10 {
        // FIXME: remove this whole recurse thing when the recursion bug is fixed
        return;
    }

    if let &Type::Generic(arg_s) = arg {
        if let Some(where_pred) = generics.where_predicates.iter().find(|g| match g {
            WherePredicate::BoundPredicate { ty, .. } => ty.def_id() == arg.def_id(),
            _ => false,
        }) {
            let bounds = where_pred.get_bounds().unwrap_or_else(|| &[]);
            for bound in bounds.iter() {
                if let GenericBound::TraitBound(poly_trait, _) = bound {
                    for x in poly_trait.generic_params.iter() {
                        if !x.is_type() {
                            continue;
                        }
                        if let Some(ty) = x.get_type() {
                            get_real_types(generics, &ty, recurse + 1, res);
                        }
                    }
                }
            }
        }
        if let Some(bound) = generics.params.iter().find(|g| g.is_type() && g.name == arg_s) {
            for bound in bound.get_bounds().unwrap_or(&[]) {
                if let Some(ty) = bound.get_trait_type() {
                    get_real_types(generics, &ty, recurse + 1, res);
                }
            }
        }
    } else {
        res.insert(arg);
        if let Some(gens) = arg.generics() {
            for gen in gens.iter() {
                if gen.is_full_generic() {
                    get_real_types(generics, gen, recurse + 1, res);
                } else {
                    res.insert(gen);
                }
            }
        }
    }
}

/// Return the full list of types when bounds have been resolved.
///
/// i.e. `fn foo<A: Display, B: Option<A>>(x: u32, y: B)` will return
/// `[u32, Display, Option]`.
fn get_all_types<'a>(generics: &'a Generics, decl: &'a FnDecl) -> (Vec<&'a Type>, Vec<&'a Type>) {
    let mut all_types = FxHashSet::default();
    for arg in decl.inputs.values.iter() {
        if arg.type_.is_self_type() {
            continue;
        }
        get_real_types(generics, &arg.type_, 0, &mut all_types);
    }

    let ret_types = match decl.output {
        FnRetTy::Return(ref return_type) => {
            let mut ret = FxHashSet::default();
            get_real_types(generics, &return_type, 0, &mut ret);
            ret.into_iter().collect()
        }
        _ => Vec::new(),
    };
    (all_types.into_iter().collect(), ret_types)
}

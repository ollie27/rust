// Copyright 2012-2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use rustc::util::nodemap::DefIdSet;
use rustc::middle::stability;
use std::mem;

use clean;
use clean::Item;
use plugins;
use fold;
use fold::DocFolder;
use fold::FoldItem::Strip;
use passes::ImplStripper;

pub fn strip_unstable(krate: clean::Crate) -> plugins::PluginResult {
    let mut retained = DefIdSet();

    let krate = {
        let mut stripper = Stripper{ retained: &mut retained, update_retained: true };
        stripper.fold_crate(krate)
    };

    let mut stripper = ImplStripper { retained: &retained };
    stripper.fold_crate(krate)
}

struct Stripper<'a> {
    retained: &'a mut DefIdSet,
    update_retained: bool,
}

impl<'a> fold::DocFolder for Stripper<'a> {
    fn fold_item(&mut self, i: Item) -> Option<Item> {
        if let Some(clean::Stability { level: stability::Unstable, issue: Some(0), .. }) = i.stability {
            match i.inner {
                clean::StructFieldItem(..) | clean::ModuleItem(..) => {
                    // We need to recurse into stripped modules to
                    // strip things like impl methods but when doing so
                    // we must not add any items to the `retained` set.
                    let old = mem::replace(&mut self.update_retained, false);
                    let ret = Strip(self.fold_item_recur(i).unwrap()).fold();
                    self.update_retained = old;
                    return ret;
                }
                clean::StrippedItem(..) => {
                    // We need to recurse into stripped modules to strip things
                    // like impl methods but when doing so we must not add any
                    // items to the `retained` set.
                    let old = mem::replace(&mut self.update_retained, false);
                    let ret = self.fold_item_recur(i);
                    self.update_retained = old;
                    return ret;
                }
                clean::DefaultImplItem(..) | clean::ImplItem(..) => {}
                _ => return None,
            }
        }
        if self.update_retained {
            self.retained.insert(i.def_id);
        }
        self.fold_item_recur(i)
    }
}

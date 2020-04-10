use crate::clean;
use crate::fold::DocFolder;
use crate::html::format::Buffer;
use crate::html::highlight;
use crate::html::layout;
use crate::html::render::{Error, SharedContext, BASIC_KEYWORDS};
use rustc_hir::def_id::LOCAL_CRATE;
use rustc_span::source_map::{FileName, SourceFile};
use std::ffi::OsStr;
use std::fs;
use std::path::{Component, Path, PathBuf};

crate fn render(
    dst: &Path,
    scx: &mut SharedContext,
    krate: clean::Crate,
) -> Result<clean::Crate, Error> {
    info!("emitting source files");
    let dst = dst.join("src").join(&krate.name);
    scx.ensure_dir(&dst)?;
    let mut folder = SourceCollector { dst, scx };
    Ok(folder.fold_crate(krate))
}

/// Helper struct to render all source code to HTML pages
struct SourceCollector<'a> {
    scx: &'a mut SharedContext,

    /// Root destination to place all HTML output into
    dst: PathBuf,
}

impl<'a> DocFolder for SourceCollector<'a> {
    fn fold_item(&mut self, item: clean::Item) -> Option<clean::Item> {
        // If we're including source files, and we haven't seen this file yet,
        // then we need to render it out to the filesystem.
        if self.scx.include_sources
            // skip all synthetic "files"
            && item.source.file.name.is_real()
            // skip non-local files
            && item.source.file.cnum == LOCAL_CRATE
        {
            self.emit_source(&item.source.file).unwrap(); // TODO: proper error handling
        }
        self.fold_item_recur(item)
    }
}

impl<'a> SourceCollector<'a> {
    /// Renders the given file into its corresponding HTML source file.
    fn emit_source(&mut self, file: &SourceFile) -> Result<(), Error> {
        let p = match &file.name {
            FileName::Real(file) => file,
            _ => return Ok(()),
        };
        if self.scx.local_sources.contains_key(p) {
            // We've already emitted this source
            return Ok(());
        }

        // Create the intermediate directories
        let mut cur = self.dst.clone();
        let mut root_path = String::from("../../");
        let mut href = String::new();
        clean_path(&self.scx.src_root, p, false, |component| {
            cur.push(component);
            root_path.push_str("../");
            href.push_str(&component.to_string_lossy());
            href.push('/');
        });
        self.scx.ensure_dir(&cur)?;
        let mut fname = p.file_name().expect("source has no filename").to_os_string();
        fname.push(".html");
        cur.push(&fname);
        href.push_str(&fname.to_string_lossy());

        let title = format!(
            "{} -- source",
            cur.file_name().expect("failed to get file name").to_string_lossy()
        );
        let desc = format!("Source to the Rust file `{}`.", file.name);
        let page = layout::Page {
            title: &title,
            css_class: "source",
            root_path: &root_path,
            static_root_path: self.scx.static_root_path.as_deref(),
            description: &desc,
            keywords: BASIC_KEYWORDS,
            resource_suffix: &self.scx.resource_suffix,
            extra_scripts: &[&format!("source-files{}", self.scx.resource_suffix)],
            static_extra_scripts: &[&format!("source-script{}", self.scx.resource_suffix)],
        };
        let v = layout::render(
            &self.scx.layout,
            &page,
            "",
            |buf: &mut _| print_src(buf, file.src.as_ref().unwrap()), // TODO: can this unwrap be removed?
            &self.scx.themes,
        );
        self.scx.fs.write(&cur, v.as_bytes())?;
        self.scx.local_sources.insert(p.clone(), href);
        Ok(())
    }
}

/// Takes a path to a source file and cleans the path to it. This canonicalizes
/// things like ".." to components which preserve the "top down" hierarchy of a
/// static HTML tree. Each component in the cleaned path will be passed as an
/// argument to `f`. The very last component of the path (ie the file name) will
/// be passed to `f` if `keep_filename` is true, and ignored otherwise.
pub fn clean_path<F>(src_root: &Path, p: &Path, keep_filename: bool, mut f: F)
where
    F: FnMut(&OsStr),
{
    // make it relative, if possible
    let p = p.strip_prefix(src_root).unwrap_or(p);

    let mut iter = p.components().peekable();

    while let Some(c) = iter.next() {
        if !keep_filename && iter.peek().is_none() {
            break;
        }

        match c {
            Component::ParentDir => f("up".as_ref()),
            Component::Normal(c) => f(c),
            _ => continue,
        }
    }
}

/// Wrapper struct to render the source code of a file. This will do things like
/// adding line numbers to the left-hand side.
fn print_src(buf: &mut Buffer, s: &str) {
    let lines = s.lines().count();
    let mut cols = 0;
    let mut tmp = lines;
    while tmp > 0 {
        cols += 1;
        tmp /= 10;
    }
    write!(buf, "<pre class=\"line-numbers\">");
    for i in 1..=lines {
        write!(buf, "<span id=\"{0}\">{0:1$}</span>\n", i, cols);
    }
    write!(buf, "</pre>");
    write!(buf, "{}", highlight::render_with_highlighting(s, None, None, None));
}

// Copyright 2013-2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Markdown formatting for rustdoc
//!
//! This module implements markdown formatting through the pulldown-cmark
//! rust-library. This module exposes all of the
//! functionality through a unit-struct, `Markdown`, which has an implementation
//! of `fmt::Display`. Example usage:
//!
//! ```rust,ignore
//! use rustdoc::html::markdown::Markdown;
//!
//! let s = "My *markdown* _text_";
//! let html = format!("{}", Markdown(s));
//! // ... something using html
//! ```

#![allow(non_camel_case_types)]

use std::ascii::AsciiExt;
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::default::Default;
use std::fmt::{self, Write};
use std::iter;
use std::str;
use syntax::feature_gate::UnstableFeatures;
use syntax::codemap::Span;

use html::render::derive_id;
use html::toc::TocBuilder;
use html::highlight;
use test;

use pulldown_cmark::{html, Event, Tag, Parser};
use pulldown_cmark::{Options, OPTION_ENABLE_FOOTNOTES, OPTION_ENABLE_TABLES};

/// A unit struct which has the `fmt::Display` trait implemented. When
/// formatted, this struct will emit the HTML corresponding to the rendered
/// version of the contained markdown string.
// The second parameter is whether we need a shorter version or not.
pub struct Markdown<'a>(pub &'a str);
/// A unit struct like `Markdown`, that renders the markdown with a
/// table of contents.
pub struct MarkdownWithToc<'a>(pub &'a str);
/// A unit struct like `Markdown`, that renders the markdown escaping HTML tags.
pub struct MarkdownHtml<'a>(pub &'a str);
/// A unit struct like `Markdown`, that renders only the first paragraph.
pub struct MarkdownSummaryLine<'a>(pub &'a str);
pub struct MarkdownSummaryLineNoLinks<'a>(pub &'a str, pub Option<String>);

/// Returns Some(code) if `s` is a line that should be stripped from
/// documentation but used in example code. `code` is the portion of
/// `s` that should be used in tests. (None for lines that should be
/// left as-is.)
fn stripped_filtered_line<'a>(s: &'a str) -> Option<&'a str> {
    let trimmed = s.trim();
    if trimmed == "#" {
        Some("")
    } else if trimmed.starts_with("# ") {
        Some(&trimmed[2..])
    } else {
        None
    }
}

/// Convert chars from a title for an id.
///
/// "Hello, world!" -> "hello-world"
fn slugify(c: char) -> Option<char> {
    if c.is_alphanumeric() || c == '-' || c == '_' {
        if c.is_ascii() {
            Some(c.to_ascii_lowercase())
        } else {
            Some(c)
        }
    } else if c.is_whitespace() && c.is_ascii() {
        Some('-')
    } else {
        None
    }
}

// Information about the playground if a URL has been specified, containing an
// optional crate name and the URL.
thread_local!(pub static PLAYGROUND: RefCell<Option<(Option<String>, String)>> = {
    RefCell::new(None)
});

/// Adds syntax highlighting and playground Run buttons to rust code blocks.
struct CodeBlocks<'a, I: Iterator<Item = Event<'a>>> {
    inner: I,
}

impl<'a, I: Iterator<Item = Event<'a>>> CodeBlocks<'a, I> {
    fn new(iter: I) -> Self {
        CodeBlocks {
            inner: iter,
        }
    }
}

impl<'a, I: Iterator<Item = Event<'a>>> Iterator for CodeBlocks<'a, I> {
    type Item = Event<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let event = self.inner.next();
        if let Some(Event::Start(Tag::CodeBlock(lang))) = event {
            if !LangString::parse(&lang).rust {
                return Some(Event::Start(Tag::CodeBlock(lang)));
            }
        } else {
            return event;
        }

        let mut origtext = String::new();
        for event in &mut self.inner {
            match event {
                Event::End(Tag::CodeBlock(..)) => break,
                Event::Text(ref s) => {
                    origtext.push_str(s);
                }
                _ => {}
            }
        }
        let lines = origtext.lines().filter(|l| {
            stripped_filtered_line(*l).is_none()
        });
        let text = lines.collect::<Vec<&str>>().join("\n");
        PLAYGROUND.with(|play| {
            // insert newline to clearly separate it from the
            // previous block so we can shorten the html output
            let mut s = String::from("\n");
            let playground_button = play.borrow().as_ref().and_then(|&(ref krate, ref url)| {
                if url.is_empty() {
                    return None;
                }
                let test = origtext.lines().map(|l| {
                    stripped_filtered_line(l).unwrap_or(l)
                }).collect::<Vec<&str>>().join("\n");
                let krate = krate.as_ref().map(|s| &**s);
                let test = test::maketest(&test, krate, false,
                                        &Default::default());
                let channel = if test.contains("#![feature(") {
                    "&amp;version=nightly"
                } else {
                    ""
                };
                // These characters don't need to be escaped in a URI.
                // FIXME: use a library function for percent encoding.
                fn dont_escape(c: u8) -> bool {
                    (b'a' <= c && c <= b'z') ||
                    (b'A' <= c && c <= b'Z') ||
                    (b'0' <= c && c <= b'9') ||
                    c == b'-' || c == b'_' || c == b'.' ||
                    c == b'~' || c == b'!' || c == b'\'' ||
                    c == b'(' || c == b')' || c == b'*'
                }
                let mut test_escaped = String::new();
                for b in test.bytes() {
                    if dont_escape(b) {
                        test_escaped.push(char::from(b));
                    } else {
                        write!(test_escaped, "%{:02X}", b).unwrap();
                    }
                }
                Some(format!(
                    r#"<a class="test-arrow" target="_blank" href="{}?code={}{}">Run</a>"#,
                    url, test_escaped, channel
                ))
            });
            s.push_str(&highlight::render_with_highlighting(
                        &text,
                        Some("rust-example-rendered"),
                        None,
                        playground_button.as_ref().map(String::as_str)));
            Some(Event::Html(s.into()))
        })
    }
}

/// Make headings links with anchor ids and build up TOC.
struct HeadingLinks<'a, 'b, I: Iterator<Item = Event<'a>>> {
    inner: I,
    toc: Option<&'b mut TocBuilder>,
    buf: VecDeque<Event<'a>>,
}

impl<'a, 'b, I: Iterator<Item = Event<'a>>> HeadingLinks<'a, 'b, I> {
    fn new(iter: I, toc: Option<&'b mut TocBuilder>) -> Self {
        HeadingLinks {
            inner: iter,
            toc: toc,
            buf: VecDeque::new(),
        }
    }
}

impl<'a, 'b, I: Iterator<Item = Event<'a>>> Iterator for HeadingLinks<'a, 'b, I> {
    type Item = Event<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(e) = self.buf.pop_front() {
            return Some(e);
        }

        let event = self.inner.next();
        if let Some(Event::Start(Tag::Header(level))) = event {
            let mut id = String::new();
            for event in &mut self.inner {
                match event {
                    Event::End(Tag::Header(..)) => break,
                    Event::Text(ref text) => id.extend(text.chars().filter_map(slugify)),
                    _ => {},
                }
                self.buf.push_back(event);
            }
            let id = derive_id(id);

            if let Some(ref mut builder) = self.toc {
                let mut html_header = String::new();
                html::push_html(&mut html_header, self.buf.iter().cloned());
                let sec = builder.push(level as u32, html_header, id.clone());
                self.buf.push_front(Event::InlineHtml(format!("{} ", sec).into()));
            }

            self.buf.push_back(Event::InlineHtml(format!("</a></h{}>", level).into()));

            let start_tags = format!("<h{level} id=\"{id}\" class=\"section-header\">\
                                      <a href=\"#{id}\">",
                                     id = id,
                                     level = level);
            return Some(Event::InlineHtml(start_tags.into()));
        }
        event
    }
}

/// Extracts just the first paragraph.
struct SummaryLine<'a, I: Iterator<Item = Event<'a>>> {
    inner: I,
    started: bool,
    depth: u32,
}

impl<'a, I: Iterator<Item = Event<'a>>> SummaryLine<'a, I> {
    fn new(iter: I) -> Self {
        SummaryLine {
            inner: iter,
            started: false,
            depth: 0,
        }
    }
}

impl<'a, I: Iterator<Item = Event<'a>>> Iterator for SummaryLine<'a, I> {
    type Item = Event<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.started && self.depth == 0 {
                return None;
            }
            if !self.started {
                self.started = true;
            }
            let event = self.inner.next();
            match event {
                Some(Event::Start(..)) => {
                    self.depth += 1;
                    if self.depth > 1 {
                        return event;
                    }
                }
                Some(Event::End(..)) => {
                    self.depth -= 1;
                    if self.depth > 0 {
                        return event;
                    }
                }
                _ => return event,
            }
        }
    }
}

/// Moves all footnote definitions to the end and add back links to the
/// references.
struct Footnotes<'a, I: Iterator<Item = Event<'a>>> {
    inner: I,
    footnotes: HashMap<String, (Vec<Event<'a>>, u16)>,
}

impl<'a, I: Iterator<Item = Event<'a>>> Footnotes<'a, I> {
    fn new(iter: I) -> Self {
        Footnotes {
            inner: iter,
            footnotes: HashMap::new(),
        }
    }
    fn get_entry(&mut self, key: &str) -> &mut (Vec<Event<'a>>, u16) {
        let new_id = self.footnotes.keys().count() + 1;
        let key = key.to_owned();
        self.footnotes.entry(key).or_insert((Vec::new(), new_id as u16))
    }
}

impl<'a, I: Iterator<Item = Event<'a>>> Iterator for Footnotes<'a, I> {
    type Item = Event<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.inner.next() {
                Some(Event::FootnoteReference(ref reference)) => {
                    let entry = self.get_entry(&reference);
                    let reference = format!("<sup id=\"supref{0}\"><a href=\"#ref{0}\">{0}\
                                             </a></sup>",
                                            (*entry).1);
                    return Some(Event::Html(reference.into()));
                }
                Some(Event::Start(Tag::FootnoteDefinition(def))) => {
                    let mut content = Vec::new();
                    for event in &mut self.inner {
                        if let Event::End(Tag::FootnoteDefinition(..)) = event {
                            break;
                        }
                        content.push(event);
                    }
                    let entry = self.get_entry(&def);
                    (*entry).0 = content;
                }
                Some(e) => return Some(e),
                None => {
                    if !self.footnotes.is_empty() {
                        let mut v: Vec<_> = self.footnotes.drain().map(|(_, x)| x).collect();
                        v.sort_by(|a, b| a.1.cmp(&b.1));
                        let mut ret = String::from("<div class=\"footnotes\"><hr><ol>");
                        for (mut content, id) in v {
                            write!(ret, "<li id=\"ref{}\">", id).unwrap();
                            let mut is_paragraph = false;
                            if let Some(&Event::End(Tag::Paragraph)) = content.last() {
                                content.pop();
                                is_paragraph = true;
                            }
                            html::push_html(&mut ret, content.into_iter());
                            write!(ret,
                                   "&nbsp;<a href=\"#supref{}\" rev=\"footnote\">↩</a>",
                                   id).unwrap();
                            if is_paragraph {
                                ret.push_str("</p>");
                            }
                            ret.push_str("</li>");
                        }
                        ret.push_str("</ol></div>");
                        return Some(Event::Html(ret.into()));
                    } else {
                        return None;
                    }
                }
            }
        }
    }
}

pub fn find_testable_code(doc: &str, tests: &mut ::test::Collector, position: Span) {
    tests.set_position(position);

    let mut parser = Parser::new(doc);
    let mut prev_offset = 0;
    let mut nb_lines = 0;
    let mut register_header = None;
    'main: while let Some(event) = parser.next() {
        match event {
            Event::Start(Tag::CodeBlock(s)) => {
                let block_info = if s.is_empty() {
                    LangString::all_false()
                } else {
                    LangString::parse(&*s)
                };
                if !block_info.rust {
                    continue
                }
                let mut test_s = String::new();
                let mut offset = None;
                loop {
                    let event = parser.next();
                    if let Some(event) = event {
                        match event {
                            Event::End(Tag::CodeBlock(_)) => break,
                            Event::Text(ref s) => {
                                test_s.push_str(s);
                                if offset.is_none() {
                                    offset = Some(parser.get_offset());
                                }
                            }
                            _ => {}
                        }
                    } else {
                        break 'main;
                    }
                }
                let offset = offset.unwrap_or(0);
                let lines = test_s.lines().map(|l| {
                    stripped_filtered_line(l).unwrap_or(l)
                });
                let text = lines.collect::<Vec<&str>>().join("\n");
                nb_lines += doc[prev_offset..offset].lines().count();
                let line = tests.get_line() + (nb_lines - 1);
                let filename = tests.get_filename();
                tests.add_test(text.to_owned(),
                               block_info.should_panic, block_info.no_run,
                               block_info.ignore, block_info.test_harness,
                               block_info.compile_fail, block_info.error_codes,
                               line, filename);
                prev_offset = offset;
            }
            Event::Start(Tag::Header(level)) => {
                register_header = Some(level as u32);
            }
            Event::Text(ref s) if register_header.is_some() => {
                let level = register_header.unwrap();
                if s.is_empty() {
                    tests.register_header("", level);
                } else {
                    tests.register_header(s, level);
                }
                register_header = None;
            }
            _ => {}
        }
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct LangString {
    original: String,
    should_panic: bool,
    no_run: bool,
    ignore: bool,
    rust: bool,
    test_harness: bool,
    compile_fail: bool,
    error_codes: Vec<String>,
}

impl LangString {
    fn all_false() -> LangString {
        LangString {
            original: String::new(),
            should_panic: false,
            no_run: false,
            ignore: false,
            rust: true,  // NB This used to be `notrust = false`
            test_harness: false,
            compile_fail: false,
            error_codes: Vec::new(),
        }
    }

    fn parse(string: &str) -> LangString {
        let mut seen_rust_tags = false;
        let mut seen_other_tags = false;
        let mut data = LangString::all_false();
        let mut allow_compile_fail = false;
        let mut allow_error_code_check = false;
        if UnstableFeatures::from_environment().is_nightly_build() {
            allow_compile_fail = true;
            allow_error_code_check = true;
        }

        data.original = string.to_owned();
        let tokens = string.split(|c: char|
            !(c == '_' || c == '-' || c.is_alphanumeric())
        );

        for token in tokens {
            match token {
                "" => {},
                "should_panic" => { data.should_panic = true; seen_rust_tags = true; },
                "no_run" => { data.no_run = true; seen_rust_tags = true; },
                "ignore" => { data.ignore = true; seen_rust_tags = true; },
                "rust" => { data.rust = true; seen_rust_tags = true; },
                "test_harness" => { data.test_harness = true; seen_rust_tags = true; },
                "compile_fail" if allow_compile_fail => {
                    data.compile_fail = true;
                    seen_rust_tags = true;
                    data.no_run = true;
                }
                x if allow_error_code_check && x.starts_with("E") && x.len() == 5 => {
                    if let Ok(_) = x[1..].parse::<u32>() {
                        data.error_codes.push(x.to_owned());
                        seen_rust_tags = true;
                    } else {
                        seen_other_tags = true;
                    }
                }
                _ => { seen_other_tags = true }
            }
        }

        data.rust &= !seen_other_tags || seen_rust_tags;

        data
    }
}

impl<'a> fmt::Display for Markdown<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let Markdown(md) = *self;
        // This is actually common enough to special-case
        if md.is_empty() { return Ok(()) }

        let mut opts = Options::empty();
        opts.insert(OPTION_ENABLE_TABLES);
        opts.insert(OPTION_ENABLE_FOOTNOTES);

        let p = Parser::new_ext(md, opts);

        let mut s = String::with_capacity(md.len() * 3 / 2);

        html::push_html(&mut s,
                        Footnotes::new(CodeBlocks::new(HeadingLinks::new(p, None))));

        fmt.write_str(&s)
    }
}

impl<'a> fmt::Display for MarkdownWithToc<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let MarkdownWithToc(md) = *self;

        let mut opts = Options::empty();
        opts.insert(OPTION_ENABLE_TABLES);
        opts.insert(OPTION_ENABLE_FOOTNOTES);

        let p = Parser::new_ext(md, opts);

        let mut s = String::with_capacity(md.len() * 3 / 2);

        let mut toc = TocBuilder::new();

        html::push_html(&mut s,
                        Footnotes::new(CodeBlocks::new(HeadingLinks::new(p, Some(&mut toc)))));

        write!(fmt, "<nav id=\"TOC\">{}</nav>", toc.into_toc())?;

        fmt.write_str(&s)
    }
}

impl<'a> fmt::Display for MarkdownHtml<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let MarkdownHtml(md) = *self;
        // This is actually common enough to special-case
        if md.is_empty() { return Ok(()) }

        let mut opts = Options::empty();
        opts.insert(OPTION_ENABLE_TABLES);
        opts.insert(OPTION_ENABLE_FOOTNOTES);

        let p = Parser::new_ext(md, opts);

        // Treat inline HTML as plain text.
        let p = p.map(|event| match event {
            Event::Html(text) | Event::InlineHtml(text) => Event::Text(text),
            _ => event
        });

        let mut s = String::with_capacity(md.len() * 3 / 2);

        html::push_html(&mut s,
                        Footnotes::new(CodeBlocks::new(HeadingLinks::new(p, None))));

        fmt.write_str(&s)
    }
}

impl<'a> fmt::Display for MarkdownSummaryLine<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let MarkdownSummaryLine(md) = *self;
        // This is actually common enough to special-case
        if md.is_empty() { return Ok(()) }

        let p = Parser::new(md);

        let mut s = String::new();

        html::push_html(&mut s, iter::once(Event::Start(Tag::Paragraph)).chain(SummaryLine::new(p).chain(iter::once(Event::End(Tag::Paragraph)))));

        fmt.write_str(&s)
    }
}

impl<'a> fmt::Display for MarkdownSummaryLineNoLinks<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let MarkdownSummaryLineNoLinks(md, ref link) = *self;
        // This is actually common enough to special-case
        if md.is_empty() { return Ok(()) }

        let p = Parser::new(md);

        let mut p = p.filter_map(|event| match event {
            Event::Start(Tag::Link(..)) | Event::End(Tag::Link(..)) => None,
            event => Some(event),
        });

        let mut s = String::new();

        html::push_html(&mut s, SummaryLine::new(&mut p));

        fmt.write_str(&s)?;
        if let Some(ref url) = *link {
            if p.next().is_some() {
                write!(fmt, " <a href=\"{}\">Read more</a>", ::html::escape::Escape(url))?;
            }
        }
        Ok(())
    }
}

pub fn plain_summary_line(md: &str) -> String {
    let mut s = String::new();
    for event in SummaryLine::new(Parser::new(md)) {
        match event {
            Event::Text(text) => s.push_str(&text.replace("\n", " ")),
            Event::SoftBreak | Event::HardBreak => s.push_str(" "),
            Event::Start(Tag::Code) | Event::End(Tag::Code) => s.push_str("`"),
            _ => {},
        }
    }
    s
}

#[cfg(test)]
mod tests {
    use super::{LangString, Markdown, MarkdownHtml};
    use super::plain_summary_line;
    use html::render::reset_ids;

    #[test]
    fn test_lang_string_parse() {
        fn t(s: &str,
            should_panic: bool, no_run: bool, ignore: bool, rust: bool, test_harness: bool,
            compile_fail: bool, error_codes: Vec<String>) {
            assert_eq!(LangString::parse(s), LangString {
                should_panic: should_panic,
                no_run: no_run,
                ignore: ignore,
                rust: rust,
                test_harness: test_harness,
                compile_fail: compile_fail,
                error_codes: error_codes,
                original: s.to_owned(),
            })
        }

        // marker                | should_panic| no_run| ignore| rust | test_harness| compile_fail
        //                       | error_codes
        t("",                      false,        false,  false,  true,  false, false, Vec::new());
        t("rust",                  false,        false,  false,  true,  false, false, Vec::new());
        t("sh",                    false,        false,  false,  false, false, false, Vec::new());
        t("ignore",                false,        false,  true,   true,  false, false, Vec::new());
        t("should_panic",          true,         false,  false,  true,  false, false, Vec::new());
        t("no_run",                false,        true,   false,  true,  false, false, Vec::new());
        t("test_harness",          false,        false,  false,  true,  true,  false, Vec::new());
        t("compile_fail",          false,        true,   false,  true,  false, true,  Vec::new());
        t("{.no_run .example}",    false,        true,   false,  true,  false, false, Vec::new());
        t("{.sh .should_panic}",   true,         false,  false,  true,  false, false, Vec::new());
        t("{.example .rust}",      false,        false,  false,  true,  false, false, Vec::new());
        t("{.test_harness .rust}", false,        false,  false,  true,  true,  false, Vec::new());
    }

    #[test]
    fn issue_17736() {
        let markdown = "# title";
        format!("{}", Markdown(markdown));
        reset_ids(true);
    }

    #[test]
    fn test_header() {
        fn t(input: &str, expect: &str) {
            let output = format!("{}", Markdown(input));
            assert_eq!(output, expect, "original: {}", input);
            reset_ids(true);
        }

        t("# Foo bar", "<h1 id=\"foo-bar\" class=\"section-header\">\
          <a href=\"#foo-bar\">Foo bar</a></h1>");
        t("## Foo-bar_baz qux", "<h2 id=\"foo-bar_baz-qux\" class=\"section-\
          header\"><a href=\"#foo-bar_baz-qux\">Foo-bar_baz qux</a></h2>");
        t("### **Foo** *bar* baz!?!& -_qux_-%",
          "<h3 id=\"foo-bar-baz--qux-\" class=\"section-header\">\
          <a href=\"#foo-bar-baz--qux-\"><strong>Foo</strong> \
          <em>bar</em> baz!?!&amp; -<em>qux</em>-%</a></h3>");
        t("#### **Foo?** & \\*bar?!*  _`baz`_ ❤ #qux",
          "<h4 id=\"foo--bar--baz--qux\" class=\"section-header\">\
          <a href=\"#foo--bar--baz--qux\"><strong>Foo?</strong> &amp; *bar?!*  \
          <em><code>baz</code></em> ❤ #qux</a></h4>");
    }

    #[test]
    fn test_header_ids_multiple_blocks() {
        fn t(input: &str, expect: &str) {
            let output = format!("{}", Markdown(input));
            assert_eq!(output, expect, "original: {}", input);
        }

        let test = || {
            t("# Example", "<h1 id=\"example\" class=\"section-header\">\
              <a href=\"#example\">Example</a></h1>");
            t("# Panics", "<h1 id=\"panics\" class=\"section-header\">\
              <a href=\"#panics\">Panics</a></h1>");
            t("# Example", "<h1 id=\"example-1\" class=\"section-header\">\
              <a href=\"#example-1\">Example</a></h1>");
            t("# Main", "<h1 id=\"main-1\" class=\"section-header\">\
              <a href=\"#main-1\">Main</a></h1>");
            t("# Example", "<h1 id=\"example-2\" class=\"section-header\">\
              <a href=\"#example-2\">Example</a></h1>");
            t("# Panics", "<h1 id=\"panics-1\" class=\"section-header\">\
              <a href=\"#panics-1\">Panics</a></h1>");
        };
        test();
        reset_ids(true);
        test();
    }

    #[test]
    fn test_plain_summary_line() {
        fn t(input: &str, expect: &str) {
            let output = plain_summary_line(input);
            assert_eq!(output, expect, "original: {}", input);
        }

        t("hello [Rust](https://www.rust-lang.org) :)", "hello Rust :)");
        t("hello [Rust](https://www.rust-lang.org \"Rust\") :)", "hello Rust :)");
        t("code `let x = i32;` ...", "code `let x = i32;` ...");
        t("type `Type<'static>` ...", "type `Type<'static>` ...");
        t("# top header", "top header");
        t("## header", "header");
    }

    #[test]
    fn test_markdown_html_escape() {
        fn t(input: &str, expect: &str) {
            let output = format!("{}", MarkdownHtml(input));
            assert_eq!(output, expect, "original: {}", input);
        }

        t("`Struct<'a, T>`", "<p><code>Struct&lt;'a, T&gt;</code></p>\n");
        t("Struct<'a, T>", "<p>Struct&lt;'a, T&gt;</p>\n");
        t("Struct<br>", "<p>Struct&lt;br&gt;</p>\n");
    }
}

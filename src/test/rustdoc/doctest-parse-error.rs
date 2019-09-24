// TODO: maybe merge with bad-codeblock-syntax

#![crate_name = "foo"]
#![doc(html_playground_url = "https://play.rust-lang.org/")]

/// ```
/// "fail
/// ```
pub fn f() {}

// @has foo/fn.f.html //pre '"fail'

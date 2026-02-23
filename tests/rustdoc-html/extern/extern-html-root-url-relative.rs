//@ compile-flags:-Z unstable-options --extern-html-root-url core=../ --extern-html-root-takes-precedence

// At depth 1 (top-level), the href should be ../core/...
//@ has extern_html_root_url_relative/index.html
//@ has - '//a/@href' '../core/iter/index.html'
#[doc(no_inline)]
pub use std::iter;

// At depth 2 (inside a module), the href should be ../../core/...
pub mod nested {
    //@ has extern_html_root_url_relative/nested/index.html
    //@ has - '//a/@href' '../../core/iter/index.html'
    #[doc(no_inline)]
    pub use std::iter;
}

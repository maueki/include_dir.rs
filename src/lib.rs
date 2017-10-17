#![crate_type="dylib"]
#![feature(plugin_registrar, rustc_private)]

extern crate syntax;
extern crate syntax_pos;
extern crate rustc;
extern crate rustc_plugin;

use syntax::ast;
use syntax::tokenstream::TokenTree;
use syntax::ext::base::{ExtCtxt, MacResult, MacEager};
use syntax::ext::build::AstBuilder;
use syntax_pos::Span;
use syntax_pos::symbol::Symbol;
use rustc_plugin::Registry;

fn expand_include_dir(cx: &mut ExtCtxt, sp: Span, _args: &[TokenTree])
                 -> Box<MacResult + 'static> {
    MacEager::expr(cx.expr_lit(sp, ast::LitKind::Str(Symbol::intern("hoge"), ast::StrStyle::Cooked)))
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("include_dir", expand_include_dir);
}

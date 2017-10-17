#![crate_type="dylib"]
#![feature(plugin_registrar, quote, rustc_private)]

extern crate syntax;
extern crate syntax_pos;
extern crate rustc;
extern crate rustc_plugin;
extern crate glob;

use syntax::ast;
use syntax::ast::Ident;
use syntax::tokenstream::TokenTree;
use syntax::ext::base::{ExtCtxt, MacResult, DummyResult, MacEager};
use syntax::ext::build::AstBuilder;
use syntax_pos::Span;
use syntax_pos::symbol::Symbol;
use rustc_plugin::Registry;
use syntax::ext::base::*;

use std::fs::{File, metadata};
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use self::glob::glob;
use std::rc::Rc;

fn expand_include_dir(cx: &mut ExtCtxt, sp: Span, tts: &[TokenTree]) -> Box<MacResult + 'static> {
    let dir = match get_single_str_from_tts(cx, sp, tts, "include_dir!") {
        Some(d) => d,
        None => return DummyResult::expr(sp),
    };
    let dir = res_rel_file(cx, sp, Path::new(&dir));

    let expr_new_map = quote_expr!(cx, std::collections::HashMap::new());
    let stmt_let_map = quote_stmt!(cx, let mut map: std::collections::HashMap<String, Vec<u8>> = $expr_new_map;)
        .unwrap();

    let mut stmts = vec![];
    stmts.push(stmt_let_map);

    let mut glob_dir = dir.clone();
    glob_dir.push("**/*");
    let paths = glob(glob_dir.to_str().unwrap()).unwrap();
    for path in paths {
        if let Ok(path) = path {
            let stat = metadata(&path).unwrap();
            if stat.is_file() {
                let mut bytes = Vec::new();
                match File::open(&path).and_then(|mut f| f.read_to_end(&mut bytes)) {
                    Err(e) => {
                        cx.span_err(sp, &format!("couldn't read {}: {}", path.display(), e));
                        return DummyResult::expr(sp);
                    }
                    Ok(..) => {
                        let filename = format!("{}", path.display());
                        cx.codemap().new_filemap_and_lines(&filename, "");

                        let lit_bytes = cx.expr_lit(sp, ast::LitKind::ByteStr(Rc::new(bytes)));

                        let stripped = path.strip_prefix(&dir).unwrap();
                        println!("file : {}", stripped.display());
                        let lit_path = cx.expr_lit(
                            sp,
                            ast::LitKind::Str(
                                Symbol::intern(stripped.to_str().unwrap()),
                                ast::StrStyle::Cooked,
                            ),
                        );

                        // FIXME: 
                        stmts.push(quote_stmt!(cx, let _ = map.insert($lit_path.to_string(), $lit_bytes.to_vec());).unwrap());
                    }
                };
            }
        }
    }

    stmts.push(quote_stmt!(cx, map).unwrap());

    let block = cx.expr_block(cx.block(sp, stmts));

    MacEager::expr(block)
}

// resolve a file-system path to an absolute file-system path (if it
// isn't already)
fn res_rel_file(cx: &mut ExtCtxt, sp: Span, arg: &Path) -> PathBuf {
    // NB: relative paths are resolved relative to the compilation unit
    if !arg.is_absolute() {
        let mut cu = PathBuf::from(&cx.codemap().span_to_filename(sp));
        cu.pop();
        cu.push(arg);
        cu
    } else {
        arg.to_path_buf()
    }
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("include_dir", expand_include_dir);
}

#![feature(plugin)]
#![plugin(include_dir)]

#[test] fn include_dir_test() {
    let hoge = include_dir!();
    assert_eq!(hoge, "hoge");
}

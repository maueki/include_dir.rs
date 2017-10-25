#![feature(plugin)]
#![plugin(include_dir_bytes)]

#[test] fn include_dir_test() {
    let file_map = include_dir!("");
    let content: String = String::from_utf8(file_map.get(std::path::Path::new("dir/a.txt")).unwrap().to_vec()).unwrap();
    assert_eq!(content, "hoge\n");
}

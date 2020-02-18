use proc_macro_sample::SelfName;

#[derive(Debug, Clone, PartialEq, SelfName)]
struct Hoge {}

#[test]
fn test_hoge() {
    let hoge = Hoge {};
    assert_eq!(hoge.self_name(), "Hoge");
}

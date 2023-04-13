extern crate dex_parser;

#[test]
fn test_multidex() {
    let parsed = dex_parser::parse_multidex("tests/multidex.apk");

    let mut many_methods_count = 0;
    for method in parsed.methods.items.iter() {
        if method.starts_with("Lcom/example/multidextestapp/manymethods/") {
            many_methods_count += 1;
        }
    }

    /* 80 classes with 1 constructor and 100 methods each */
    assert_eq!(many_methods_count, 80080);
}

extern crate dex_parser;

#[test]
/// Simple test to make sure we get all the methods
/// for apps using multidex
///
/// This test simply parse the test APK and checks the
/// number of methods in the `manymethods` package.
fn test_multidex() {
    let parsed = dex_parser::parse("tests/multidex.apk");

    let mut many_methods_count = 0;
    for method in parsed.methods.items.iter() {
        if method.starts_with("Lcom/example/multidextestapp/manymethods/") {
            many_methods_count += 1;
        }
    }

    /* 80 classes with 100 methods plus 1 constructor each */
    assert_eq!(many_methods_count, 80080);
}

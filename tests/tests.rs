use string_literal_const_replace::string_literal_replace;

#[test]
fn test_macro_inputs() {
    assert_eq!(string_literal_replace!("hello, world!"), "hello, world!");
    assert_eq!(
        string_literal_replace!("hello, world!" ("hello" -> "goodbye")),
        "goodbye, world!"
    );
    assert_eq!(
        string_literal_replace!(stringify!(5 < 7) ("<" -> "&lt;")),
        "5 &lt; 7"
    );
}

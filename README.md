string-literal-const-replace
---

This is a proc macro which allows you to, at compile time, perform find/replace on string literals.
This macro is intended to be called by other macros, to do processing on inputs provided to the
macro.

Example of use:

```rust
assert_eq!(
    string_literal_replace!("hello, world!" ("hello" -> "goodbye")),
    "goodbye, world!"
);
```

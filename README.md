string-literal-const-replace
---

This is a proc macro which allows you to, at compile time, perform find/replace on string literals.
This macro is intended to be called by other macros, to do processing on inputs provided to the
macro.

Note that this macro is nightly-only as it relies on the
[`proc_macro_expand`](https://github.com/rust-lang/rust/issues/90765) unstable feature.

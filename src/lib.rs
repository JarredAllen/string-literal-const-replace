//! A macro for compile-time replacement on string literals.
//!
//! This is meant for combining with other macros, such as doing a find/replace on a `stringify!`
//! result in a const context.
//!
//! See [`string_literal_replace!`].

#![feature(proc_macro_expand)]

use proc_macro::{Literal, Span, TokenStream, TokenTree};

/// Replace contents of a string literal at compile-time.
///
/// ```
/// # use string_literal_const_replace::string_literal_replace;
/// assert_eq!(string_literal_replace!("hello, world!" ("hello" -> "goodbye")), "goodbye, world!");
/// ```
///
/// This macro can chain with other macros that output string literals, like [`concat!`]:
///
/// ```
/// # use string_literal_const_replace::string_literal_replace;
/// assert_eq!(
///     string_literal_replace!(concat!("hello", ", world!") ("hello" -> "goodbye")),
///     "goodbye, world!"
/// );
/// ```
#[proc_macro]
pub fn string_literal_replace(input: TokenStream) -> TokenStream {
    let ParsedInput {
        original_string,
        replacements,
    } = syn::parse_macro_input!(input as ParsedInput);
    let mut processed_str = original_string.clone();
    for (from, to) in replacements {
        processed_str = processed_str.replace(&from, &to);
    }
    let mut lit = Literal::string(&processed_str);
    lit.set_span(Span::call_site());
    TokenTree::Literal(lit).into()
}

/// The macro input, parsed.
struct ParsedInput {
    /// The original, pre-replacements string.
    original_string: String,
    /// The replacements being used.
    replacements: Vec<(String, String)>,
}

impl syn::parse::Parse for ParsedInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let original_string = input.parse::<StringExpander>()?.into();
        let mut replacements = Vec::new();
        while let Ok(group) = input.parse::<proc_macro2::Group>() {
            let Replacement { from, to } = syn::parse2(group.stream())?;
            replacements.push((from, to));
        }
        Ok(Self {
            original_string,
            replacements,
        })
    }
}

/// A single replacement, parsed.
struct Replacement {
    /// The text to search for.
    from: String,
    /// The text to replace with.
    to: String,
}
impl syn::parse::Parse for Replacement {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let from = input.parse::<StringExpander>()?.into();
        input.parse::<syn::Token![->]>()?;
        let to = input.parse::<StringExpander>()?.into();
        Ok(Self { from, to })
    }
}

/// A single string literal, parsed and maybe expanded.
struct StringExpander {
    /// The parsed and maybe expanded string literal.
    expanded: String,
}
impl From<StringExpander> for String {
    fn from(value: StringExpander) -> Self {
        value.expanded
    }
}
impl syn::parse::Parse for StringExpander {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        use quote::ToTokens;

        if let Ok(expr) = input.parse::<syn::ExprMacro>() {
            let expression = TokenStream::from(expr.into_token_stream());
            Ok(Self {
                expanded: syn::parse::<syn::LitStr>(
                    expression.expand_expr().map_err(|e| input.error(e))?,
                )?
                .value(),
            })
        } else {
            Ok(Self {
                expanded: input.parse::<syn::LitStr>()?.value(),
            })
        }
    }
}

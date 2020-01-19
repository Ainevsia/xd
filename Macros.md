Macro, a family of features in Rust: declarative macros with `macro_rules!` and three kinds of *procedural macros*:

1. Custom `#[derive]` macros that specify code added with the derive attribute used on structs and enums
2. Attribute-like macros that define custom attributes usable on any item
3. Function-like macros that look like function calls but operate on the tokens specified as their argument

## Declarative Macros with `macro_rules!` for General Metaprogramming

```rust
#[macro_export] // what's this
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
```

`#[macro_export]` means once this crate is brought into scope, this macro will be available. (what's this annotation)

After that update, `macro_rules!` will be effectively deprecated. Most Rust programmers will use macros more than write macros

## Procedural Macros for Generating Code from Attributes

```rust
use proc_macro;

#[some_attribute]
pub fn some_name(input: TokenStream) -> TokenStream {
}
```

When creating procedural macros, the definitions must reside in their own crate with a special crate type. `some_attribute` is a placeholder for using a specific macro.

The function that defines a procedural macro takes a TokenStream as an input and produces a TokenStream as an output. The TokenStream type is defined by the proc_macro crate that is included with Rust and represents a sequence of tokens. This is the core of the macro: the source code that the macro is operating on makes up the input TokenStream, and the code the macro produces is the output TokenStream. The function also has an attribute attached to it that specifies which kind of procedural macro we’re creating. We can have multiple kinds of procedural macros in the same crate.

use `#[derive(Debug)]` because Debug trait does not have default implimentation for method `fmt`, which is impossible if your stuct is like `{a: i32, b: i32, ... (unknown vars)}`. But macros can do that.

At the time of this writing, procedural macros need to be in their **own crate**.
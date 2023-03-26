extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn my_macro(_input: TokenStream) -> TokenStream {
    // Here you could perform your checks and generate an error message
    // in the `error_msg` variable if needed.
    let error_msg = "My error message".to_string();

    // Generate Rust code that uses `compile_error!` to display the error message.
    let error_tokens = quote! {
        compile_error!(#error_msg);
    };

    // Return the error message as a compile-time error.
    error_tokens.into()
}

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(AnswerFn)]
pub fn derive_answer_fn(input: TokenStream) -> TokenStream {
    let tree = syn::parse(input).unwrap();
    impl_answerfn(&tree)
}

fn impl_answerfn(tree: &syn::DeriveInput) -> TokenStream {
    let name = &tree.ident;
    let gen = quote! {
        impl AnswerFn for #name {
            fn answer(&self) -> i32 {
                42
            }
        }
    };
    gen.into()
}


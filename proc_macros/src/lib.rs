extern crate proc_macro;
use proc_macro::TokenStream;

// aim: To create a macro which implements the AnswerFn trait
#[proc_macro_derive(AnswerFn)]
pub fn derive_answer_fn(item: TokenStream) -> TokenStream {
    item
}

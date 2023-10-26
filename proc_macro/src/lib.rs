extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_derive(AnswerFn)]
pub fn derive_answer_fn(item: TokenStream) -> TokenStream {
    let name = item.to_string();
    let mut code = format!("impl {} ", name);
    code += "{fn answer() -> u32 { 42 }}";

    code.parse().unwrap()
}


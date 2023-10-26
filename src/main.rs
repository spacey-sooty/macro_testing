use proc_macrobrrrr::{make_answer, AnswerFn, HelperAttr, show_streams};

#[macro_export]
macro_rules! worse_vec {
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

#[derive(HelperAttr)]
struct Struct {
    #[helper] lol: (),
}

#[derive(AnswerFn)]
struct Hello;

make_answer!();

fn main() {
    let hello = Hello;
    let vec = worse_vec!["hello", "joe"];
    println!("{:#?}", vec);
    println!("{}", hello.answer());
    println!("{}", answer());

    // Example: Basic function
    #[show_streams]
    fn invoke1() {}
    // out: attr: ""
    // out: item: "fn invoke1() {}"

    // Example: Attribute with input
    #[show_streams(bar)]
    fn invoke2() {}
    // out: attr: "bar"
    // out: item: "fn invoke2() {}"

    // Example: Multiple tokens in the input
    #[show_streams(multiple => tokens)]
    fn invoke3() {}
    // out: attr: "multiple => tokens"
    // out: item: "fn invoke3() {}"

    // Example:
    #[show_streams { delimiters }]
    fn invoke4() {}
    // out: attr: "delimiters"
    // out: item: "fn invoke4() {}"
}


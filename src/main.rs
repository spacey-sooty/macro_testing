mod map;
mod answer;

use proc_macros::AnswerFn;
use answer::AnswerFn;

#[derive(AnswerFn)]
struct Hello;

fn main() {
    let map = map!([1, 2], ["hello", "world"]);
    for (key, value) in map {
        println!("\n{:#?}: {:#?}", key, value);
    }

    let hello = Hello;
    println!("\nThe answer is {}", hello.answer());
}


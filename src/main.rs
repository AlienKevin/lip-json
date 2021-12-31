use lip::{display_error, ParseResult, Parser};
use lip_json::value;

fn main() {
    let simple = include_str!("../data/wrong/delimiter.json");
    match value().run(simple, ()) {
        ParseResult::Ok { output: v, .. } => println!("{:?}", v),
        ParseResult::Err {
            message, from, to, ..
        } => println!("{}", display_error(simple, message, from, to)),
    }
}

use super::{value, Value::*};
use lip::{display_error, ParseResult, Parser};
use std::collections::HashMap;

#[test]
fn json_test() {
    let input = r#"{
    "array": [1, ""],
    "object": {},
    "string": "\u0041\u0042\nn!?@#$%^\t&*-=_+[]{}\\印刷機🙏👩‍🔬👩🏿‍💻",
    "number": 3.14,
    "small_number": 0.59,
    "int": -100,
    "exp": -1e2,
    "exp_neg": 23e-2,
    "true": true,
    "false"  : false,
    "null" : null
}"#;
    let result = value().run(input, ());
    let expected = VObject(
        vec![
            ("array", VArray(vec![VNumber(1.0), VString("".to_string())])),
            ("object", VObject(HashMap::new())),
            ("number", VNumber(3.14)),
            (
                "string",
                VString("\u{0041}\u{0042}\nn!?@#$%^\t&*-=_+[]{}\\印刷機🙏👩‍🔬👩🏿‍💻".to_string()),
            ),
            ("small_number", VNumber(0.59)),
            ("int", VNumber(-100.)),
            ("exp", VNumber(-1e2)),
            ("exp_neg", VNumber(23E-2)),
            ("true", VBool(true)),
            ("false", VBool(false)),
            ("null", VNull),
        ]
        .into_iter()
        .map(|(k, v)| (k.to_string(), v))
        .collect(),
    );
    match result {
        ParseResult::Ok { output, .. } => assert_eq!(output, expected),
        ParseResult::Err {
            message, from, to, ..
        } => {
            println!("{}", display_error(input, message, from, to));
            panic!();
        }
    }
}

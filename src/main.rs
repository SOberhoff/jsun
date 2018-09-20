extern crate json;

use json::JsonValue;
use std::io::Read;

fn main() {
    let parse = |input: String|
        json::parse(input.as_str()).map_err(|e| format!("error: invalid JSON passed to jsun\n{}", e));

    match get_input().and_then(parse).map(unquote) {
        Ok(json_value) => print!("{}", json_value.pretty(2)),
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1)
        }
    }
}

/// get the first os arg if there is one, otherwise consume stdin
fn get_input() -> Result<String, String> {
    let mut os_args: Vec<String> = std::env::args().collect();
    match os_args.len() - 1 {
        0 => {
            let mut buf = std::string::String::new();
            std::io::stdin().read_to_string(&mut buf).unwrap();
            Ok(buf)
        }
        1 => Ok(os_args.remove(1)),
        n => Err(format!("error: too many arguments ({}) passed to jsun", n)),
    }
}

/// recursively implemented unquoting function
fn unquote(json_value: JsonValue) -> JsonValue {
    match json_value {
        // if it's an object, recurse on the object values
        JsonValue::Object(json_object) => {
            let mut new_object = json::JsonValue::new_object();
            for (key, val) in json_object.iter() {
                // I haven't figured out how to take ownership of the object entries, so we have to clone :(
                new_object[key] = unquote(val.clone())
            }
            new_object
        }
        // if it's an array, recurse on the array entries
        JsonValue::Array(json_entries) => {
            JsonValue::Array(json_entries.into_iter().map(unquote).collect())
        }
        // if it's a string, attempt to parse the string, see if it's valid json, and if it is, recurse on that
        JsonValue::String(json_string) =>
            match json::parse(json_string.as_str()) {
                Ok(unquoted) => unquote(unquoted),
                Err(_) => JsonValue::String(json_string),
            }
        _ => json_value
    }
}
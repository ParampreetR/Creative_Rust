use serde_json::{ Number, Value };

fn main() {
    // Taking arguments from user
    let config_file_path = std::env::args().nth(1).unwrap();
    let output_path = std::env::args().nth(2).unwrap();

    // Reading json file and parsing it
    let mut configs = {
        let config_file_contents = std::fs::read_to_string(&config_file_path).unwrap();
        serde_json::from_str::<Value>(&config_file_contents).unwrap()
    };

    // Retrieve field from parsed data. It is a number so Value::Number() is used.
    if let Value::Number(n) = &configs["sales"][0]["quantity"] {
        // Increment field by 1.5
        configs["sales"][0]["quantity"] = Value::Number(Number::from_f64(n.as_f64().unwrap() + 1.5).unwrap());
    }
    
    // Write to destination file with some prettify with serde_json::to_string_pretty()
    std::fs::write(
        output_path,
        serde_json::to_string_pretty(&configs).unwrap()
    ).unwrap();
}

extern crate toml;



fn main() {
    let config_values = {
        let path_to_config = std::env::args().nth(1).unwrap();
        let config_file_content = std::fs::read_to_string(&path_to_config).unwrap();
        config_file_content.parse::<toml::Value>().unwrap()        
    };

    println!("{:#?}", config_values);
    println!("Get Result {}", config_values.get("postgresql").unwrap().get("database").unwrap());

}

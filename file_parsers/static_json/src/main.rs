use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct Products {
    id: u32,
    category: String,
    name: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Sales {
    id: String,
    product_id: u32,
    date: u64,
    quantity: f64,
    unit: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Report {
    products: Vec<Products>,
    sales: Vec<Sales>,
}


fn main() -> Result<(), std::io::Error>{
    let config_file_path = std::env::args().nth(1).unwrap();
    let destination_path = std::env::args().nth(2).unwrap();

    let mut configs: Report = {
        let config_file_contents = std::fs::read_to_string(&config_file_path)?;
        serde_json::from_str::<Report>(&config_file_contents).unwrap()
    };

    configs.sales[0].quantity += 1.5;
    std::fs::write(
        destination_path,
        serde_json::to_string_pretty(&configs).unwrap()
        ).unwrap();

    Ok(())
}

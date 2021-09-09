use postgres::{ error::Error, Client, NoTls };

#[derive(Debug)]
struct SaleWithProduct {
    category: String,
    name: String,
    quantity: f64,
    unit: String,
    date: i64,
}

fn connect_db() -> Result<Client,Error> {
    let mut conn = Client::connect("postgres://postgres:mypass123@localhost/myrust", NoTls)?;
    conn.execute("DROP TABLE Sales", &[])?;
    conn.execute("DROP TABLE Products", &[])?;
    conn.execute(
        "CREATE TABLE Products (
            id INTEGER PRIMARY KEY,
            category TEXT NOT NULL,
            name TEXT NOT NULL UNIQUE)",
        &[],
    )?;
    conn.execute(
        "CREATE TABLE Sales (
            id TEXT PRIMARY KEY,
            product_id INTEGER NOT NULL REFERENCES Products,
            sale_date BIGINT NOT NULL,
            quantity DOUBLE PRECISION NOT NULL,
            unit TEXT NOT NULL)",
        &[],
    )?;
    Ok(conn)
}

fn store_data(conn: &mut Client) -> Result<(), Error> {
    conn.execute(
        "INSERT INTO Products (
            id, category, name
            ) VALUES ($1, $2, $3)",
        &[&1, &"fruit", &"pears"],
    )?;
    conn.execute(
        "INSERT INTO Sales (
            id, product_id, sale_date, quantity, unit
            ) VALUES ($1, $2, $3, $4, $5)",
        &[&"2020-183", &1, &1_234_567_890_i64, &7.439, &"Kg"],
    )?;
    Ok(())
}

fn print_db(conn: &mut Client) -> Result<(), Error> {
    for row in &conn.query(
        "SELECT p.name, s.unit, s.quantity, s.sale_date
        FROM Sales s
        LEFT JOIN Products p
        ON p.id = s.product_id
        ORDER BY s.sale_date",
        &[],
    )? {
        let sale_with_product = SaleWithProduct{
            category: "".to_string(),
            name: row.get(0),
            quantity: row.get(2),
            unit: row.get(1),
            date: row.get(3),
        };
        println!(
            "At instant {}, {} {} of {} were sold.",
            sale_with_product.date,
            sale_with_product.quantity,
            sale_with_product.unit,
            sale_with_product.name
        );
    }
    Ok(())
}

fn main() {
    let mut conn = match connect_db() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("=[_-_]> Error in connecting to database");
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    store_data(&mut conn).unwrap();
    print_db(&mut conn).unwrap();
}

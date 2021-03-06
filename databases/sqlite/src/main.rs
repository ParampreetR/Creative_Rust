use rusqlite::{ Connection, Result, params };

#[derive(Debug)]
struct SalesWithProducts {
    category: String,
    name: String,
    quantity: f64,
    unit: String,
    date: i64,
}


fn connect_db() -> Result<Connection> {
    let conn = Connection::open("data.db")?;
    //let _ = conn.execute("DROP TABLE Sales", params![])?;
    //let _ = conn.execute("DROP TABLE Products", params![])?;

    conn.execute(
        "CREATE TABLE Products (
            id INTEGER PRIMARY KEY,
            category TEXT NOT NULL,
            name TEXT NOT NULL UNIQUE)",
            params![]
        )?;

    conn.execute(
        "CREATE TABLE Sales (
            id TEXT PRIMARY KEY,
            product_id INTEGER NOT NULL REFERENCES Products,
            sale_date BIGINT NOT NULL,
            quantity DOUBLE PRECISION NOT NULL,
            unit TEXT NOT NULL)",
            params![]
        )?;

    Ok(conn)
}

fn populate_db(conn: &Connection) -> Result<()> {
    conn.execute(
        "INSERT INTO Products (
            id, category, name
            ) VALUES ($1, $2, $3)",
        params![1, "fruit", "pears"],
    )?;
    conn.execute(
        "INSERT INTO Sales (
            id, product_id, sale_date, quantity, unit
            ) VALUES ($1, $2, $3, $4, $5)",
        params!["2020-183", 1, 1_234_567_890_i64, 7.439, "Kg",],
    )?;
    Ok(())
}

fn print_result(conn: &Connection) -> Result<()> {
    let mut command = conn.prepare(
        "SELECT p.name, s.unit, s.quantity, s.sale_date
        FROM Sales s
        LEFT JOIN Products p
        ON p.id = s.product_id
        ORDER BY s.sale_date",
    )?;

    for query_result in command.query_map(params![], |row| {
        Ok(SalesWithProducts {
            category: "".to_string(),
            name: row.get(0)?,
            quantity: row.get(2)?,
            unit: row.get(1)?,
            date: row.get(3)?
        })
    })? {
        if let Ok(sale) = query_result {
            println!("Product {} sold {} {} on {}", sale.name, sale.quantity, sale.unit, sale.date);
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    let conn: Connection = connect_db()?;
    populate_db(&conn)?;
    print_result(&conn)?;
    Ok(())
}

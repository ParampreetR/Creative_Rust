use redis::Commands;

fn main() -> redis::RedisResult<()>{
    let client = redis::Client::open("redis://localhost/")?;
    let mut conn = client.get_connection()?;
    conn.set("myNumber", 123)?;
    conn.set("key", 1212)?;
    conn.set("name", "param")?;

    println!("{} {} {} {}",
        conn.get::<_, u32>("myNumber")?,
        conn.get::<_, i64>("key")?,
        conn.get::<_, String>("name")?,
        conn.exists::<_, bool>(12)?
        );
    Ok(())
}

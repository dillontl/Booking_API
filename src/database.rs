use mysql::*;

pub(crate) fn db_connection() -> Result<PooledConn, Box<dyn std::error::Error>> {
    let db_url = "mysql://admin:password@localhost:3306/BookingApplicationDB";
    let db_pool = Pool::new(db_url)?;
    let conn = db_pool.get_conn()?;
    println!("Connected to database successfully. 👍");
    Ok(conn)
}
use dotenv_codegen::dotenv;
use mysql::*;
extern crate dotenv;

pub(crate) fn db_connection() -> Result<PooledConn, Box<dyn std::error::Error>> {
    let db_url = dotenv!("DB_CONN_STR");
    let db_pool = Pool::new(db_url)?;
    let conn = db_pool.get_conn()?;
    Ok(conn)
}
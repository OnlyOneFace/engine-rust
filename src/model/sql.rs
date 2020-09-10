use mysql_async::Pool;

static mut CONN_POOL: Pool = init_sql();

pub fn get_sql() -> Pool {
    unsafe { &*CONN_POOL }
}

fn init_sql() -> Pool {
    let sql_url = format!("mysql://{}:{}@{}:{}/{}?pool_min={}&pool_max={}&conn_ttl=10",
                          "root",
                          "123456",
                          "192.168.31.62",
                          3306,
                          "identity_information",
                          10,
                          100,
    );
    Pool::new(sql_url)
}
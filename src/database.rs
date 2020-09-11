use once_cell::sync::OnceCell;
use sqlx::Pool;
use sqlx::mysql::{MySql, MySqlPoolOptions};

use log::{error, info};

pub static MYSQL_POOL: OnceCell<Pool<MySql>> = OnceCell::new();

pub fn init_db_pool() {
    // mysql://root:root@127.0.0.1:3306/testdb
    let sql_url = format!("mysql://{}:{}@{}:{}/{}?charset={}&parseTime=True&loc=Local",
                          "root",
                          "123456",
                          "192.168.31.62",
                          3306,
                          "identity_information",
                          "utf8",
    );
    match MySqlPoolOptions::new()
        .min_connections(15)
        .max_connections(30)
        .connect_lazy(sql_url.as_str()) {
        Ok(pool) => {
            MYSQL_POOL.set(pool);
            info!("set mysql pool success");
        }
        Err(e) => {
            error!("build mysql pool failed.Error:{}", e);
            panic!("{}", e)
        }
    }
}

pub fn get_pool() -> Option<&'static Pool<MySql>> {
    MYSQL_POOL.get()
}
use sqlx::{MySqlPool, MySqlConnection, Pool};
use std::time;
use log::{info, error};
use std::sync::{Arc, Mutex};

pub fn get_sql() -> Arc<Mutex<Pool<MySqlConnection>>> {
    static mut CONN_POOL: Option<Arc<Mutex<Pool<MySqlConnection>>>> = None;

    unsafe {// Rust中使用可变静态变量都是unsafe的
        CONN_POOL.get_or_insert_with(|| {
            // 初始化单例对象的代码
            Arc::new(Mutex::new(init_sql()))
        }).clone()
    }
}

fn init_sql() -> Pool<MySqlConnection> {
    let sql_url = format!("%{}:%{}@tcp(%{}:%{})/%{}?charset=%v&parseTime=True&loc=Local",
                          "root",
                          "123456",
                          "192.168.31.62",
                          3306,
                          "identity_information"
    );
    info! {"{}", sql_url};
    let pool = MySqlPool::builder().
        max_size(100).
        min_size(10).
        connect_timeout(time::Duration::from_secs(10)).
        max_lifetime(time::Duration::from_secs(1800)).
        idle_timeout(time::Duration::from_secs(600)).
        build(sql_url.as_str()).await;
    match p {
        Ok(conn) => conn,
        Err(e) => {
            let msg = format!("mysql connect failed.Error:{:?}", e);
            error! {"{}", msg};
            panic!("{}", msg)
        }
    }
}
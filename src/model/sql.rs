use mysql_async::Pool;
use std::sync::Arc;
use log::error;

static mut CONN_POOL: Option<Arc<Pool>> = None;

pub fn get_pool<'staic>() -> &'staic Pool {
    unsafe {// Rust中使用可变静态变量都是unsafe的
        CONN_POOL.get_or_insert_with(|| {
            // 初始化单例对象的代码
            Arc::new({
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
            })
        });
        match CONN_POOL.as_ref() {
            Some(p) => p,
            None => {
                error!("get conn pool failed.value is None");
                panic!("called `Option::unwrap()` on a `None` value")
            }
        }
    }
}
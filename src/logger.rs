use log::LevelFilter;
use log4rs::{
    append::file::FileAppender,
    encode::json::JsonEncoder,
    config::{Appender, Config, Root},
};

pub fn init() {
    let requests = FileAppender::builder()
        // encoder按什么格式输出，在appender中配置
        .encoder(Box::new(JsonEncoder::new()))
        .build("log/engine.log")
        .expect("build request log failed.Error");

    let config = Config::builder()
        //appender控制输出到什么地方去，例子添加了输出到文件
        .appender(Appender::builder().build("rs", Box::new(requests)))
        // 构建了一个全局日志实例
        .build(Root::builder().appender("rs").build(LevelFilter::Debug))
        .expect("build log config failed.Error");

    log4rs::init_config(config).expect("init log failed.Error");
}
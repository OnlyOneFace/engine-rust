mod logger;

use log::info;

fn main() {
    logger::init();
    info!("run ...");
}
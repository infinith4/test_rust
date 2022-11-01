mod logLib;
use chrono::{DateTime, Local};
use log::error;
use std::fmt;

use crate::logLib::LogTrait;

fn main() {
    println!("Hello, world!");
    //https://qiita.com/fujitayy/items/ae6175118cbed7134594
    let local_datetime = Local::now().format("%Y%m%d").to_string(); //Local::now().to_rfc3339().to_string();
    let local_datetime_str: String = String::from(local_datetime);
    let local_datetime_str2: &str = &local_datetime_str;
    //println!("aaaa{}", str2);
    let file_name = format!("{}.log", local_datetime_str2);
    let logLib = logLib::LogLib::new(&file_name);
    let logger = logLib::Logger {};
    logger.error("testtest");
    //error!("Bright red error");
    // info!("This only appears in the log file");
    // debug!("This level is currently not enabled for any logger");
}

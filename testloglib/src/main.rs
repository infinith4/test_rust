mod logLib;

fn main() {
    println!("Hello, world!");
    let file_name: &str = "test20221028.log";
    let logLib = LogLib::new(file_name);
    // error!("Bright red error");
    // info!("This only appears in the log file");
    // debug!("This level is currently not enabled for any logger");
}

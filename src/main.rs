extern crate chrono;
extern crate dbcon;
extern crate fspec;
use chrono::{DateTime, Utc};
use dbcon::DBConn;
use fspec::file_spec::Filespec;
use std::env;
use std::thread::sleep;
use std::time::Duration;

fn creds() -> (String, String, String, String) {
    (
        env::var("HOTEL_IMPORT_USERNAME").unwrap(),
        env::var("HOTEL_IMPORT_HOST").unwrap(),
        env::var("HOTEL_IMPORT_DB_NAME").unwrap(),
        env::var("HOTEL_IMPORT_PORT").unwrap(),
    )
}

fn main() {
    let started: DateTime<Utc> = Utc::now();
    sleep(Duration::new(1, 0));

    let (n, h, d, p) = creds();
    let supplier = String::from("expedia");
    let source_url =
        String::from("https://www.ian.com/affiliatecenter/include/V2/ActivePropertyList.zip");
    let lfnam = String::from("ActivePropertyList.zip");
    let dirname = String::from("ean_hotels");
    let fs = Filespec::new(supplier, source_url, lfnam, dirname);
    println!("Hello, world!");
    println!("DB creds: {},{},{},{}", n, h, d, p);
    println!("Expedia Filespec: {}", fs);

    println!("\nConnecting to database now...\n");

    let dbport = match p.parse::<u32>() {
        Ok(v) => v,
        Err(err) => panic!("PANIC dbport: {}", err),
    };
    //let conn = match DBConn::make_connection_string(n, h, d, dbport, false) {
    let conn = match DBConn::make_connection_string(n, h, d, dbport, true) {
        Ok(v) => v,
        Err(err) => panic!("PANIC conn string: {}", err),
    };
    println!("Connection string {} \n", conn);
    let dbconn = DBConn::new(conn);
    let db = match dbconn.conn {
        Ok(v) => v,
        Err(err) => {
            println!("ERROR db: {}", err);
            return;
        }
    };
    println!("DB success! {:?}", db);
    let success = match fs.download_file_with_name() {
        Ok(()) => (true, String::from("ok")),
        Err(e) => (false, e.to_string()),
    };
    let re = match fs.register_download(started, success.0, success.1, db) {
        Ok(p) => (p),
        Err(why) => why.to_string(),
    };
    println!("\nregister_download called! {}", re);
}

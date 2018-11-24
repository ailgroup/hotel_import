use std::thread::sleep;
extern crate chrono;
use chrono::prelude::*;
use std::fmt;
use std::time::Duration;

fn main() {
    let started: DateTime<Local> = Local::now();
    let fs = Filespec::new(
        String::from("yoyo"),
        String::from("https://yoyo.com"),
        String::from("yoyo.csv"),
        String::from("yoyodir"),
    );
    //println!("{}", fs);
    sleep(Duration::new(2, 0));
    let re = match fs.register_download(started) {
        Ok(p) => (p),
        Err(why) => why,
    };
    println!("{}", re)
}

struct Filespec {
    supplier: String,
    url: String,
    local_filename: String,
    download_dir: String,
}

// methods
impl Filespec {
    fn register_download(&self, started: DateTime<Local>) -> Result<String, String> {
        //fn register_download(&self, started: DateTime<Local>) {
        let now: DateTime<Local> = Local::now();
        let elapsed = now.signed_duration_since(started).num_seconds();
        /* 
            connect to db; return any error
            insert

            qs := "INSERT INTO download_registers(started_at,finished_at,download_time_seconds,download_target_file,supply_origin_url,supplier) VALUES($1,$2,$3,$4,$5,$6);"

            _, err = conn.DB.Query(
                qs,
                start, timestamp...
                time.Now(), timestamp..
                elapsed,  in seconds..
                spec.LocalFilename,
                spec.URL,
                spec.Supplier,
            )

        )
        */
        Ok(format!(
            "fspec: {} \n started: {}\n now: {}\n elapsed: {}",
            self, started, now, elapsed
        ))
    }
}

// related functions
impl Filespec {
    fn new(
        supplier: String,
        url: String,
        local_filename: String,
        download_dir: String,
    ) -> Filespec {
        Filespec {
            supplier: supplier,
            url: url,
            local_filename: local_filename,
            download_dir: download_dir,
        }
    }
}

impl fmt::Display for Filespec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "'{}' from '{}' to '{}/{}'",
            self.supplier, self.url, self.download_dir, self.local_filename
        )
    }
}

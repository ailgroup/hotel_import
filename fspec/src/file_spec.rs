extern crate chrono;
extern crate dbcon;
extern crate postgres;
use self::chrono::{DateTime, Utc};
use std::fmt;

pub struct Filespec {
    pub supplier: String,
    pub url: String,
    pub local_filename: String,
    pub download_dir: String,
}

// methods
impl Filespec {
    pub fn register_download(
        &self,
        started: DateTime<Utc>,
        ok_success: bool,
        status_msg: String,
        conn: postgres::Connection,
    ) -> Result<String, postgres::Error> {
        let now: DateTime<Utc> = Utc::now();
        let elapsed = now.signed_duration_since(started).num_seconds();

        let qs = "INSERT INTO register_downloads(started_at,finished_at,success,status_msg,download_time_seconds,download_target_file,supply_origin_url,supplier) VALUES($1,$2,$3,$4,$5,$6,$7,$8);";
        conn.execute(
            qs,
            &[
                &started,
                &now,
                &ok_success,
                &status_msg,
                &elapsed,
                &self.local_filename,
                &self.url,
                &self.supplier,
            ],
        )?;

        Ok(format!(
            "SUCCESS! fspec: {}\n started: {}\n now: {}\n elapsed(s): {}",
            self, started, now, elapsed
        ))
    }
}

// related functions
impl Filespec {
    pub fn new(
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

#[cfg(test)]
mod tests {
    use super::*;
    //use std::thread::sleep;
    //use std::time::Duration;

    fn mock_file_spec() -> Filespec {
        Filespec::new(
            String::from("yoyo"),
            String::from("https://yoyo.com"),
            String::from("yoyo.csv"),
            String::from("yoyodir"),
        )
    }

    #[test]
    fn test_new() {
        let fs = mock_file_spec();
        assert_eq!(String::from("yoyo"), fs.supplier);
        assert_eq!(String::from("https://yoyo.com"), fs.url);
        assert_eq!(String::from("yoyo.csv"), fs.local_filename);
        assert_eq!(String::from("yoyodir"), fs.download_dir);
    }

    #[test]
    fn test_file_spec_formatter() {
        let fs = mock_file_spec();
        assert_eq!(
            format!("{}", fs),
            "'yoyo' from 'https://yoyo.com' to 'yoyodir/yoyo.csv'"
        );
    }

    /*
    #[test]
    fn test_duration() {
        let started: DateTime<Local> = Local::now();
        let fs = mock_file_spec();
        //sleep(Duration::new(1, 0));

        let re = match fs.register_download(started) {
            Ok(p) => (p),
            Err(why) => why.to_string(),
        };
        assert_ne!(re, "Error")
    }
    */
}

extern crate chrono;
extern crate dbcon;
extern crate failure;
extern crate postgres;
extern crate reqwest;
use self::chrono::{DateTime, Utc};
use self::failure::Fallible;
use self::reqwest::{Client, Response, StatusCode, Url};
use std::cell::RefCell;
use std::fmt;
use std::fs;
use std::fs::File;
use std::io::copy;
use std::io::Read;
use std::path::Path;
use std::time::Duration;

/////////////////////////////////////////////////////////////////////////
//https://github.com/mattgathu/duma
#[derive(Debug, Clone)]
pub struct ClientConfig {
    pub resume: bool,
    pub local_filename: String,
    pub download_dir: String,
    pub timeout: Option<Duration>,
    pub concurrent: bool,
    pub max_retries: i32,
    pub num_workers: usize,
    pub chunk_size: u64, //for download chunk size
}
pub struct HttpDownload {
    url: Url,
    handlers: Vec<RefCell<Box<dyn ClientHandlers>>>,
    client_config: ClientConfig,
}

impl fmt::Debug for HttpDownload {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "HttpDownload url: {}", self.url)
    }
}

impl HttpDownload {
    pub fn new(url: Url, client_config: ClientConfig) -> Self {
        HttpDownload {
            url: url,
            handlers: Vec::new(),
            client_config: client_config,
        }
    }
}

impl HttpDownload {
    fn new_client(&self) -> reqwest::Client {
        Client::new()
    }

    fn new_client_with_config(&self, protocol: &str) -> Fallible<(Client)> {
        let mut builder = Client::builder();
        // more options can be added here...
        if let Some(secs) = self.client_config.timeout {
            builder = builder.timeout(secs);
        }
        Ok(builder.build()?)
    }

    pub fn download(&mut self) -> Fallible<()> {
        let client = self.new_client();
        let req = client.get(self.url.clone());
        ////make head request; returned if resource requested with an HTTP GET
        // let head_resp = client.head(self.url.clone()).send()?;
        ////parse headers here if needed....
        // let headers = head_resp.headers();
        let mut resp = req.send()?;

        if resp.status().is_success() {
            if self.client_config.concurrent {
                //TODO implement...
                // self.concurrent_download(client, &headers)?;
            } else {
                self.sequential_download(&mut resp)?;
            }
        } else {
            for handle in &self.handlers {
                handle.borrow_mut().fail_mode(resp.status());
            }
        }

        for handle in &self.handlers {
            handle.borrow_mut().done();
        }

        Ok(())
    }

    fn sequential_download(&mut self, resp: &mut Response) -> Fallible<()> {
        loop {
            let mut buffer = vec![0, self.client_config.chunk_size as u8];
            let bcount = resp.read(&mut buffer[..])?;
            //shift...
            buffer.truncate(bcount);
            if !buffer.is_empty() {
                self.send_content(buffer.as_slice())?;
            } else {
                break;
            }
        }
        Ok(())
    }

    fn send_content(&mut self, content: &[u8]) -> Fallible<()> {
        for handle in &self.handlers {
            handle.borrow_mut().on_content(content)?;
        }
        Ok(())
    }
}

#[allow(unused_variables)]
pub trait ClientHandlers {
    // fn success(&self) {}
    fn resume_download(&mut self, bytes_on_disk: u64) {}
    // fn proc_ftp_content_length(&mut self, ct_len: Option<u64>) {}
    fn max_retries(&mut self) {}
    // fn supports_resume(&mut self) {}

    fn done(&mut self) {}
    fn fail_mode(&self, status_code: StatusCode) {}
    fn on_content(&mut self, content: &[u8]) -> Fallible<()> {
        Ok(())
    }
}
/////////////////////////////////////////////////////////////////

pub struct FileSpec {
    pub supplier: String,
    pub url: String,
    pub local_filename: String,
    pub download_dir: String,
    // pub concurrent: bool,
    // pub max_retries: i32,
    // pub num_workers: usize,
}

pub struct FtpAuth {
    pub file_spec: FileSpec,
    pub username: String,
    pub password: String,
    pub port: String,
}

// methods
impl FileSpec {
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
    pub fn download_file_with_name(&self) -> Result<(), std::io::Error> {
        if !Path::new(&self.download_dir).exists() {
            fs::create_dir(&self.download_dir)?;
        }
        let mut res = match reqwest::get(&self.url) {
            Ok(body) => body,
            Err(e) => return Err(std::io::Error::new(std::io::ErrorKind::Other, e)),
        };
        let mut dest = {
            //println!("file to download: {}", &self.local_filename);
            //let full_path = Path::new(&self.download_dir).join(&self.local_filename);
            //println!("will be located under: {:#?}", full_path);
            File::create(Path::new(&self.download_dir).join(&self.local_filename))?
        };
        copy(&mut res, &mut dest)?;
        Ok(())
    }
}

// related functions
impl FileSpec {
    pub fn new(
        supplier: String,
        url: String,
        local_filename: String,
        download_dir: String,
    ) -> FileSpec {
        FileSpec {
            supplier: supplier,
            url: url,
            local_filename: local_filename,
            download_dir: download_dir,
        }
    }
}

impl fmt::Display for FileSpec {
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

    fn mock_file_spec() -> FileSpec {
        FileSpec::new(
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

/*
#[derive(Debug)]
pub enum DownloadFileError {
    GetRequest(reqwest::Error),
    CreateFile(std::io::Error),
    // NetError(hyper_tls::Error),
    ConnectionClosed,
    IncompleteResponse,
    Internal,
    IncorrectFilepath,
    FileWrite,
}

impl std::fmt::Display for DownloadFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DownloadFileError::GetRequest(_) => write!(f, "ERROR: Get Request"),
            DownloadFileError::CreateFile(e) => write!(f, "ERROR: creating file. {}", e),
            // DownloadFileError::NetError(_) => write!(f, "ERROR: network error"),
            DownloadFileError::ConnectionClosed => write!(f, "ERROR: connection was closed"),
            DownloadFileError::IncompleteResponse => write!(f, "ERROR: incomplete response"),
            DownloadFileError::Internal => write!(f, "ERROR: internal error"),
            DownloadFileError::IncorrectFilepath => write!(f, "ERROR: incorrect file path"),
            DownloadFileError::FileWrite => write!(f, "ERROR: writing to file"),
        }
    }
}

impl std::error::Error for DownloadFileError {
    fn source(&self) -> Option<&(std::error::Error + 'static)> {
        use self::DownloadFileError::*;
        match *self {
            GetRequest(ref err) => Some(err),
            CreateFile(ref err) => Some(err),
            _ => None,
        }
    }
}
*/

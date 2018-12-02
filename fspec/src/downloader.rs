extern crate reqwest;
use file_spec::Filespec;
use std::fs;
use std::fs::File;
use std::io::copy;
use std::path::Path;

impl Filespec {
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

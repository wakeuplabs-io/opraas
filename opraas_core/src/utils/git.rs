use std::io::{self, Cursor, Read};
use std::path::Path;
use mockall::automock;
use reqwest::header::CONTENT_LENGTH;

pub struct Git {
    client: reqwest::blocking::Client,
}

impl Git {
    pub fn new() -> Self {
        Self { client: reqwest::blocking::Client::new() }
    }
}

#[automock]
pub trait GitReleaseDownloader {
    fn download_release(
        &self,
        release_url: &str,
        release_tag: &str,
        destination: &str,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

impl GitReleaseDownloader for Git {
    fn download_release(
        &self,
        release_url: &str,
        release_tag: &str,
        destination: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!("{}/archive/refs/tags/{}.zip", release_url, release_tag);
        let mut response = self.client.get(&url).send()?;
    
        // Get the total size of the file from the response headers
        let total_size = response
            .headers()
            .get(CONTENT_LENGTH)
            .and_then(|h| h.to_str().ok())
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(0);
    
        // Use a stream to read the response body
        let mut bytes = Vec::new();
        let chunk_size = 1024;
    
        loop {
            let mut buffer = vec![0; chunk_size];
            let bcount = response
                .read_vectored(&mut [io::IoSliceMut::new(&mut buffer)])
                .unwrap();
            buffer.truncate(bcount);
            
            if !buffer.is_empty() {
                bytes.extend(buffer.into_boxed_slice().into_vec().iter().cloned());
            } else { break; }
        }

        // Write the file to disk unzipped
        let target = Path::new(destination);
        if !target.exists() {
            std::fs::create_dir_all(target)?;
        }
        zip_extract::extract(Cursor::new(bytes), &target, true)?;

        Ok(())
    }
}



use mockall::automock;
use std::fs;
use std::io::Cursor;
use std::path::Path;

pub struct Git;

impl Git {
    pub fn new() -> Self {
        Self {}
    }
}

#[automock]
pub trait GitReleaseDownloader: Send + Sync {
    fn download_release(
        &self,
        release_url: &str,
        release_tag: &str,
        destination: &str,
    ) -> Result<(), Box<dyn std::error::Error>>;

    fn download_release_zipped_asset(
        &self,
        release_url: &str,
        release_tag: &str,
        asset_name: &str,
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
        download_and_extract(&format!("{}/archive/refs/tags/{}.zip", release_url, release_tag), destination)?;

        Ok(())
    }

    fn download_release_zipped_asset(
        &self,
        release_url: &str,
        release_tag: &str,
        zip_name: &str,
        destination: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        download_and_extract(&format!("{}/releases/download/{}/{}.zip", release_url, release_tag, zip_name), destination)?;

        Ok(())
    }
}

fn download_and_extract(url: &str, destination: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Fetch the response body
    let response = reqwest::blocking::get(url)?;

    // Collect the entire response body into a Vec<u8>
    let bytes = response.bytes()?;

    // Write the file to disk unzipped
    let target = Path::new(destination);
    if !target.exists() {
        fs::create_dir_all(target)?;
    }

    // Extract the zip contents
    zip_extract::extract(Cursor::new(bytes), &target, true)?;

    Ok(())
}

// let url = format!("{}/archive/refs/tags/{}.zip", release_url, release_tag);
// let mut response = self.client.get(&url).send()?;

// // Use a stream to read the response body
// let mut bytes = Vec::new();
// let chunk_size = 1024;

// loop {
//     let mut buffer = vec![0; chunk_size];
//     let bcount = response
//         .read_vectored(&mut [io::IoSliceMut::new(&mut buffer)])
//         .unwrap();
//     buffer.truncate(bcount);

//     if !buffer.is_empty() {
//         bytes.extend(buffer.into_boxed_slice().into_vec().iter().cloned());
//     } else { break; }
// }

// // Write the file to disk unzipped
// let target = Path::new(destination);
// if !target.exists() {
//     std::fs::create_dir_all(target)?;
// }
// zip_extract::extract(Cursor::new(bytes), &target, true)?;

// Ok(())

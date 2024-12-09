use std::io::Write;
use zip::write::FileOptions;

pub fn create_zip(name: &str, email: &str, message: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut zip_buffer = Vec::new();

    {
        let mut zip = zip::ZipWriter::new(std::io::Cursor::new(&mut zip_buffer));

        let options = FileOptions::default()
            .compression_method(zip::CompressionMethod::Stored)
            .unix_permissions(0o755);

        zip.start_file("form_data.txt", options)?;
        zip.write_all(format!("Name: {}\nEmail: {}\nMessage: {}", name, email, message).as_bytes())?;
        zip.finish()?;
    } // Explicitly drop `zip` here to release the borrow on `zip_buffer`.

    Ok(zip_buffer)
}

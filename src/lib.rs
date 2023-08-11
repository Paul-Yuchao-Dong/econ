/*! 
This is mainly just the download file function
*/

use std::cmp::min;
use std::fs::File;
use std::io::Write;
use reqwest::Client;
use indicatif::{ProgressBar, ProgressStyle};
use futures_util::StreamExt;
pub mod econ_date;


/** `download_file` asynchronously downloads a file from a URL and saves it to a specified local path.
It uses the reqwest library to perform the HTTP GET request, indicatif library to display a progress bar, and futures_util to handle the async stream of bytes from the response.
# Example
```
download_file(&Client::new(), &url, "economist.zip").await.unwrap();
```
*/
pub async fn download_file(client: &Client, url: &str, path: &str) -> Result<(), String> {
    // makes a GET request to the URL.
    let res = client
        .get(url)
        .send()
        .await
        .or(Err(format!("Failed to GET from '{}'", &url)))?;
    // and checks if the response has the content length header.
    let total_size = res
        .content_length()
        .ok_or(format!("Failed to get content length from '{}'", &url))?;
    // creates a progress bar with the total size and sets the style for the bar. 
    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.white/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
        .progress_chars("█  "));
    // pb.set_message(&format!("Downloading {}", url));

    let mut file;
    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();
    
    // checks if the file already exists locally
    if std::path::Path::new(path).exists() {
        let file_size = std::fs::metadata(path).ok().map(|m| m.len()).unwrap_or(0);
        if file_size == total_size {
            println!("File already exists and has the same size, skipping download.");
            return Ok(());
        } else {
            println!("File exists, but has different size. Removing and downloading again.");
            std::fs::remove_file(path).or(Err(format!("Failed to remove file '{}'", path)))?;
            file = File::create(path).or(Err(format!("Failed to create file '{}'", path)))?;
        }
    } else {
    // If the file does not exist, it creates a new file.
        println!("Fresh file..");
        file = File::create(path).or(Err(format!("Failed to create file '{}'", path)))?;
    }

    println!("Commencing transfer");
    while let Some(item) = stream.next().await {
        // begins downloading the file by continuously reading from the stream of bytes
        let chunk = item.or(Err(format!("Error while downloading file")))?;
        // writing the data to the local file
        file.write(&chunk)
            .or(Err(format!("Error while writing to file")))?;
        // updating the progress bar with the current download progress
        let new = min(downloaded + (chunk.len() as u64), total_size);
        downloaded = new;
        pb.set_position(new);
    }
    // Once the download is complete, it finishes the progress bar
    pb.finish_with_message(&format!("Downloaded {} to {}", url, path));
    // returns an Ok result
    return Ok(());
}

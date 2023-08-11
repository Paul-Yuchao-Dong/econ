
use chrono::Local;
use reqwest::Client;

use econ::download_file;
use econ::econ_date::EconDate;

#[tokio::main]
async fn main() {
    // get today's date
    let dt = Local::now();
    // let dt = "2023-08-11T1:00:09Z".parse::<DateTime<Local>>().unwrap();
    // get nearest Saturday
    let econ = EconDate::new(dt);
    // generate download url
    let url = econ.to_string();

    // println!("{}", econ);
    download_file(&Client::new(), &url, &econ.file_name).await.unwrap();
}
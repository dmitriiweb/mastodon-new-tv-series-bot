use std::collections::HashMap;
use std::fs;
use std::error::Error;
use reqwest;
use std::io::{Cursor, Read};

pub trait RequestData {
    fn url(&self) -> String;
    fn params(&self) -> Vec<(String, String)> {
        vec![]
    }
    fn headers(&self) -> reqwest::header::HeaderMap {
        reqwest::header::HeaderMap::new()
    }
    fn json_body(&self) -> HashMap<String, String> {
        HashMap::new()
    }
}

pub struct FileDownload {
    download_url: String,
    save_folder: String,
}

impl FileDownload {
    fn file_path(&self) -> String {
        let file_name = self.download_url.split("/").last().unwrap();
        format!("{}/{}", self.save_folder, file_name)
    }
}

pub struct FileUpload {
    upload_url: String,
    file_path: String,
}


pub fn get<T: RequestData>(data: &T) -> Result<String, Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();
    let mut response = client.get(&data.url())
        .headers(data.headers())
        .query(&data.params())
        .send()?;
    let body = response.text()?;
    Ok(body)
}

pub fn post<T: RequestData>(data: &T) -> Result<String, Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();
    let mut response = client.post(&data.url())
        .headers(data.headers())
        .query(&data.params())
        .json(&data.json_body())
        .send()?;
    let body = response.text()?;
    Ok(body)
}

pub fn download_file<T: RequestData>(data: &T, file: FileDownload) -> Result<String, Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();
    let mut response = client.get(&data.url())
        .headers(data.headers())
        .query(&data.params())
        .send()?;
    let mut file = fs::File::create(&file.file_path())?;
    let mut content = Cursor::new(response.bytes()?);
    std::io::copy(&mut content, &mut file)?;
    Ok(file.file_path())
}

pub fn upload_file<T: RequestData>(data: &T, file: FileUpload) -> Result<String, Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();
    let file = fs::File::open(&file.file_path)?;
    let mut response = client.post(&data.url())
        .headers(data.headers())
        .query(&data.params())
        .body(file)
        .send()?;
    let body = response.text()?;
    Ok(body)
}
use reqwest;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::io::Cursor;

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
    pub download_url: String,
    pub save_folder: String,
    pub headers: reqwest::header::HeaderMap,
}

impl FileDownload {
    fn file_path(&self) -> String {
        let file_name = self.download_url.split("/").last().unwrap();
        format!("{}{}", self.save_folder, file_name)
    }
}

pub struct FileUpload {
    pub upload_url: String,
    pub file_path: String,
    pub headers: reqwest::header::HeaderMap,
    pub json_body: HashMap<String, String>,
    pub params: Vec<(String, String)>
}

pub fn get<T: RequestData>(data: &T) -> Result<String, Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(&data.url())
        .headers(data.headers())
        .query(&data.params())
        .send()?;
    let body = response.text()?;
    Ok(body)
}

pub fn post<T: RequestData>(data: &T) -> Result<String, Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();
    let response = client
        .post(&data.url())
        .headers(data.headers())
        .query(&data.params())
        .json(&data.json_body())
        .send()?;
    let body = response.text()?;
    Ok(body)
}

pub fn download_file(source_file: FileDownload) -> Result<String, Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(&source_file.download_url)
        .headers(source_file.headers.clone())
        .send()?;
    let mut file = fs::File::create(source_file.file_path())?;
    let mut content = Cursor::new(response.bytes()?);
    std::io::copy(&mut content, &mut file)?;
    Ok(source_file.file_path())
}

pub fn upload_file(source_file: FileUpload) -> Result<String, Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();
    let file = fs::File::open(&source_file.file_path)?;
    let response = client
        .post(&source_file.upload_url)
        .headers(source_file.headers)
        .query(&source_file.params)
        .body(file)
        .send()?;
    let body = response.text()?;
    Ok(body)
}

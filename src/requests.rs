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
    fn json_multipart(&self) -> reqwest::blocking::multipart::Form {
        reqwest::blocking::multipart::Form::new()
    }

    fn json_body(&self) -> HashMap<String, String> {
        let body = HashMap::new();
        body
    }
}

pub struct FileDownload {
    pub download_url: String,
    pub save_folder: String,
    pub headers: reqwest::header::HeaderMap,
}

impl FileDownload {
    pub fn file_path(&self) -> String {
        let file_name = self.download_url.split('/').last().unwrap();
        format!("{}{}", self.save_folder, file_name)
    }
    pub fn file_name(&self) -> String {
        self.download_url.split('/').last().unwrap().to_string()
    }
}

pub struct FileUpload {
    pub upload_url: String,
    pub file_path: String,
    pub headers: reqwest::header::HeaderMap,
    pub description: String,
    pub params: Vec<(String, String)>,
}

pub fn get<T: RequestData>(data: &T) -> Result<String, Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(data.url())
        .headers(data.headers())
        .query(&data.params())
        .send()?;
    let body = response.text()?;
    Ok(body)
}

pub fn post_multipart<T: RequestData>(data: &T) -> Result<String, Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();
    let response = client
        .post(data.url())
        .headers(data.headers())
        .query(&data.params())
        .multipart(data.json_multipart())
        .send()?;
    let body = response.text()?;
    Ok(body)
}

pub fn post_json<T: RequestData>(data: &T) -> Result<String, Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();
    let response = client
        .post(data.url())
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
    let form = reqwest::blocking::multipart::Form::new()
        .file("file", source_file.file_path.clone())?
        .text("description", source_file.description.clone());
    let response = client
        .post(&source_file.upload_url)
        .timeout(std::time::Duration::from_secs(60))
        .headers(source_file.headers)
        .query(&source_file.params)
        .multipart(form)
        .send()?;
    let body = response.text()?;
    Ok(body)
}

use futures::StreamExt;
use regex::{bytes, Regex};
use reqwest::{self, Client, Response};
use std::{env, fmt::write, path::Path, string};
use tokio::{self, io::copy};
use url::Url;

enum InputType {
    File,
    Url,
    Invalid,
}

fn determine_input(input: &String) -> InputType {
    match Url::parse(&input) {
        Ok(_) => InputType::Url,
        Err(_) => {
            let path = Path::new(input);
            if path.exists() {
                return InputType::File;
            }
            InputType::Invalid
        }
    }
}

async fn make_request(url: &String) -> (String, Response) {
    println!("Making request to {}", url);

    let path = env::current_dir().unwrap().to_string_lossy().to_string();
    let result = reqwest::get(url).await;
    let response = result.unwrap();

    let url_path = Url::parse(url).unwrap().path().to_string();
    let path_elements: Vec<_> = url_path.split("/").collect();
    let file_name = path_elements.last().clone().unwrap();

    let file_string = format!("{path}/{file_name}");
    return (file_string, response);
}

async fn make_file(file_name: String, response: Response) {
    let bytes = response.bytes().await.unwrap();
    println!("Saving to {}", file_name);
    std::fs::write(file_name, bytes).unwrap();
}

async fn make_files(path: &String) {
    let bytes = std::fs::read(path).unwrap();
    let byte_string = String::from_utf8_lossy(&bytes);
    let websites: Vec<_> = byte_string.split("\n").collect();
    for url in websites {
        let url_string = url.to_string();
        let (file_name, response) = make_request(&url_string).await;
        make_file(file_name, response).await;
    }
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let url = &args[1];

    let input_type = determine_input(url);
    match input_type {
        InputType::Url => {
            let (file_name, response) = make_request(url).await;
            make_file(file_name, response).await;
        }
        InputType::Invalid => {
            println!(" You provided an invalid input");
        }

        InputType::File => make_files(url).await,
    }
}

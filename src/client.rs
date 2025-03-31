// This file contains the implementation of the HTTP client. 
// It includes functions for making GET and POST requests, 
// handling responses, and managing headers.

use std::collections::HashMap;
use std::error::Error;
use reqwest::blocking::{Client, Response};

pub struct HttpClient {
    client: Client,
}

impl HttpClient {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let client = Client::new();
        Ok(HttpClient { client })
    }

    pub fn get(&self, url: &str, headers: Option<HashMap<&str, &str>>) -> Result<Response, Box<dyn Error>> {
        let mut request = self.client.get(url);

        if let Some(hdrs) = headers {
            for (key, value) in hdrs {
                request = request.header(key, value);
            }
        }

        let response = request.send()?;
        Ok(response)
    }

    pub fn post(&self, url: &str, body: &str, headers: Option<HashMap<&str, &str>>) -> Result<Response, Box<dyn Error>> {
        let mut request = self.client.post(url).body(body.to_string());

        if let Some(hdrs) = headers {
            for (key, value) in hdrs {
                request = request.header(key, value);
            }
        }

        let response = request.send()?;
        Ok(response)
    }
}

pub fn run_client() -> Result<(), Box<dyn std::error::Error>> {
    let client = HttpClient::new().unwrap();
    let response = client.get("http://127.0.0.1:8080", None);

    match response {
        Ok(resp) => {
            println!("Response: {}", resp.text().unwrap());
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
    Ok(())
}
use reqwest;

use serde::{Serialize, Deserialize};
use std::collections::HashMap;

const APIURL: &str = "https://librehomework-api.herokuapp.com/";

#[derive(Deserialize, Serialize)]
pub struct ApiResponse {
    error: Option<String>,
    data: Option<String>,
}

impl std::convert::From<reqwest::Error> for ApiResponse {
    fn from(e: reqwest::Error) -> Self {
        ApiResponse {
            error: Some(e.to_string()),
            data: None,
        }
    }
}


#[tauri::command]
pub async fn request(url: &str, method: Option<&str>, form: Option<HashMap<String, String>>) -> Result<(ApiResponse, u16), String> {
    let mut _url = url.to_string();
    let client = reqwest::Client::new();
    if !url.contains("https://") {
        _url = APIURL.to_owned() + &_url;
    }

    println!("{}", &_url);
    let res = match method {
        Some(m) => if m == "POST" {
                            client.post(_url).form(&form).send().await
                        } else {
                            client.get(_url).send().await
                        },
        None => client.get(_url).send().await

    }.map_err(|e| e.to_string())?;

    let status = res.status().as_u16();

    let apires: ApiResponse = res.json().await.map_err(|e| e.to_string())?;
    Ok( (apires, status) )
}
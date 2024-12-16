
use reqwest::Error;
use serde_json::Value;
use std::env;


//TODO: check url format https, api, github etc..


async fn call_github(api: String) -> Result<Value, Error> {
    let client = reqwest::Client::new();
    let res = client
        .get(api)
        .header("Accept", "application/vnd.github.preview")
        .header("User-Agent", "reqwest-rust")
        .send()
        .await?;

    let body = res.text().await.unwrap();
    let json = serde_json::from_str(&body).unwrap();
    Ok(json)
}

fn sanitize_input(arg: String) -> Result<String, Box<dyn std::error::Error>> {
    let trimmed = arg.trim();
    if trimmed.is_empty() {
        Err("Github repo cannot be empty!".into())
    } else {
        Ok(trimmed.to_string())
    }
}

#[tokio::main]
async fn main() {
    let input: Vec<String> = env::args().collect();
    let api = &input[1];
    match sanitize_input(api.clone()) {
        Ok(repo) => println!("Succ {repo:?}"),
        Err(e) => println!("Error:{e}"),
    }
    let res = call_github(api.clone()).await;
    match res {
        Ok(json) => {
            if let Some(stars) = json["stargazers_count"].as_i64() {
                println!("🌟 {stars} 🌟 stars!");
            } else {
                panic!("stargazes_count field not found!");
            }
        }
        Err(err) => println!("Error {err:?}"),
    }
}

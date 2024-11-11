use serde::Deserialize;
use reqwest::Error;
use reqwest::header::USER_AGENT;

#[derive(Deserialize'de, Debug)]
struct User {
    login: String,
    id: u32,
}

#[tokio::fn main()]
 async fn main2 () -> Result<(), Error> {
    let request_url = format!("https://api.github.com/repos/{owner}/{repo}/stargazers",
                              owner = "rust-lang-nursery",
                              repo = "rust-cookbook");
    println!("{}", request_url);
    let client = reqwest::Client::new();
    let response = client
        .get(&request_url)
        .header(USER_AGENT, "rust web-api-client demo")
        .send()
        .await?;

    let users: Vec<User> = response.json().await?;
    println!("{:?}", users);
    Ok(())
}
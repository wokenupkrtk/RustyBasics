use error_chain::error_chain;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpsRequest(reqwest::Error);
    }
}
    #[tokio::main]
    async fn main() -> Result<()>{
        let res = reqwest::get("https://httpbin.org/get").await?;
        println!("Status:{}", res.status());
        println!("Header:\n{:#?}", res.headers());

        let body = res.text().await?;
        println!("Body:\n{}", body);
        Ok(())
    }

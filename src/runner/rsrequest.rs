pub struct RsRequest;

impl RsRequest {
    pub async fn async_run(&self) -> Result<String, String> {
        let gh_link = "https://raw.githubusercontent.com/loop614/ruststarter/main/src/runner/mod.rs";
        return match reqwest::get(gh_link).await {
            Ok(res) => {
                println!("Status: {}", res.status());
                println!("Headers:\n{:#?}", res.headers());
                match res.status() {
                    reqwest::StatusCode::OK => {
                        let body = res.text().await.unwrap();
                        println!("Body:\n{}", body);
                        Ok(body)
                    },
                    reqwest::StatusCode::UNAUTHORIZED => Err("Get a new token".to_string()),
                    _ => Err(format!("bad res status {:?}", res.status()))
                }
            },
            Err(_) => Err("could not find the server".to_string())
        };
    }
}

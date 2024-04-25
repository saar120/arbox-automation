use serde_json::json;
use std::result::Result;
use std::string::String;

pub struct ArboxAPI {
    base_url: String,
    client: reqwest::Client,
    token: Option<String>,
}

#[derive(serde::Deserialize)]
struct LoginResponse {
    data: LoginData,
}

#[derive(serde::Deserialize)]
struct LoginData {
    token: String,
}

impl ArboxAPI {
    pub fn new() -> Self {
        Self {
            base_url: "https://apiappv2.arboxapp.com/api/v2".to_string(),
            client: reqwest::Client::new(),
            token: None,
        }
    }

    pub async fn init(&mut self, email: &str, password: &str) {
        let res = self.load_token(email, password).await;
        match res {
            Ok(_) => println!("Token loaded"),
            Err(e) => println!("Error: {}", e),
        }
    }

    // async fn get(&self, path: &str) -> Result<reqwest::Response, reqwest::Error> {

    //     let mut req = self.client
    //         .get(&format!("{}/{}", self.base_url, path))
    //         .header("Content-Type", "application/json")
    //         .header("User-Agent", "Arbox/4000531 CFNetwork/1494.0.7 Darwin/23.4.0")
    //         .header("whitelabel", "Arbox")
    //         .header("version", 11)
    //         .header("referername", "app");

    //     if self.token.is_some() {
    //         req = req.header("accesstoken", self.token.as_ref().unwrap());
    //     }

    //     req.send().await
    // }

    async fn post(&self, path: &str, body: &str) -> Result<reqwest::Response, reqwest::Error> {
        let mut req = self
            .client
            .post(&format!("{}/{}", self.base_url, path))
            .body(body.to_string())
            .header("Content-Type", "application/json")
            .header(
                "User-Agent",
                "Arbox/4000531 CFNetwork/1494.0.7 Darwin/23.4.0",
            )
            .header("whitelabel", "Arbox")
            .header("version", 11)
            .header("referername", "app");

        if self.token.is_some() {
            req = req.header("accesstoken", self.token.as_ref().unwrap());
        }
        req.send().await
    }

    async fn load_token(
        &mut self,
        email: &str,
        password: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let body = json!({
            "email": email,
              "password": password,
        });
        let res: reqwest::Response = self.post("user/login", &body.to_string()).await?;

        if let Ok(body) = res.text().await {
            let login_res: LoginResponse = serde_json::from_str(&body)?;
            self.token = Some(login_res.data.token.clone());
            Ok(())
        } else {
            Err("Failed to load token".into())
        }
    }
}

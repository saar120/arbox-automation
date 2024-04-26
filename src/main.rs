use dotenv::dotenv;
use std::env;

mod api;

#[tokio::main]
async fn main() {

    dotenv().ok();

    let email = env::var("ARBOX_EMAIL").expect("ARBOX_EMAIL must be set");
    let password = env::var("ARBOX_PASSWORD").expect("ARBOX_PASSWORD must be set");

    println!("env vars loaded");


    let mut arbox = api::arbox_api::ArboxAPI::new();
    arbox.init(&*email, &*password).await;

    let profile = arbox.get_profile().await;

    print!("{:?}", profile)
}
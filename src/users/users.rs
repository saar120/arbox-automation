use std::{env, fs};
use log::{error, info};
use crate::api;
use crate::users::models::User;

pub fn load_users_from_json() -> Vec<User> {
    // load users.json
    let users_json = fs::read_to_string("./src/users/users.json");
    let users_json = match users_json {
        Ok(json) => json,
        Err(e) => {
            panic!("Error reading users.json: {}", e);
        }
    };
    let users: Vec<User> = serde_json::from_str(&users_json).expect("Error parsing users.json");
    users
}

pub async fn sign_user_to_class(user_id: u32) {
    let email = env::var("ARBOX_EMAIL").expect("ARBOX_EMAIL must be set");
    let password = env::var("ARBOX_PASSWORD").expect("ARBOX_PASSWORD must be set");

    info!("env vars loaded");


    let mut arbox = api::arbox_api::ArboxAPI::new();
    arbox.init(&*email, &*password).await;

    let profile = arbox.get_profile().await;

    match profile {
        Ok(profile) => {
            info!("Loaded user {} profile", email);
        }
        Err(e) => {
            error!("Error: {:?}", e);
        }
    }
}

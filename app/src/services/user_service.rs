use std::env;

use reqwasm::http;
use serde::{Deserialize, Serialize};

pub async fn register_user(name: &str, email: &str, password: &str) -> Result<User, String> {
    let user = User {
        name: name.to_string(),
        email: email.to_string(),
        password: password.to_string(),
    };
    let request = serde_json::to_string(&user).expect("Invalid json provided");
    let response = match http::Request::post(&backend_url("/register"))
        .header("Content-Type", "application/json")
        .body(request)
        .send()
        .await
    {
        Ok(res) => res,
        Err(_) => return Err("Failed to make request".to_string()),
    };

    if response.status() != 200 {
        let error_response = response.text().await;
        if let Ok(error_response) = error_response {
            return Err(error_response);
        } else {
            return Err(format!("Error status code: {}", response.status()));
        }
    }

    let res_json = response.json::<User>().await;
    match res_json {
        Ok(user) => Ok(user),
        Err(_) => Err("Failed to parse response".to_string()),
    }
}

pub async fn login_user(username: &str, password: &str) -> Result<String, String> {
    let auth_request = AuthenticationRequest {
        username: username.to_string(),
        password: password.to_string(),
    };
    let request = serde_json::to_string(&auth_request).expect("Invalid json provided");
    let response = match http::Request::post(&backend_url("/login"))
        .header("Content-Type", "application/json")
        .credentials(http::RequestCredentials::Include)
        .body(request)
        .send()
        .await
    {
        Ok(res) => res,
        Err(_) => return Err("Failed to make request".to_string()),
    };

    if response.status() != 200 {
        let error_response = response.text().await;
        if let Ok(error_response) = error_response {
            return Err(error_response);
        } else {
            return Err(format!("API error: {}", response.status()));
        }
    }

    let res_json = response.text().await;
    match res_json {
        Ok(data) => Ok(data),
        Err(_) => Err("Failed to parse response".to_string()),
    }
}

pub async fn load_users() -> Result<Vec<User>, String> {
    let response = match http::Request::get(&backend_url("/api/users/"))
        .header("Content-Type", "application/json")
        .credentials(http::RequestCredentials::Include)
        .send()
        .await
    {
        Ok(res) => res,
        Err(_) => return Err("Failed to make request".to_string()),
    };

    if response.status() != 200 {
        let error_response = response.json::<String>().await;
        if let Ok(error_response) = error_response {
            return Err(error_response);
        } else {
            return Err(format!("API error: {}", response.status()));
        }
    }

    let res_json = response.json::<Vec<User>>().await;
    match res_json {
        Ok(data) => Ok(data),
        Err(_) => Err("Failed to parse response".to_string()),
    }
}

pub async fn logout() -> Result<String, String> {
    let response = match http::Request::post(&backend_url("/logout"))
        .header("Content-Type", "application/json")
        .credentials(http::RequestCredentials::Include)
        .send()
        .await
    {
        Ok(res) => res,
        Err(_) => return Err("Failed to make request".to_string()),
    };

    if response.status() != 200 {
        let error_response = response.json::<String>().await;
        if let Ok(error_response) = error_response {
            return Err(error_response);
        } else {
            return Err(format!("API error: {}", response.status()));
        }
    }

    let res_json = response.text().await;
    match res_json {
        Ok(data) => Ok(data),
        Err(_) => Err("Failed to parse response".to_string()),
    }
}

fn backend_url<'a>(path: &str) -> String {
    let host = env!("BACKEND_URL", "BACKEND_URL not set at compile time").to_owned();
    host + path
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub email: String,
    pub name: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthenticationRequest {
    pub username: String,
    pub password: String,
}
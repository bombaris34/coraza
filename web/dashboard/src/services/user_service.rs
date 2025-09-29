use super::BASE_URL;
use crate::models::{TokenResponse, User, UserLogin, UserRegistration, UserUpdate};
use anyhow::Result;
use reqwest::Client;

pub async fn login(credentials: UserLogin) -> Result<String> {
    let client = Client::new();
    let res = client
        .post(format!("{}/auth/login", BASE_URL))
        .json(&credentials)
        .send()
        .await?;
    let token = res.json::<TokenResponse>().await?;
    Ok(token.token)
}

pub async fn register(credentials: UserRegistration) -> Result<String> {
    let client = Client::new();
    let res = client
        .post(format!("{}/auth/register", BASE_URL))
        .json(&credentials)
        .send()
        .await?;
    let token = res.json::<TokenResponse>().await?;
    Ok(token.token)
}

pub async fn get_users(token: &str) -> Result<Vec<User>> {
    let client = Client::new();
    let res = client
        .get(format!("{}/users/", BASE_URL))
        .bearer_auth(token)
        .send()
        .await?;
    let users = res.json::<Vec<User>>().await?;
    Ok(users)
}

pub async fn create_user(token: &str, user: User) -> Result<User> {
    let client = Client::new();
    let res = client
        .post(format!("{}/users/", BASE_URL))
        .bearer_auth(token)
        .json(&user)
        .send()
        .await?;
    let user = res.json::<User>().await?;
    Ok(user)
}

pub async fn update_user(token: &str, id: &str, user: UserUpdate) -> Result<User> {
    let client = Client::new();
    let res = client
        .put(format!("{}/users/{}", BASE_URL, id))
        .bearer_auth(token)
        .json(&user)
        .send()
        .await?;
    let user = res.json::<User>().await?;
    Ok(user)
}

pub async fn delete_user(token: &str, id: &str) -> Result<()> {
    let client = Client::new();
    client
        .delete(format!("{}/users/{}", BASE_URL, id))
        .bearer_auth(token)
        .send()
        .await?;
    Ok(())
}

pub async fn get_current_user(token: &str) -> Result<User> {
    let client = Client::new();
    let res = client
        .get(format!("{}/users/me", BASE_URL))
        .bearer_auth(token)
        .send()
        .await?;
    let user = res.json::<User>().await?;
    Ok(user)
}

pub async fn validate_token(token: &str) -> Result<User> {
    let client = Client::new();
    let res = client
        .post(format!("{}/auth/validate", BASE_URL))
        .json(&serde_json::json!({ "token": token }))
        .send()
        .await?;
    if res.status().is_success() {
        Ok(res.json::<User>().await?)
    } else {
        Err(anyhow::anyhow!("Token validation failed"))
    }
}

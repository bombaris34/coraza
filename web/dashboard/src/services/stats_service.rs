use super::BASE_URL;
use crate::models::RegistrationStats;
use reqwest::Client;
use std::error::Error;

pub async fn get_registration_stats(token: &str) -> Result<Vec<RegistrationStats>, Box<dyn Error>> {
    let client = Client::new();
    let url = format!("{}/stats/registrations", BASE_URL);
    let stats = client
        .get(&url)
        .bearer_auth(token)
        .send()
        .await?
        .json()
        .await?;
    Ok(stats)
}


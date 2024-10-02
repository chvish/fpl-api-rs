pub mod fixture;
pub mod manager;
pub mod bootstrap;

use serde::de::DeserializeOwned;
use reqwest;
use anyhow::Result;


pub struct FPLClient{
    base_url: String
}

impl FPLClient {
    pub fn new() -> Self {
        Self{
            base_url: "https://fantasy.premierleague.com/".to_string()
        }
    }
}

impl FPLClient {

    async fn get_data<T: DeserializeOwned>(&self, url: &str) -> Result<T> {
        let val = reqwest::get(url)
            .await?
            .json::<T>()
            .await?;
        Ok(val)
    }

    pub async fn get_bootstrap_data(&self) -> Result<bootstrap::BootstrapData> {
        let url = format!("{}/api/bootstrap-static/", self.base_url);
        self.get_data::<bootstrap::BootstrapData>(&url).await
    }
    pub async fn get_fixtures(&self) -> Result<fixture::Fixtures> {
        let url = format!("{}/api/fixtures/", self.base_url);
        self.get_data::<fixture::Fixtures>(&url).await
    }
    
    pub async fn get_manager_details(&self, id: &str) -> Result<manager::Manager>{
        let url = format!("{}/api/entry/{}/", self.base_url, id);
        self.get_data::<manager::Manager>(&url).await
    }

    pub fn get_manager_transfers(&self, id: &str) -> (){
        let _ = format!("{}/api/entry/{}/transfers/", self.base_url, id);
    }

    pub async fn get_manager_team_for_gw(&self, id: &str, gw: &str) -> Result<manager::GWTeam>{
        let url = format!("{}/api/entry/{}/event/{}/picks/", self.base_url, id, gw);
        self.get_data::<manager::GWTeam>(&url).await
    }
}





#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fixtures() {
        let client = FPLClient::new();
        let maybe_fixtures = client.get_fixtures().await;
        assert_eq!(maybe_fixtures.is_ok(), true);
    }
}


pub mod bootstrap;
pub mod fixture;
pub mod manager;

use anyhow::Result;
use reqwest;
use serde::de::DeserializeOwned;

pub struct FPLClient {
    base_url: String,
}

impl FPLClient {
    pub fn new() -> Self {
        Self {
            base_url: "https://fantasy.premierleague.com/".to_string(),
        }
    }
}

impl FPLClient {
    async fn get_data<T: DeserializeOwned>(&self, url: &str) -> Result<T> {
        let val = reqwest::get(url).await?.json::<T>().await?;
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

    pub async fn get_manager_details(&self, id: &str) -> Result<manager::Manager> {
        let url = format!("{}/api/entry/{}/", self.base_url, id);
        self.get_data::<manager::Manager>(&url).await
    }

    pub async fn get_manager_transfers(&self, id: &str) -> Result<manager::Transfers> {
        let url = format!("{}/api/entry/{}/transfers/", self.base_url, id);
        self.get_data::<manager::Transfers>(&url).await
    }

    pub async fn get_manager_team_for_gw(&self, id: &str, gw: &str) -> Result<manager::GWTeam> {
        let url = format!("{}/api/entry/{}/event/{}/picks/", self.base_url, id, gw);
        self.get_data::<manager::GWTeam>(&url).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_manager() {
        let client = FPLClient::new();
        let manager = client.get_manager_details("3332624").await.unwrap();
        assert_eq!(manager.player_first_name, "Vishal".to_string());
        assert_eq!(manager.name, "Gakpo Ke Gunde".to_string());

        let gw_team = client
            .get_manager_team_for_gw("3332624", "4")
            .await
            .unwrap();
        assert_eq!(gw_team.active_chip, None);

        let transfers = client.get_manager_transfers("3332624").await.unwrap();
        assert!(transfers.len() > 0);
    }

    #[tokio::test]
    async fn test_fixtures() {
        let client = FPLClient::new();
        let maybe_fixtures = client.get_fixtures().await;
        assert!(maybe_fixtures.is_ok());
    }
}

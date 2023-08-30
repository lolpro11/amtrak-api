use crate::{errors, responses};

const BASE_API_URL: &str = "https://api-v3.amtraker.com/v3";

pub type Result<T> = std::result::Result<T, errors::Error>;

#[derive(Debug, Clone)]
pub struct Client {
    base_url: String,
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

impl Client {
    pub fn new() -> Self {
        Self {
            base_url: BASE_API_URL.to_string(),
        }
    }

    pub fn with_base_url(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
        }
    }

    pub async fn trains(&self) -> Result<responses::TrainResponse> {
        let url = format!("{}/trains", self.base_url);

        let response = reqwest::Client::new()
            .get(url)
            .send()
            .await?
            .json::<responses::TrainResponse>()
            .await?;

        Ok(response)
    }

    pub async fn train(&self, train_id: &str) -> Result<responses::TrainResponse> {
        let url = format!("{}/trains/{}", self.base_url, train_id);

        let response = reqwest::Client::new()
            .get(url)
            .send()
            .await?
            .json::<responses::TrainResponse>()
            .await?;

        Ok(response)
    }

    pub async fn stations(&self) -> Result<responses::StationResponse> {
        let url = format!("{}/stations", self.base_url);

        let response = reqwest::Client::new()
            .get(url)
            .send()
            .await?
            .json::<responses::StationResponse>()
            .await?;

        Ok(response)
    }

    pub async fn station(&self, station_id: &str) -> Result<responses::StationResponse> {
        let url = format!("{}/stations/{}", self.base_url, station_id);

        let response = reqwest::Client::new()
            .get(url)
            .send()
            .await?
            .json::<responses::StationResponse>()
            .await?;

        Ok(response)
    }
}

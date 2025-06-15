use anyhow::Result;
use reqwest::Error;
use reqwest::blocking::Response;
use std::fmt;
pub struct Client {
    base: String,
    http: reqwest::blocking::Client,
}

impl fmt::Display for Client {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.base)
    }
}

impl Client {
    pub fn new() -> Self {
        Client {
            base: "https://hacker-news.firebaseio.com/v0".into(),
            http: reqwest::blocking::Client::new(),
        }
    }

    pub fn fetch_top_ids(&self) -> Result<Vec<u64>, Error> {
        let url = format!("{}/topstories.json", &self.base);
        let response: Response = self.http.get(url).send()?;
        let ids: Vec<u64> = response.json()?;
        Ok(ids)
    }
}

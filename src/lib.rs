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

#[derive(serde::Deserialize, Debug)]
pub struct Story {
    by: String,
    descendants: u32,
    id: u64,
    kids: Vec<u64>,
    score: u32,
    text: Option<String>,
    time: u64,
    title: String,
    r#type: String,
    url: Option<String>,
}

impl Client {
    pub fn new() -> Self {
        Client {
            base: "https://hacker-news.firebaseio.com/v0".into(),
            http: reqwest::blocking::Client::new(),
        }
    }

    pub fn fetch_story(&self, id: usize) -> Result<Story, Error> {
        let url = format!("{}/item/{}.json", &self.base, id);
        let response: Response = self.http.get(url).send()?;
        let story: Story = response.json()?;
        Ok(story)
    }

    pub fn fetch_top_ids(&self, number: usize) -> Result<Vec<u64>, Error> {
        let url = format!("{}/topstories.json?", &self.base);
        let response: Response = self
            .http
            .get(url)
            .query(&[
                ("limitToFirst", number.to_string()),
                ("orderBy", "\"$priority\"".to_string()),
            ])
            .send()?;
        let ids: Vec<u64> = response.json()?;
        Ok(ids)
    }

    pub fn fetch_new_stories(&self, number: usize) -> Result<Vec<u64>, Error> {
        let url = format!("{}/newstories.json", &self.base);
        let response: Response = self
            .http
            .get(url)
            .query(&[
                ("limitToFirst", number.to_string()),
                ("orderBy", "\"$priority\"".to_string()),
            ])
            .send()?;
        let ids: Vec<u64> = response.json()?;
        Ok(ids)
    }
}

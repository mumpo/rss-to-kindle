use crate::models::{Article, Feed};
use rss::Channel;
use std::error::Error;

pub fn fetch_feed(url: &str) -> Result<Feed, Box<dyn Error>> {
    let content = reqwest::blocking::get(url)?.bytes()?;
    let channel = Channel::read_from(&content[..])?;

    Ok(Feed {
        articles: channel
            .items()
            .iter()
            .map(|x| Article {
                title: x.title().unwrap_or_default().to_string(),
                content: x.content().unwrap_or_default().to_string(),
            })
            .collect(),
    })
}

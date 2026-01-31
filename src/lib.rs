use rss::Channel;
use std::error::Error;

pub fn get_feed_from_url(url: &str) -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::blocking::get(url)?.bytes()?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}

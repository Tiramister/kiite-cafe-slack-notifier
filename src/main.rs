use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Deserialize, Debug)]
struct NextSongResponse {
    id: u32,
    video_id: String,
    title: String,
    artist_id: u32,
    artist_name: String,
    start_time: DateTime<Utc>,
    msec_duration: u32,
    thumbnail: String,
}

#[derive(Serialize)]
struct SlackBody {
    text: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;

    let client = reqwest::Client::new();
    let response = client
        .get("https://cafe.kiite.jp/api/cafe/next_song")
        .send()
        .await?
        .json::<NextSongResponse>()
        .await?;
    eprintln!("{:?}", response);

    let objective_video_ids = std::fs::read_to_string("videos.txt")?;
    if !objective_video_ids
        .lines()
        .any(|video_id| video_id == &response.video_id)
    {
        eprintln!("Skip Slack notification");
        return Ok(());
    }

    let slack_body = SlackBody {
        text: format!("{} {}", response.title, response.thumbnail),
    };
    let webhook_url = env::var("WEBHOOK_URL")?;
    client.post(webhook_url).json(&slack_body).send().await?;
    eprintln!("Sent Slack notification");

    Ok(())
}

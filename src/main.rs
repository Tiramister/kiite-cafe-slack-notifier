use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = reqwest::Client::new();
    let response = client
        .get("https://cafe.kiite.jp/api/cafe/next_song")
        .send()
        .await?
        .json::<NextSongResponse>()
        .await?;
    eprintln!("{:?}", response);
    Ok(())
}

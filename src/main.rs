#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let response = reqwest::get("https://cafe.kiite.jp/api/cafe/next_song")
        .await?
        .text()
        .await?;
    println!("{}", response);
    Ok(())
}

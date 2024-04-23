use rocket::tokio;
use rss::Channel;

#[tokio::test]
async fn demo1() -> Result<(), Box<dyn std::error::Error>> {
    let content = reqwest::get("https://www.solidot.org/index.rss")
        .await?
        .bytes()
        .await?;
    let channel = Channel::read_from(&content[..])?;
    dbg!(&channel);
    Ok(())
}

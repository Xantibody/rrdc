use rrdc::service::http_client::HttpClient;
use rrdc::Result;

pub async fn execute(url: &str) -> Result<()> {
    let client = HttpClient::new()?;
    let html = client.fetch_html(url).await?;

    // Unix philosophy: output to stdout
    print!("{}", html);

    Ok(())
}

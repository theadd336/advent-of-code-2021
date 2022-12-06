use anyhow::Error;
use futures_util::StreamExt;
use reqwest::Client;

const AOC_BASE_URL: &str = "https://adventofcode.com/2022/day";

async fn fetch_data(day: u8) -> Result<(), Error> {
    let client = Client::new();
    let mut response = client
        .get(format!("{AOC_BASE_URL}/{day}/input"))
        .send()
        .await?
        .bytes_stream();
    while let Some(item) = response.next().await {
        println!("Chunk: {:?}", item?);
    }
    Ok(())
}

pub async fn write_input_file(day: u8) -> Result<(), Error> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fetch_data() {
        fetch_data(4).await.unwrap();
    }
}

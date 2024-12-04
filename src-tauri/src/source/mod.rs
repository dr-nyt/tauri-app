mod server;

#[derive(Clone)]
pub struct Part {
    pub title: String,
    pub url: String,
    pub content: Option<String>,
}

pub async fn download_file(
    source: &str,
    url: &str,
    batch_size: usize,
) -> Result<Vec<String>, String> {
    if source == "server" {
        return server::download_file(url, batch_size).await;
    }
    Err(format!("Source {} not found", source))
}

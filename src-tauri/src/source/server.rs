use futures::future::join_all;
use kuchikiki::{traits::*, NodeRef};

pub async fn download_file(url: &str, batch_size: usize) -> Result<Vec<String>, String> {
    let document = kuchikiki::parse_html().one("<html></html>");

    let total_pages = get_total_pages(&document).await;

    Ok(vec![])
}

async fn get_total_pages(document: &NodeRef) -> usize {
    let mut total_pages = 1;

    let last_page_node = match document.select("#list-chapter ul.pagination > li.last a") {
        Ok(mut nodes) => nodes.next(),
        Err(_) => None,
    };

    match last_page_node {
        Some(node) => {
            let last_page_url = node.attributes.borrow().get("href").unwrap().to_string();
            let total_page = last_page_url
                .split("=")
                .last()
                .expect("Couldn't get last split at '='")
                .parse::<usize>()
                .unwrap_or(1);
            total_pages = total_page;
        }
        None => {
            total_pages = 1;
        }
    };

    return total_pages;
}

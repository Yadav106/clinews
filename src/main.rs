#![allow(unused)]

use dotenv::dotenv;
use newsapi::{Article, Country, Endpoint, NewsAPI, NewsApiResponse};
use std::{error::Error, fmt::format};

mod themes;

fn render_articles(articles: &Vec<Article>) {
    let theme = themes::default();
    theme.print_text("# Top Headlines\n\n");
    for a in articles {
        theme.print_text(&format!("`{}`", a.title()));
        theme.print_text(&format!("> *{}*", a.url()));
        theme.print_text("---");
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv();

    let api_key = std::env::var("CLI_NEWS_API_KEY")?;

    let mut newsapi = NewsAPI::new(&api_key);
    newsapi
        .endpoint(Endpoint::TopHeadlines)
        .country(Country::In);

    let news_api_response = newsapi.fetch_async().await?;

    render_articles(news_api_response.articles());

    Ok(())
}

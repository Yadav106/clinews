#![allow(unused)]

use dotenv::dotenv;
use newsapi::{get_articles, Articles};
use std::{error::Error, fmt::format};

mod themes;

fn render_articles(articles: &Articles) {
    let theme = themes::default();
    theme.print_text("# Top Headlines\n\n");
    for a in &articles.articles {
        theme.print_text(&format!("`{}`", a.title));
        theme.print_text(&format!("> *{}*", a.url));
        theme.print_text("---");
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    dotenv();

    let api_key = std::env::var("CLI_NEWS_API_KEY")?;

    let url: &str = "https://newsapi.org/v2/top-headlines?country=in&apiKey=";
    let url = format!("{}{}", url, api_key);
    let articles: Articles = get_articles(&url)?;

    render_articles(&articles);

    Ok(())
}

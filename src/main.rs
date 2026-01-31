fn main() {
    let sources = [
        "https://www.eldiario.es/rss",
        "https://api.3cat.cat/noticies?_format=rss&origen=frontal&frontal=n324-portada-noticia&version=2.0",
    ];

    let feed = rss_to_kindle::feed::fetch_feed("https://www.eldiario.es/rss/").unwrap();

    for article in feed.articles.iter().take(10) {
        println!("item {}", article.title)
    }
}

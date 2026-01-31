fn main() {
    let sources = [
        "https://www.eldiario.es/rss",
        "https://api.3cat.cat/noticies?_format=rss&origen=frontal&frontal=n324-portada-noticia&version=2.0",
    ];

    let chan = rss_to_kindle::get_feed_from_url("https://www.eldiario.es/rss/").unwrap();

    for item in chan.items().iter().take(10) {
        println!("item {}", item.title().unwrap_or_default())
    }
}

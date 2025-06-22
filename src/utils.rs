pub fn yt_url(url: &str) -> bool {
    url.contains("youtube.com") || url.contains("youtu.be")
}
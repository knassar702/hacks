pub fn send_request(url: &str) -> Result<ureq::Response, ureq::Error> {
    ureq::get(url).set("User-agent", "burl/0.1").call()
}

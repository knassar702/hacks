mod app;

fn main() {
    let httprobe = app::Httprobe {
        urls: vec!["nokia.com".to_string()],
        threads: 50,
        timeout: 10,
    };
    httprobe.start(app::Mode::Large);
}

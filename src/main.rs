use dotenv::dotenv;
use std::sync::{Arc, Mutex};
use std::thread;

mod app;
mod packet_capture;

fn main() -> iced::Result {
    dotenv().ok();

    let captured_domains: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    let captured_domains_clone = captured_domains.clone();

    thread::spawn(move || {
        packet_capture::run_packet_capture(captured_domains_clone);
    });

    app::run_app(captured_domains.clone())
}

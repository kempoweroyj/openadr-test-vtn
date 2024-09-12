use log::info;

pub async fn ping() -> &'static str {
    info!("Ping handler execution");
    "pong"
}

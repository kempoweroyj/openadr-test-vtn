use log::info;

pub async fn get_ping() -> &'static str {
    info!("Ping handler execution");
    "pong"
}

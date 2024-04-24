use std::fs::File;
use std::io::Write;
use std::time::Duration;

use aws_config::imds::client::Client;

use tokio;

const CONFIG_FILE: &str = "/etc/instance.conf";

#[tokio::main]
async fn main() {
    let client = Client::builder()
        .max_attempts(10)
        .connect_timeout(Duration::from_secs(5))
        .build();

    let instance_id = client.get("/latest/meta-data/instance-id")
        .await.expect("Unable to retrieve instance-id from IMDS.");

    println!("Fetched instance-id {}.", instance_id.as_ref());

    let mut config_file = File::create(CONFIG_FILE)
        .expect("Unable to open config file.");

    let config = format!("instance = \"{}\"\n", instance_id.as_ref());

    config_file.write_all(config.as_bytes())
        .expect("Unable to write to configuration file.");

    println!("Finished writing configuration to {}.", CONFIG_FILE);
}
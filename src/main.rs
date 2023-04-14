
use bluez_async::BluetoothSession;
use std::{time::Duration, collections::HashMap};
use tokio::time;
const pswd: &str = "123";
const url: &str = "minutebackend.hopto.org:1312/customers";
const SCAN_DURATION: Duration = Duration::from_secs(30);
#[tokio::main]
async fn main() -> Result<(), eyre::Report> {
    let mut map = HashMap::new();

    let client = reqwest::Client::new();
    pretty_env_logger::init();

    let (_, session) = BluetoothSession::new().await?;
    loop{
        session.start_discovery().await?;
        time::sleep(SCAN_DURATION).await;
        session.stop_discovery().await?;
        let devices = session.get_devices().await?;
        println!("Devices: {:#?}", devices.len());
        map.insert("pswd", pswd);
        if &devices.len() < &15 {
            map.insert("customers", "2");  
            if &devices.len() < &6{
            map.insert("customers", "1");  
            }
        }else{
            map.insert("customers", "3");  
        }
        let res = client.post(url)
        .json(&map)
        .send()
        .await?;
      }

}
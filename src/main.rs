
use std::time::Duration;


//use embassy_executor::{Spawner, Executor};

use esp_idf_hal::prelude::Peripherals;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::timer::EspTaskTimerService;
use esp_idf_sys as _;
use serde::{Serialize, Deserialize};
use simplehttp::simplehttp_esp32::new_esp_http;
use surrealdb_http::surreal::{SurrealDbClient, SurrealStatementReply};
mod wifi;


const PASS: &str = env!("SURREALDB_ENDPOINT");

#[derive(Serialize,Deserialize,Debug)]
struct Actor {
    first_name: String,
    last_name: String,
    id: String,
}
fn main() {

    esp_idf_sys::link_patches();
    let peripherals = Peripherals::take().unwrap();
    let sysloop = EspSystemEventLoop::take().unwrap();
    let timer_service = EspTaskTimerService::new().unwrap();
    let _wifi = wifi::wifi(peripherals.modem, sysloop,Some(EspDefaultNvsPartition::take().unwrap()),timer_service).unwrap();

    let mut surreal_client = SurrealDbClient::new("root", "root", "http://10.11.12.177:8000", "myns", "mydb", new_esp_http());

    loop {
        for i in 1..50 {
            let actor: SurrealStatementReply<Actor> = surreal_client.query_single(&format!("SELECT * FROM actor WHERE id=actor:{}",i)).unwrap();
            let a = actor.result.first().unwrap();
            println!("Actor: {:?}",a);
            std::thread::sleep(Duration::from_micros(1000000));
        }

    }
    // block_on(_wifi.disconnect()).unwrap();
    // println!("Disconnected WiFi");

}

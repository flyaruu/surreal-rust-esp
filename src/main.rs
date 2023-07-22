
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;


//use embassy_executor::{Spawner, Executor};

use anyhow::anyhow;
use embedded_svc::http::Method;
use embedded_svc::io::Write;
use esp_idf_hal::prelude::Peripherals;
use esp_idf_svc::errors::EspIOError;
use esp_idf_svc::{eventloop::EspSystemEventLoop, http::server::EspHttpServer};
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::timer::EspTaskTimerService;
use esp_idf_sys as _;
use serde::{Serialize, Deserialize};
use simplehttp::simplehttp_esp32::new_esp_http;
use surrealdb_http::surreal::{SurrealDbClient, SurrealStatementReply};

use esp_idf_hal::gpio::*;
// use esp_idf_hal::peripherals::Peripherals;


mod wifi;


const SURREALDB_ENDPOINT: &str = env!("SURREALDB_ENDPOINT");

#[derive(Serialize,Deserialize,Debug)]
struct Actor {
    first_name: String,
    last_name: String,
    id: String,
    films: Vec<String>,
}

fn main() {

    esp_idf_sys::link_patches();
    let peripherals = Peripherals::take().unwrap();
    let sysloop = EspSystemEventLoop::take().unwrap();
    let timer_service = EspTaskTimerService::new().unwrap();
    let _wifi = wifi::wifi(peripherals.modem, sysloop,Some(EspDefaultNvsPartition::take().unwrap()),timer_service).unwrap();
    let led_pin = PinDriver::output(peripherals.pins.gpio4).unwrap().into_output().unwrap();

    let led = Arc::new(Mutex::new(led_pin));

    let surreal_client = Arc::new(Mutex::new(SurrealDbClient::new("root", "root", SURREALDB_ENDPOINT, "myns", "mydb", new_esp_http())));

    let _httpd = httpd(led.clone(),surreal_client).unwrap();
    loop {
        println!("looping!");
        std::thread::sleep(Duration::from_micros(2000000));
    }
}

fn httpd<T>(led: Arc<Mutex<PinDriver<'static,T,Output>>>, client: Arc<Mutex<SurrealDbClient>>)->Result<EspHttpServer,EspIOError>
    where T: Pin {
    let mut server = EspHttpServer::new(&Default::default())?;
    server
        .fn_handler("/actor", Method::Get,move |mut req| {
            let id = extract_id(req.connection().uri()).unwrap();
            let actor: SurrealStatementReply<Actor> = client.lock().unwrap().query_single(&format!("SELECT *,->played_in->film.title as films FROM actor WHERE id=actor:{}",id)).unwrap();
            let a = actor.result.first().unwrap();
            led.lock().unwrap().toggle().unwrap();
            req.into_response(200,Some("OK"), &[("Content-Type","application/json")])?
                .write_all(serde_json::to_string_pretty(a).unwrap().as_bytes())?;
            Ok(())
        })?;
    Ok(server)

}

/// Crude way to extract http params
fn extract_id(url: &str)->anyhow::Result<u32> {
    let (_,param) = url.split_once('?').ok_or(anyhow!("missing id"))?;
    let params: HashMap<&str,&str> = param.split('&').filter_map(|e| e.split_once('=')).collect();
    Ok(params.get("id").ok_or(anyhow!("missing id"))?.parse()?)
}
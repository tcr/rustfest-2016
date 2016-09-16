// Required (for now) to compile programs that run on Tessel.
#![feature(alloc_system)]
extern crate alloc_system;

#[macro_use] extern crate nickel;
extern crate climate_si7020;
extern crate local_ip;
extern crate relay_mono;
extern crate rustc_serialize;
// Crate for the Tessel microcontroller.
extern crate tessel;

use climate_si7020::Climate;
use nickel::{Nickel, HttpRouter, MediaType, Options};
use relay_mono::RelayArray;
use rustc_serialize::json;
use std::io::prelude::*;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use tessel::Tessel;

#[derive(RustcDecodable, RustcEncodable)]
struct Measurement {
    fahrenheit: f64,
}

const INDEX_HTML: &'static str = include_str!("index.html");
const JQUERY_JS: &'static str = include_str!("jquery.min.js");

fn main() {
    // Create a new Tessel
    let (port_a, port_b)  = Tessel::ports().unwrap();

    let mut temp = Climate::new(port_b);
    temp.connect().expect("Could not connect to climate sensor.");
    println!("Connected to climate sensor.");
    let sensor = Mutex::new(temp);

    // Create the relay array.
    let mut relays = RelayArray::new(port_a);
    relays.connect().expect("Could not connect to relay array.");
    let latches = Mutex::new(relays);

    let mut server = Nickel::new();

    // Force four threads.
    server.options = Options::default()
        .thread_count(Some(4));

    server.get("/", middleware! { |_, mut res|
        res.set(MediaType::Html);
        INDEX_HTML
    });

    server.get("/jquery.min.js", middleware! { |_, mut res|
        res.headers_mut().set_raw("content-type", vec!["text/javascript".as_bytes().to_vec()]);
        JQUERY_JS
    });

    // A POST request turns on both relay channels.
    server.post("/api/cool", middleware! { |_, mut res|
        let mut relays = latches.lock().unwrap();
        relays.set_latch(1, true);
        relays.set_latch(2, true);
        r#"{"success": true}"#
    });

    // Send temperature readings to the browser as JSON.
    server.get("/api/temperature", middleware! { |_, mut res|
        res.headers_mut().set_raw("content-type", vec!["text/event-stream".as_bytes().to_vec()]);

        let mut stream = match res.start() {
            Ok(res) => res,
            _ => panic!("Could not send headers."),
        };

        loop {
            let temp = {
                sensor.lock().unwrap().read_temperature().unwrap()
            };

            let reading = Measurement {
                fahrenheit: temp,
            };

            // Encode using rustc_serialize
            if let Err(_) = stream.write_all(b"data: ") { break; }
            if let Err(_) = stream.write_all(json::encode(&reading).unwrap().as_bytes()) { break; }
            if let Err(_) = stream.write_all(b"\n\n") { break; }
            if let Err(_) = stream.flush() { break; }

            thread::sleep(Duration::from_millis(100));
        }

        println!("done.");
        return Ok(nickel::Halt(stream));
    });

    // Print local IP
    println!("LAN: http://{}/", local_ip::get().unwrap());

    server.listen("0.0.0.0:80");
}

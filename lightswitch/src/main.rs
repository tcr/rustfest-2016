#![feature(alloc_system)]

extern crate alloc_system;
extern crate ble;
extern crate servo_pca9685;
extern crate tessel;

use servo_pca9685::ServoArray;
use std::thread;
use std::time::Duration;
use tessel::Tessel;

fn main() {
    // Acquire port A.
    let (port_a, _) = Tessel::ports().unwrap();

    // Create the accelerometer object and connect to the sensor.
    let mut servos = ServoArray::new(port_a, false, false);
    servos.connect().expect("Could not connect to servo array.");
    servos.set_duty_cycle(1, 0.12);

    println!("Starting BLE scan...");

    let mut scan = ble::scan().unwrap();

    for discovery in scan {
        if discovery.name() == "Pebble Time LE B355" {
            servos.set_duty_cycle(1, 0.05);
        }
    }

    println!("... done.");
}

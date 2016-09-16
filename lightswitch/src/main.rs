// Required (for now) to compile programs that run on Tessel.
#![feature(alloc_system)]
extern crate alloc_system;

// Bluetooth Low Energy crate.
extern crate ble;
// Servo crate for hte PCA9685 chip (the Tessel Servo Module).
extern crate servo_pca9685;
// Crate for the Tessel microcontroller.
extern crate tessel;

use servo_pca9685::ServoArray;
use std::thread;
use std::time::Duration;
use tessel::Tessel;

fn main() {
    // Tessel has two ports. We want to acquire port A.
    let (port_a, _) = Tessel::ports().unwrap();

    // Create a connection to the servo module, which supports up to 16 servos.
    // The `false, false` is extra configuration.
    let mut servos = ServoArray::new(port_a, false, false);
    servos.connect().expect("Could not connect to servo array.");

    // Set the first servo (1-indexed) all the way to its highest point. Stock
    // servos have a maximum duty cycle (percentage their PWM value is high) of
    // 0.12. In a future API, we'll support the range 0.05..0.12 as a default
    // range, and this value will be 1.0 for "max".
    servos.set_duty_cycle(1, 0.12);

    // Start the Bluetooth Low Energy scan. This continuously cans in the
    // background until the scan.stop() method is called.
    println!("Starting BLE scan...");
    let mut scan = ble::scan().unwrap();

    // While scanning, we can iterate over the results synchronously. The
    // iterator returns discovery objects which have a name and a MAC address
    // of the device that was found.
    for discovery in scan {
        // Here, we are looking for a device by the name of my smartwatch, with
        // the unique ID "Pebble Time LE B355".
        if discovery.name() == "Pebble Time LE B355" {
            // We turn the servo all the way to its minimum setting, which on
            // stock servos is about 0.05 (or lesser). This triggers the switch.
            servos.set_duty_cycle(1, 0.05);
            // There is no way to reset the servo now, but perhaps you'd like
            // to add one after a timeout? After someone else enters the room?
            // Go nuts!
        }
    }

    // We'll never get to this point, but it inspires a sense of satisfaction:
    println!("... done.");
}

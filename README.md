# RustFest.eu 2016 Demos

Here are the demos I presented at RustFest.eu for the talk "Sensors, Servos, and
Signals with Rust."

All demos require a nightly compiler to enable the `alloc_system` feature. All
demos written to run in the Tessel 2 platform. You should [read the `tessel-rust` README](http://github.com/tessel/tessel-rust) for more details on how to compile these.

### lightswitch

Turns a servo when it detects a Bluetooth Low Energy signal of a particular name.
Can be used to turn a lightswitch on in your house as you walk into a room (by
detecting your phone or smartwatch).

https://www.youtube.com/watch?v=WEB2vzmAjRY&feature=youtu.be

### fanremote

A web page that shows the current temperature in an environment. Also supports
turning on a relay through a POST request. Can be used to turn on an air
conditioner or fan remotely.

https://www.youtube.com/watch?v=YQkMxud8-xg&feature=youtu.be

## License

MIT or Apache-2.0, at your option.

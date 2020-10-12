# gps-monitor

Very small program that glues together a couple of crates in order to read
NMEA-0183 sentences from a serial port and print out the debug representation
of the parsed sentences.

## Usage

### Build and Run

    cargo run --release -- /path/to/serial/port

Where `/path/to/serial/port` is something like `/dev/ttyUSB0` on Linux.

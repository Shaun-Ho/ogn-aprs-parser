# OGN APRS Parser

A Rust library for parsing Open Glider Network (OGN) Automatic Packet Reporting System (APRS) messages.

This crate uses `nom` parser combinator library to efficiently extract aircraft telemetry, routing data, and tracker hardware metadata from raw OGN APRS byte streams.

### Basic Example

Here is a quick example of how to parse a raw OGN APRS byte slice into an `AircraftBeacon` struct.

```rust
use ogn_aprs_parser::parse_ogn_aprs_aircraft_beacon;

fn main() {
    // A sample raw OGN APRS message (e.g., received via TCP from the OGN APRS servers)
    let raw_message = b"ICA4400DC>OGADSB,qAS,HLST:/190606h5158.29N/01013.06E^066/488/A=034218 !W10! id254400DC -832fpm FL353.00 A3:EJU47ML";

    match parse_ogn_aprs_aircraft_beacon(raw_message) {
        Ok(beacon) => {
            println!("Callsign: {}", beacon.callsign);           // ICA4400DC
            println!("Receiver: {}", beacon.receiver);           // HLST
            println!("Protocol: {:?}", beacon.ogn_aprs_protocol);// OGADSB
            println!("Time UTC: {}", beacon.time);               // 19:06:06
            println!("Latitude: {:.4}", beacon.latitude);        // 51.9715
            println!("Longitude: {:.4}", beacon.longitude);      // 10.2177
            println!("Altitude: {} ft", beacon.gps_altitude);    // 34218

            // The parser extracts and decodes the embedded tracker ID metadata
            let prefix = beacon.ogn_beacon_id.prefix;
            println!("Aircraft Type: {:?}", prefix.aircraft_type); // DropPlane (example)
            println!("Address Type: {:?}", prefix.address_type);   // ICAO
            println!("Stealth Mode: {}", prefix.stealth_mode);     // false
        }
        Err(e) => {
            eprintln!("Failed to parse aircraft beacon: {}", e);
        }
    }
}
```

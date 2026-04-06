# OGN APRS Parser

A fast, fully-typed Python wrapper for the `ogn_aprs_parser` Rust crate.

This library parses raw Open Glider Network (OGN) APRS aircraft beacon messages into structured Python objects. It extracts telemetric data (coordinates, ground track, speed, altitude) and aircraft identification metadata (ICAO address, aircraft type, stealth flags).

## Installation

You can install the package directly via pip:

```bash
pip install ogn-aprs-parser
```

## Quickstart

from ogn_aprs_parser import parse_ogn_aprs_aircraft_beacon

# Example raw OGN APRS byte string

```python
raw_message = b"FLRDD3B4B>APRS,qAS,Larnaca:/074548h3453.52N/03338.30E'086/039/A=000918 !W64! id0ADD3B4B +020fpm -1.0rot 55.2dB 0e -4.3kHz gps2x3"

try:
    # Parse the raw bytes
    beacon = parse_ogn_aprs_aircraft_beacon(raw_message)

    # Access telemetry
    print(f"Callsign:  {beacon.callsign}")
    print(f"Altitude:  {beacon.gps_altitude} meters")
    print(f"Speed:     {beacon.ground_speed} m/s")
    print(f"Heading:   {beacon.ground_track} degrees")
    print(f"Time:      {beacon.time}")

    # Access internal ID metadata
    print(f"Aircraft:  {beacon.ogn_beacon_id.prefix.aircraft_type.name}")
    print(f"No Track:  {beacon.ogn_beacon_id.prefix.no_track}")
    print(f"ICAO:      {beacon.ogn_beacon_id.icao_address}")

except ValueError as e:
    print(f"Failed to parse beacon: {e}")

```

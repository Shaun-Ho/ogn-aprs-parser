from __future__ import annotations

import datetime

import pytest

from ogn_aprs_parser import (
    AircraftBeacon,
    ICAOAddress,
    OGNAddressType,
    OGNAircraftType,
    OgnAprsProtocol,
    OGNBeaconID,
    OGNIDPrefix,
    parse_ogn_aprs_aircraft_beacon,
)


@pytest.mark.parametrize(
    ("message", "expected"),
    [
        (
            rb"ICA4CA1FF>OGADSB,qAS,LEMDadsb:/140833h4044.38N\00356.29W^024/426/A=040000 id254CA1FF +000fpm  fnRYR6ZM",
            AircraftBeacon(
                callsign="ICA4CA1FF",
                q_construct="qAS",
                receiver="LEMDadsb",
                ogn_aprs_protocol=OgnAprsProtocol.OGADSB,
                time=datetime.time(hour=14, minute=8, second=33),
                latitude=40.739666666666665,
                longitude=-3.9381666666666666,
                ground_track=24.0,
                ground_speed=426.0,
                gps_altitude=40000.0,
                ogn_beacon_id=OGNBeaconID(
                    prefix=OGNIDPrefix(
                        OGNAircraftType.JET_TURBOPROP_AIRCRAFT,
                        address_type=OGNAddressType.ICAO,
                        no_track=False,
                        stealth_mode=False,
                    ),
                    icao_address=ICAOAddress(5022207),
                ),
            ),
        ),
        (
            rb"ICA4CA4EB>OGADSB,qAS,LEMDadsb:/142346h4034.03N\00315.64W^008/370/A=038000 id254CA4EB +000fpm  0.0rot fnRYR4057  regEI-DPG modelB738",
            AircraftBeacon(
                callsign="ICA4CA4EB",
                q_construct="qAS",
                receiver="LEMDadsb",
                ogn_aprs_protocol=OgnAprsProtocol.OGADSB,
                time=datetime.time(hour=14, minute=23, second=46),
                latitude=40.567166666666665,
                longitude=-3.260666666666667,
                ground_track=8.0,
                ground_speed=370.0,
                gps_altitude=38000.0,
                ogn_beacon_id=OGNBeaconID(
                    prefix=OGNIDPrefix(
                        OGNAircraftType.JET_TURBOPROP_AIRCRAFT,
                        address_type=OGNAddressType.ICAO,
                        no_track=False,
                        stealth_mode=False,
                    ),
                    icao_address=ICAOAddress(5022955),
                ),
            ),
        ),
        (
            rb"ICAA8CBA8>OGFLR,qAS,MontCAIO:/231150z4512.12N\01059.03E^192/106/A=009519 !W20! id21A8CBA8 -039fpm +0.0rot 3.5dB 2e -8.7kHz gps1x2 s6.09 h43 rDF0267",
            AircraftBeacon(
                callsign="ICAA8CBA8",
                q_construct="qAS",
                receiver="MontCAIO",
                ogn_aprs_protocol=OgnAprsProtocol.OGFLR,
                time=datetime.time(hour=23, minute=11, second=50),
                latitude=45.202,
                longitude=10.983833333333333,
                ground_track=192.0,
                ground_speed=106.0,
                gps_altitude=9519.0,
                ogn_beacon_id=OGNBeaconID(
                    prefix=OGNIDPrefix(
                        OGNAircraftType.RECIPROCATING_ENGINE_AIRCRAFT,
                        address_type=OGNAddressType.ICAO,
                        no_track=False,
                        stealth_mode=False,
                    ),
                    icao_address=ICAOAddress(11062184),
                ),
            ),
        ),
        (
            rb"FLR200295>OGFLR,qAS,TT:/071005h4613.92N/01427.53Eg000/000/A=001313 !W00! id1E200295 +000fpm +0.0rot 37.0dB -1.8kHz gps3x5",
            AircraftBeacon(
                callsign="FLR200295",
                q_construct="qAS",
                receiver="TT",
                ogn_aprs_protocol=OgnAprsProtocol.OGFLR,
                time=datetime.time(hour=7, minute=10, second=5),
                latitude=46.232,
                longitude=14.458833333333333,
                ground_track=0.0,
                ground_speed=0.0,
                gps_altitude=1313.0,
                ogn_beacon_id=OGNBeaconID(
                    prefix=OGNIDPrefix(
                        OGNAircraftType.PARAGLIDER,
                        address_type=OGNAddressType.FLARM,
                        no_track=False,
                        stealth_mode=False,
                    ),
                    icao_address=ICAOAddress(2097813),
                ),
            ),
        ),
        (
            rb"SKY3E5906>OGNSKY,qAS,SafeSky:/072449h5103.95N/00524.50E'193/034/A=001250 !W65! id1C3E5906 +000fpm gps4x1",
            AircraftBeacon(
                callsign="SKY3E5906",
                q_construct="qAS",
                receiver="SafeSky",
                ogn_aprs_protocol=OgnAprsProtocol.OGNSKY,
                time=datetime.time(hour=7, minute=24, second=49),
                latitude=51.06583333333333,
                longitude=5.408333333333333,
                ground_track=193.0,
                ground_speed=34.0,
                gps_altitude=1250.0,
                ogn_beacon_id=OGNBeaconID(
                    prefix=OGNIDPrefix(
                        OGNAircraftType.PARAGLIDER,
                        address_type=OGNAddressType.UNKNOWN,
                        no_track=False,
                        stealth_mode=False,
                    ),
                    icao_address=ICAOAddress(4086022),
                ),
            ),
        ),
    ],
)
def test_parsed_beacon_structure(message: bytes, expected: AircraftBeacon) -> None:
    parsed = parse_ogn_aprs_aircraft_beacon(message)

    assert parsed == expected

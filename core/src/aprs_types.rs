//! Data types and structures for OGN (Open Glider Network) APRS messages.
//!
//! This module defines the core types used in OGN telemetry, including
//! protocols, device address types, aircraft types, and the bitfield logic
//! used to decode the unique OGN Beacon ID prefix.
//!
//! Specific details about the different enum values can be found on the [OGN website](http://wiki.glidernet.org/wiki:ogn-flavoured-aprs)

use crate::errors::{
    APRSMessageParseError, APRSParseContext, ICAOAddressError, OGNAddressTypeError,
    OGNAircraftTypeError, OGNBeaconIDError, OGNIDPrefixError,
};

/// Supported OGN APRS protocols indicating the source or format of the data.
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, strum_macros::EnumString)]
pub enum OgnAprsProtocol {
    /// OGN ADS-B rebroadcast.
    OGADSB,
    /// FLARM tracker rebroadcast.
    OGFLR,
    /// SafeSky rebroadcast.
    OGNSKY,
}

impl OgnAprsProtocol {
    /// Parses a string into an [`OgnAprsProtocol`].
    ///
    /// # Arguments
    ///
    /// * `s` - The string slice representing the protocol (e.g., `"OGADSB"`).
    ///
    /// # Errors
    ///
    /// Returns an [`APRSMessageParseError`] if the string does not match any
    /// supported protocol.
    pub fn parse_protocol(s: &str) -> Result<Self, APRSMessageParseError> {
        s.parse::<Self>().map_err(|_| {
            APRSMessageParseError::InvalidOGNAprsProtocol(APRSParseContext {
                input: s.to_owned(),
                message: "Unsupported OGN APRS Protocol".to_owned(),
            })
        })
    }
}

/// A 24-bit ICAO aircraft address or a generic 24-bit tracker ID.
///
/// Internally represented as a `u32`, but restricted upon creation to ensure
/// it does not exceed the maximum 24-bit value (`0x00FF_FFFF`).
#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub struct ICAOAddress(u32);

impl ICAOAddress {
    /// The maximum allowed value for a 24-bit ICAO address.
    pub const MAX_VALUE: u32 = 0x00FF_FFFF;

    /// Creates a new [`ICAOAddress`].
    ///
    /// # Arguments
    ///
    /// * `value` - The raw `u32` value to wrap.
    ///
    /// # Errors
    ///
    /// Returns an [`ICAOAddressError::InvalidAddress`] if the value is strictly
    /// greater than `0x00FF_FFFF`.
    pub fn new(value: u32) -> Result<Self, ICAOAddressError> {
        if value <= Self::MAX_VALUE {
            Ok(ICAOAddress(value))
        } else {
            Err(ICAOAddressError::InvalidAddress(value))
        }
    }

    /// Returns the underlying raw integer value.
    #[must_use]
    pub fn value(&self) -> u32 {
        self.0
    }
}

impl std::fmt::Display for ICAOAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:08X}", self.0)
    }
}

/// Represents a complete OGN beacon identifier.
///
/// In OGN APRS, a beacon ID is composed of a 1-byte metadata prefix (encoding
/// things like aircraft type and tracker type) and a 3-byte (24-bit) address.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct OGNBeaconID {
    /// The decoded metadata extracted from the first byte of the beacon ID.
    pub prefix: OGNIDPrefix,
    /// The 24-bit aircraft ICAO address.
    pub icao_address: ICAOAddress,
}

impl OGNBeaconID {
    /// Constructs a new [`OGNBeaconID`] from a given prefix and address.
    #[must_use]
    pub fn new(prefix: OGNIDPrefix, icao_address: ICAOAddress) -> Self {
        OGNBeaconID {
            prefix,
            icao_address,
        }
    }
}
impl std::str::FromStr for OGNBeaconID {
    type Err = OGNBeaconIDError;

    /// Parses an 8-character hex string into an [`OGNBeaconID`].
    ///
    /// Expected format: `XXYYYYYY` where:
    /// * `XX` is the 2-character hex representation of the 1-byte prefix.
    /// * `YYYYYY` is the 6-character hex representation of the 24-bit address.
    ///
    /// # Errors
    ///
    /// Returns an [`OGNBeaconIDError`] if the string length is not exactly 8,
    /// or if the prefix/address components fail hex conversion.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 8 {
            let prefix_hex = &s[..2]; // "XX"
            let address_hex = &s[2..]; // "YYYYYY"

            let prefix = OGNIDPrefix::from_hex_str(prefix_hex)
                .map_err(OGNBeaconIDError::OGNIDPrefixError)?;
            let icao_address =
                ICAOAddress::new(u32::from_str_radix(address_hex, 16).map_err(|_| {
                    OGNBeaconIDError::ICAOAddressError(ICAOAddressError::InvalidHexFormat)
                })?)
                .map_err(OGNBeaconIDError::ICAOAddressError)?;

            Ok(OGNBeaconID::new(prefix, icao_address))
        } else {
            Err(OGNBeaconIDError::InvalidOGNBeaconFormat(s.to_string()))
        }
    }
}

/// Identifies the namespace or hardware type of the 24-bit address.
///
/// These values are mapped from the lower 2 bits (bits 0-1) of the [OGN ID prefix](`OGNIDPrefix`).
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum OGNAddressType {
    /// Unknown address assignment.
    Unknown = 0,
    /// Official ICAO assigned aircraft address.
    ICAO = 1,
    /// Address assigned by FLARM hardware.
    FLARM = 2,
    /// Address assigned by an OGN Tracker.
    OGNTracker = 3,
}
impl OGNAddressType {
    pub fn from_u8(value: u8) -> Result<Self, OGNAddressTypeError> {
        match value {
            0 => Ok(OGNAddressType::Unknown),
            1 => Ok(OGNAddressType::ICAO),
            2 => Ok(OGNAddressType::FLARM),
            3 => Ok(OGNAddressType::OGNTracker),
            other => Err(OGNAddressTypeError::InvalidAddressType(other)),
        }
    }
}

/// The decoded metadata parsed from the 1-byte OGN ID prefix.
///
/// The OGN network embeds privacy requests and object type classification into
/// the first byte of a tracker's ID string using specific bitwise flags.
///
/// # Bit Layout
/// * **Bits 0-1:** Device Address Type (2 bits) -> [`OGNAddressType`]
/// * **Bits 2-5:** Aircraft / Object Type (4 bits) -> [`OGNAircraftType`]
/// * **Bit 6:** No-Track Flag (1 bit). If true, do not record trace logs.
/// * **Bit 7:** Stealth Mode Flag (1 bit). If true, delay or hide on live maps.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct OGNIDPrefix {
    /// Aircraft type - defined by [`OGNAircraftType`]
    pub aircraft_type: OGNAircraftType,
    /// The protocol that issued the tracker ID - one of [`OGNAddressType`].
    pub address_type: OGNAddressType,
    /// If true, tracking platforms should not log historical traces of this flight.
    pub no_track: bool,
    /// If true, tracking platforms should hide or blur live identification.
    pub stealth_mode: bool,
}

impl OGNIDPrefix {
    /// Parses an [`OGNIDPrefix`] directly from an 8-bit unsigned integer.
    ///
    /// Bitwise shifting and masking are applied to extract the underlying values.
    ///
    /// # Errors
    ///
    /// Returns an [`OGNIDPrefixError`] if the extracted bits map to invalid
    /// enum variants for either [`OGNAircraftType`] or [`OGNAddressType`].
    pub fn new(value: u8) -> Result<Self, OGNIDPrefixError> {
        let raw_type = (value >> 2) & 0b1111; // extract 4 bits
        let aircraft_type =
            OGNAircraftType::from_u8(raw_type).map_err(OGNIDPrefixError::AircraftType)?;

        let raw_address = value & 0b11; // extract 2 bits
        let address_type =
            OGNAddressType::from_u8(raw_address).map_err(OGNIDPrefixError::AddressType)?;

        let no_track = ((value >> 6) & 0b1) == 1;
        let stealth_mode = ((value >> 7) & 0b1) == 1;
        Ok(OGNIDPrefix {
            aircraft_type,
            address_type,
            no_track,
            stealth_mode,
        })
    }

    /// Parses an [`OGNIDPrefix`] from a 2-character hexadecimal string.
    ///
    /// # Errors
    ///
    /// Returns an [`OGNIDPrefixError::HexFormat`] if the string is not exactly
    /// 2 characters long, or if it cannot be parsed as base-16 hex.
    pub fn from_hex_str(s: &str) -> Result<Self, OGNIDPrefixError> {
        if s.len() != 2 {
            return Err(OGNIDPrefixError::HexFormat);
        }
        let parsed_value = u8::from_str_radix(s, 16).map_err(|_| OGNIDPrefixError::HexFormat)?;

        OGNIDPrefix::new(parsed_value)
    }
}

/// Categorizes the type of aircraft or object carrying the tracker.
///
/// These values are extracted from a 4-bit chunk (bits 2-5) within the OGN ID
/// prefix. Value 0 and 14 are reserved.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum OGNAircraftType {
    Reserved = 0,
    Glider = 1,
    TowPlane = 2,
    Helicopter = 3,
    Parachute = 4,
    DropPlane = 5,
    HangGlider = 6,
    Paraglider = 7,
    ReciprocatingEngineAircraft = 8,
    JetTurbopropAircraft = 9,
    Unknown = 10,
    Balloon = 11,
    Airship = 12,
    UAVs = 13,
    StaticObstacle = 15,
}

impl OGNAircraftType {
    /// Maps a raw 4-bit unsigned integer (0-15) to its corresponding
    /// [`OGNAircraftType`].
    ///
    /// Note that both values `0` and `14` are designated as `Reserved`.
    ///
    /// # Errors
    ///
    /// Returns an [`OGNAircraftTypeError::InvalidEnum`] if the value is
    /// out of the 0-15 bounds (though practically impossible if correctly masked
    /// from 4 bits).
    pub fn from_u8(value: u8) -> Result<Self, OGNAircraftTypeError> {
        match value {
            0 | 14 => Ok(OGNAircraftType::Reserved),
            1 => Ok(OGNAircraftType::Glider),
            2 => Ok(OGNAircraftType::TowPlane),
            3 => Ok(OGNAircraftType::Helicopter),
            4 => Ok(OGNAircraftType::Parachute),
            5 => Ok(OGNAircraftType::DropPlane),
            6 => Ok(OGNAircraftType::HangGlider),
            7 => Ok(OGNAircraftType::Paraglider),
            8 => Ok(OGNAircraftType::ReciprocatingEngineAircraft),
            9 => Ok(OGNAircraftType::JetTurbopropAircraft),
            10 => Ok(OGNAircraftType::Unknown),
            11 => Ok(OGNAircraftType::Balloon),
            12 => Ok(OGNAircraftType::Airship),
            13 => Ok(OGNAircraftType::UAVs),
            15 => Ok(OGNAircraftType::StaticObstacle),
            other => Err(OGNAircraftTypeError::InvalidEnum(other)),
        }
    }
}

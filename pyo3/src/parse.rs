use pyo3::PyResult;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use crate::types::PyAircraftBeacon;

#[cfg(feature = "stubgen")]
use pyo3_stub_gen::derive::gen_stub_pyfunction;

/// Parses a raw OGN APRS aircraft beacon message.
///
/// This is the primary entry point for decoding an OGN beacon. It extracts
/// the header information, positional block, and trailing extension tags from
/// the raw byte stream.
///
/// # Arguments
///
/// * `input` (`bytes`): A bytes-like object containing the raw APRS message.
///
/// # Returns
///
/// `AircraftBeacon`: The successfully parsed beacon object.
///
/// # Raises
///
/// * `ValueError`: If the parsing fails. This typically occurs if:
///   * The overall message format is invalid.
///   * A specific token (time, coordinates, etc.) fails to parse.
///   * The required OGN Beacon ID is missing from the message.
#[cfg_attr(feature = "stubgen", gen_stub_pyfunction)]
#[pyfunction]
#[pyo3(name = "parse_ogn_aprs_aircraft_beacon")]
pub fn parse_ogn_aprs_aircraft_beacon_py(
    input: std::borrow::Cow<[u8]>,
) -> PyResult<PyAircraftBeacon> {
    ogn_aprs_parser::parse_ogn_aprs_aircraft_beacon(&input)
        .map(PyAircraftBeacon::from)
        .map_err(|e| PyValueError::new_err(format!("Parse error: {:?}", e)))
}

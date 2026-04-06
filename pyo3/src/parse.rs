use pyo3::PyResult;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use crate::types::PyAircraftBeacon;

#[cfg(feature = "stubgen")]
use pyo3_stub_gen::derive::gen_stub_pyfunction;

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

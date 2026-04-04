use pyo3::prelude::*;

#[pymodule]
mod ogn_aprs_parser_pyo3 {
    use crate::types::{
        PyAircraftBeacon, PyICAOAddress, PyOGNAddressType, PyOGNAircraftType, PyOGNBeaconID,
        PyOGNIDPrefix, PyOgnAprsProtocol,
    };
    use pyo3::PyResult;
    use pyo3::exceptions::PyValueError;
    use pyo3::prelude::*;

    #[cfg(feature = "stubgen")]
    use pyo3_stub_gen::derive::gen_stub_pyfunction;

    #[pymodule_init]
    fn init(m: &Bound<'_, PyModule>) -> PyResult<()> {
        m.add_function(wrap_pyfunction!(parse_ogn_aprs_aircraft_beacon_py, m)?)?;
        m.add_class::<PyAircraftBeacon>()?;
        m.add_class::<PyOGNAircraftType>()?;
        m.add_class::<PyICAOAddress>()?;
        m.add_class::<PyOGNBeaconID>()?;
        m.add_class::<PyOGNIDPrefix>()?;
        m.add_class::<PyOGNAddressType>()?;
        m.add_class::<PyOgnAprsProtocol>()?;
        Ok(())
    }

    #[cfg_attr(feature = "stubgen", gen_stub_pyfunction)]
    #[pyfunction]
    #[pyo3(name = "parse_ogn_aprs_aircraft_beacon")]
    fn parse_ogn_aprs_aircraft_beacon_py(
        input: std::borrow::Cow<[u8]>,
    ) -> PyResult<PyAircraftBeacon> {
        ogn_aprs_parser::parse_ogn_aprs_aircraft_beacon(&input)
            .map(PyAircraftBeacon::from)
            .map_err(|e| PyValueError::new_err(format!("Parse error: {:?}", e)))
    }
}

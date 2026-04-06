use pyo3::prelude::*;

#[pymodule]
mod ogn_aprs_parser_pyo3 {
    use crate::parse::parse_ogn_aprs_aircraft_beacon_py;
    use crate::types::{
        PyAircraftBeacon, PyICAOAddress, PyOGNAPRSProtocol, PyOGNAddressType, PyOGNAircraftType,
        PyOGNBeaconID, PyOGNIDPrefix,
    };

    use pyo3::PyResult;
    use pyo3::prelude::*;

    #[pymodule_init]
    fn init(m: &Bound<'_, PyModule>) -> PyResult<()> {
        m.add_function(wrap_pyfunction!(parse_ogn_aprs_aircraft_beacon_py, m)?)?;

        m.add_class::<PyAircraftBeacon>()?;
        m.add_class::<PyICAOAddress>()?;
        m.add_class::<PyOGNAddressType>()?;
        m.add_class::<PyOGNAircraftType>()?;
        m.add_class::<PyOGNAPRSProtocol>()?;
        m.add_class::<PyOGNBeaconID>()?;
        m.add_class::<PyOGNIDPrefix>()?;

        Ok(())
    }
}

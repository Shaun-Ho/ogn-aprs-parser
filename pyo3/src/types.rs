use pyo3::{exceptions::PyValueError, prelude::*};

pub use self::enums::{PyOGNAddressType, PyOGNAircraftType, PyOgnAprsProtocol};

#[cfg(feature = "stubgen")]
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};

#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[pyclass(name = "AircraftBeacon", eq, from_py_object)]
#[derive(Clone, Debug, PartialEq)]
pub struct PyAircraftBeacon {
    #[pyo3(get, set)]
    pub callsign: String,
    #[pyo3(get, set)]
    pub q_construct: String,
    #[pyo3(get, set)]
    pub receiver: String,
    #[pyo3(get, set)]
    pub ogn_aprs_protocol: PyOgnAprsProtocol,
    #[pyo3(get, set)]
    pub time: chrono::NaiveTime,
    #[pyo3(get, set)]
    pub latitude: f64,
    #[pyo3(get, set)]
    pub longitude: f64,
    #[pyo3(get, set)]
    pub ground_track: f64,
    #[pyo3(get, set)]
    pub ground_speed: f64,
    #[pyo3(get, set)]
    pub gps_altitude: f64,
    #[pyo3(get, set)]
    pub ogn_beacon_id: PyOGNBeaconID,
}
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl PyAircraftBeacon {
    #[allow(clippy::too_many_arguments)]
    #[new]
    fn py_new(
        callsign: String,
        q_construct: String,
        receiver: String,
        ogn_aprs_protocol: PyOgnAprsProtocol,
        time: chrono::NaiveTime,
        latitude: f64,
        longitude: f64,
        ground_track: f64,
        ground_speed: f64,
        gps_altitude: f64,
        ogn_beacon_id: PyOGNBeaconID,
    ) -> Self {
        Self {
            callsign,
            q_construct,
            receiver,
            ogn_aprs_protocol,
            time,
            latitude,
            longitude,
            ground_track,
            ground_speed,
            gps_altitude,
            ogn_beacon_id,
        }
    }

    #[allow(unused_variables, clippy::too_many_arguments)]
    fn __init__(
        &self,
        callsign: String,
        q_construct: String,
        receiver: String,
        ogn_aprs_protocol: PyOgnAprsProtocol,
        time: chrono::NaiveTime,
        latitude: f64,
        longitude: f64,
        ground_track: f64,
        ground_speed: f64,
        gps_altitude: f64,
        ogn_beacon_id: PyOGNBeaconID,
    ) -> PyResult<()> {
        Ok(())
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self)
    }
}
impl From<ogn_aprs_parser::AircraftBeacon> for PyAircraftBeacon {
    fn from(value: ogn_aprs_parser::AircraftBeacon) -> Self {
        PyAircraftBeacon {
            callsign: value.callsign,
            q_construct: value.q_construct,
            receiver: value.receiver,
            ogn_aprs_protocol: value.ogn_aprs_protocol.into(),
            time: value.time,
            latitude: value.latitude,
            longitude: value.longitude,
            ground_track: value.ground_track,
            ground_speed: value.ground_speed,
            gps_altitude: value.gps_altitude,
            ogn_beacon_id: value.ogn_beacon_id.into(),
        }
    }
}

#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[pyclass(name = "ICAOAddress", eq, from_py_object)]
#[derive(Clone, Debug, PartialEq)]
pub struct PyICAOAddress(pub u32);
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl PyICAOAddress {
    #[new]
    fn py_new(value: u32) -> PyResult<Self> {
        let py_icao_address = ogn_aprs_parser::ICAOAddress::new(value)
            .map(Self::from)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(py_icao_address)
    }
    fn __repr__(&self) -> String {
        format!("{:?}", self)
    }
}
impl From<ogn_aprs_parser::ICAOAddress> for PyICAOAddress {
    fn from(value: ogn_aprs_parser::ICAOAddress) -> Self {
        PyICAOAddress(value.value())
    }
}

#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[pyclass(name = "OGNBeaconID", eq, from_py_object)]
#[derive(Clone, Debug, PartialEq)]
pub struct PyOGNBeaconID {
    #[pyo3(get, set)]
    pub prefix: PyOGNIDPrefix,
    #[pyo3(get, set)]
    pub icao_address: PyICAOAddress,
}
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl PyOGNBeaconID {
    #[new]
    fn py_new(prefix: PyOGNIDPrefix, icao_address: PyICAOAddress) -> Self {
        Self {
            prefix,
            icao_address,
        }
    }
    fn __repr__(&self) -> String {
        format!("{:?}", self)
    }
}
impl From<ogn_aprs_parser::OGNBeaconID> for PyOGNBeaconID {
    fn from(value: ogn_aprs_parser::OGNBeaconID) -> Self {
        PyOGNBeaconID {
            prefix: value.prefix.into(),
            icao_address: value.icao_address.into(),
        }
    }
}

#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[pyclass(name = "OGNIDPrefix", eq, from_py_object)]
#[derive(Clone, Debug, PartialEq)]
pub struct PyOGNIDPrefix {
    #[pyo3(get, set)]
    pub aircraft_type: PyOGNAircraftType,
    #[pyo3(get, set)]
    pub address_type: PyOGNAddressType,
    #[pyo3(get, set)]
    pub no_track: bool,
    #[pyo3(get, set)]
    pub stealth_mode: bool,
}
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl PyOGNIDPrefix {
    #[new]
    fn py_new(
        aircraft_type: PyOGNAircraftType,
        address_type: PyOGNAddressType,
        no_track: bool,
        stealth_mode: bool,
    ) -> Self {
        Self {
            aircraft_type,
            address_type,
            no_track,
            stealth_mode,
        }
    }
}
impl From<ogn_aprs_parser::OGNIDPrefix> for PyOGNIDPrefix {
    fn from(value: ogn_aprs_parser::OGNIDPrefix) -> Self {
        PyOGNIDPrefix {
            aircraft_type: value.aircraft_type.into(),
            address_type: value.address_type.into(),
            no_track: value.no_track,
            stealth_mode: value.stealth_mode,
        }
    }
}

#[allow(clippy::upper_case_acronyms)]
mod enums {
    macro_rules! mirror_enum {
        ($src:ty, $name:ident, $pyname:ident, [$($v:ident),* $(,)?]) => {
            #[allow(non_snake_case)]
            mod $pyname {
                use super::super::*;
                #[cfg(feature = "stubgen")]
                use pyo3_stub_gen::derive::gen_stub_pyclass_enum;

                // The inner enum is exact python enum name
                #[cfg_attr(feature = "stubgen", gen_stub_pyclass_enum)]
                #[pyclass(
                    eq,
                    eq_int,
                    rename_all = "SCREAMING_SNAKE_CASE",
                    from_py_object
                )]
                #[derive(Copy, Clone, Debug, PartialEq, Eq)]
                pub enum $pyname {
                    $($v = <$src>::$v as isize),*
                }

                #[pymethods]
                impl $pyname {
                    fn __repr__(&self) -> String {
                        format!("{:?}", self)
                    }
                }

                impl From<$src> for $pyname {
                    fn from(v: $src) -> Self {
                        #[allow(unreachable_patterns)]
                        match v {
                            $(<$src>::$v => Self::$v,)*
                            _ => panic!("Unmapped variant in {}", stringify!($src)),
                        }
                    }
                }

                impl From<$pyname> for $src {
                    fn from(v: $pyname) -> Self {
                        match v {
                            $($pyname::$v => <$src>::$v,)*
                        }
                    }
                }
            }

            // export python named enum as the rust enum
            pub use $pyname::$pyname as $name;
        };
    }

    mirror_enum!(
        ogn_aprs_parser::OgnAprsProtocol,
        PyOgnAprsProtocol,
        OgnAprsProtocol,
        [OGADSB, OGFLR, OGNSKY]
    );

    mirror_enum!(
        ogn_aprs_parser::OGNAircraftType,
        PyOGNAircraftType,
        OGNAircraftType,
        [
            Reserved,
            Glider,
            TowPlane,
            Helicopter,
            Parachute,
            DropPlane,
            HangGlider,
            Paraglider,
            ReciprocatingEngineAircraft,
            JetTurbopropAircraft,
            Unknown,
            Balloon,
            Airship,
            UAVs,
            StaticObstacle,
        ]
    );

    mirror_enum!(
        ogn_aprs_parser::OGNAddressType,
        PyOGNAddressType,
        OGNAddressType,
        [Unknown, ICAO, FLARM, OGNTracker]
    );
}

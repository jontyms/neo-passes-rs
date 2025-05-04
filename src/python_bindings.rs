use crate::pass::barcode::{Barcode, BarcodeFormat};
use crate::{Package, Pass, resource, sign};
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use std::{fs::File, io::Read};

#[pyclass]
pub struct PyPassConfig {
    #[pyo3(get, set)]
    pub organization_name: String,
    #[pyo3(get, set)]
    pub description: String,
    #[pyo3(get, set)]
    pub pass_type_identifier: String,
    #[pyo3(get, set)]
    pub team_identifier: String,
    #[pyo3(get, set)]
    pub serial_number: String,
}

#[pymethods]
impl PyPassConfig {
    #[new]
    fn new(
        organization_name: String,
        description: String,
        pass_type_identifier: String,
        team_identifier: String,
        serial_number: String,
    ) -> Self {
        Self {
            organization_name,
            description,
            pass_type_identifier,
            team_identifier,
            serial_number,
        }
    }
}

// Takes in a pass.json object as config and other options and writes a pkpass to file location
#[pyfunction]
#[pyo3(signature = (
    config,
    cert_path,
    key_path,
    output_path,
    icon_path = None,
    icon2x_path = None,
))]
fn generate_pass(
    config: &str,
    cert_path: &str,
    key_path: &str,
    output_path: &str,
    icon_path: Option<&str>,
    icon2x_path: Option<&str>,
) -> PyResult<()> {
    /* -------- build pass -------- */
    let pass = Pass::from_json(config).unwrap();

    let mut package = Package::new(pass);

    /* ---------- icons ----------- */
    if let Some(p) = icon_path {
        let f = File::open(p)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;
        package
            .add_resource(resource::Type::Icon(resource::Version::Standard), f)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
    }
    if let Some(p) = icon2x_path {
        let f = File::open(p)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;
        package
            .add_resource(resource::Type::Icon(resource::Version::Size2X), f) // @2x
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
    }
    /* ---------------------------- */

    /* ---- read cert & key ---- */
    let mut cert = Vec::new();
    File::open(cert_path)
        .and_then(|mut f| f.read_to_end(&mut cert))
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("Cert error: {e}")))?;

    let mut key = Vec::new();
    File::open(key_path)
        .and_then(|mut f| f.read_to_end(&mut key))
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("Key error: {e}")))?;

    let scfg = sign::SignConfig::new(sign::WWDR::G4, &cert, &key)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Sign cfg: {e}")))?;
    package.add_certificates(scfg);

    /* ---- write .pkpass ---- */
    let outfile = File::create(output_path)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("Output error: {e}")))?;
    package
        .write(outfile)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("Package write: {e}")))?;

    Ok(())
}

#[pymodule]
fn passes_rs_py(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyPassConfig>()?;
    m.add_function(wrap_pyfunction!(generate_pass, m)?)?;
    Ok(())
}

// Removed unused imports: Barcode, BarcodeFormat
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
    logo_path = None,
    logo2x_path = None,
    thumbnail_path = None,
    thumbnail2x_path = None,
    strip_path = None,
    strip2x_path = None,
    background_path = None,
    background2x_path = None,
    footer_path = None,
    footer2x_path = None,
))]
fn generate_pass(
    config: &str,
    cert_path: &str,
    key_path: &str,
    output_path: &str,
    icon_path: Option<&str>,
    icon2x_path: Option<&str>,
    logo_path: Option<&str>,
    logo2x_path: Option<&str>,
    thumbnail_path: Option<&str>,
    thumbnail2x_path: Option<&str>,
    strip_path: Option<&str>,
    strip2x_path: Option<&str>,
    background_path: Option<&str>,
    background2x_path: Option<&str>,
    footer_path: Option<&str>,
    footer2x_path: Option<&str>,
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

    /* ---------- logos ----------- */
    if let Some(p) = logo_path {
        let f = File::open(p)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;
        package
            .add_resource(resource::Type::Logo(resource::Version::Standard), f)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
    }
    if let Some(p) = logo2x_path {
        let f = File::open(p)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;
        package
            .add_resource(resource::Type::Logo(resource::Version::Size2X), f) // @2x
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
    }

    /* -------- thumbnails -------- */
    if let Some(p) = thumbnail_path {
        let f = File::open(p)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;
        package
            .add_resource(resource::Type::Thumbnail(resource::Version::Standard), f)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
    }
    if let Some(p) = thumbnail2x_path {
        let f = File::open(p)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;
        package
            .add_resource(resource::Type::Thumbnail(resource::Version::Size2X), f) // @2x
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
    }

    /* ---------- strips ---------- */
    if let Some(p) = strip_path {
        let f = File::open(p)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;
        package
            .add_resource(resource::Type::Strip(resource::Version::Standard), f)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
    }
    if let Some(p) = strip2x_path {
        let f = File::open(p)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;
        package
            .add_resource(resource::Type::Strip(resource::Version::Size2X), f) // @2x
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
    }

    /* ------- backgrounds -------- */
    if let Some(p) = background_path {
        let f = File::open(p)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;
        package
            .add_resource(resource::Type::Background(resource::Version::Standard), f)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
    }
    if let Some(p) = background2x_path {
        let f = File::open(p)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;
        package
            .add_resource(resource::Type::Background(resource::Version::Size2X), f) // @2x
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
    }

    /* --------- footers --------- */
    if let Some(p) = footer_path {
        let f = File::open(p)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;
        package
            .add_resource(resource::Type::Footer(resource::Version::Standard), f)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
    }
    if let Some(p) = footer2x_path {
        let f = File::open(p)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;
        package
            .add_resource(resource::Type::Footer(resource::Version::Size2X), f) // @2x
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

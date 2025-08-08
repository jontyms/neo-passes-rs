use rsa::{RsaPrivateKey, pkcs8::DecodePrivateKey};
use x509_cert::{
    Certificate,
    der::{Decode, DecodePem},
};

use crate::error::PassError;

/// Configuration for package signing.
///
/// Contains WWDR (Apple Worldwide Developer Relations), Signer Certificate (Developer), Signer Certificate Key (Developer)
/// certificate for pass signing with private key
#[derive(Debug, Clone)]
pub struct SignConfig {
    pub sign_key: RsaPrivateKey,
    pub cert: Certificate,
    pub sign_cert: Certificate,
}

impl SignConfig {
    /// Create new config from buffers
    /// # Errors
    /// Returns `PassError` when the certs and keys cannot be loaded
    pub fn new(wwdr: &WWDR, sign_cert: &[u8], sign_key: &str) -> Result<SignConfig, PassError> {
        let cert = match wwdr {
            WWDR::G4 => Certificate::from_der(G4_CERT)?,
            WWDR::Custom(buf) => Certificate::from_pem(buf)?,
        };
        let sign_cert = Certificate::from_pem(sign_cert)?;
        let sign_key = RsaPrivateKey::from_pkcs8_pem(sign_key)?;

        Ok(SignConfig {
            sign_key,
            cert,
            sign_cert,
        })
    }
}

/// G4 certificate from <https://www.apple.com/certificateauthority/>
const G4_CERT: &[u8; 1113] = include_bytes!("AppleWWDRCAG4.cer");

/// Predefined certificate from Apple CA, or custom certificate
pub enum WWDR<'a> {
    G4,
    Custom(&'a [u8]),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_config() {
        // Generate a self-signed certificate and PKCS#8 private key using rcgen (dev-dependency)
        let rc_cert = rcgen::generate_simple_self_signed(vec!["example.com".into()]).unwrap();
        let cert_pem = rc_cert.serialize_pem().unwrap();
        let key_pem = rc_cert.serialize_private_key_pem();

        let _ = SignConfig::new(&WWDR::G4, cert_pem.as_bytes(), &key_pem).unwrap();
    }
}

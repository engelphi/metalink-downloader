use serde::Deserialize;
use serde_with::{serde_as, DisplayFromStr};

/// Representation of the metalink:signature field according to
/// [RFC5854 Section 4.2.13](https://www.rfc-editor.org/rfc/rfc5854#section-4.2.13)
#[serde_as]
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Signature {
    // TODO add validation: needs to be MIME type describing the signature type
    #[serde(rename = "@mediatype")]
    #[serde_as(as = "DisplayFromStr")]
    media_type: mime::Mime,
    #[serde(rename = "$text")]
    signature: String,
}

impl Signature {
    /// Create a new Signature with the given media type and signature
    pub fn new(media_type: mime::Mime, signature: &str) -> Self {
        Self {
            media_type,
            signature: String::from(signature),
        }
    }

    /// Returns the mediatype of the signature
    pub fn media_type(&self) -> &mime::Mime {
        &self.media_type
    }

    /// Returns the actual signature data
    pub fn signature(&self) -> &String {
        &self.signature
    }
}

impl std::str::FromStr for Signature {
    type Err = crate::MetalinkError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(crate::utils::from_str::<Signature>(s)?)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    #[test]
    fn read_signature() {
        const SIGNATURE: &str = r#"
            <signature mediatype="application/pgp-signature">
               -----BEGIN PGP SIGNATURE-----
               Version: GnuPG v1.4.10 (GNU/Linux)

               iEYEABECAAYFAkrxdXQACgkQeOEcayedXJHqFwCfd1p/HhRf/iDvYhvFbTrQPz+p
               p3oAoO9lKHoOqOE0EMB3zmMcLoYUrNkg
               =ggAf
               -----END PGP SIGNATURE-----
            </signature>
        "#;

        let signature: Signature = SIGNATURE.parse().unwrap();
        assert_eq!(
            mime::Mime::from_str("application/pgp-signature").unwrap(),
            *signature.media_type()
        );
        assert_eq!(
            String::from(
                "-----BEGIN PGP SIGNATURE-----
               Version: GnuPG v1.4.10 (GNU/Linux)

               iEYEABECAAYFAkrxdXQACgkQeOEcayedXJHqFwCfd1p/HhRf/iDvYhvFbTrQPz+p
               p3oAoO9lKHoOqOE0EMB3zmMcLoYUrNkg
               =ggAf
               -----END PGP SIGNATURE-----"
            ),
            *signature.signature()
        );
    }
}

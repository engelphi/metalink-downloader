use crate::{
    Copyright, Description, FileUrl, Hash, Identity, Language, Logo, MetaUrl, MetalinkError,
    Pieces, Publisher, Signature, Size, Version, OS,
};
use serde::Deserialize;

/// Representation of the metalink:file element according to
/// [RFC5854 Section 4.1.2](https://www.rfc-editor.org/rfc/rfc5854#section-4.1.2)
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct File {
    #[serde(rename = "@name")]
    name: String,
    copyright: Option<Copyright>,
    description: Option<Description>,
    hash: Option<Vec<Hash>>,
    identity: Option<Identity>,
    language: Option<Vec<Language>>,
    logo: Option<Logo>,
    metaurl: Option<Vec<MetaUrl>>,
    os: Option<Vec<OS>>,
    pieces: Option<Pieces>,
    publisher: Option<Publisher>,
    signature: Option<Signature>,
    size: Option<Size>,
    url: Option<Vec<FileUrl>>,
    version: Option<Version>,
}

impl File {
    /// Returns the name of the file element
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Returns the copyright of the file element if set
    pub fn copyright(&self) -> Option<&Copyright> {
        self.copyright.as_ref()
    }

    /// Returns the description of the file element if set
    pub fn description(&self) -> Option<&Description> {
        self.description.as_ref()
    }

    /// Returns the hashes of the file element if set
    pub fn hashes(&self) -> Option<&Vec<Hash>> {
        self.hash.as_ref()
    }

    /// Returns the identity of the file element if set
    pub fn identity(&self) -> Option<&Identity> {
        self.identity.as_ref()
    }

    /// Returns the languages of the file element if set
    pub fn languages(&self) -> Option<&Vec<Language>> {
        self.language.as_ref()
    }

    /// Returns the logo of the file element if set
    pub fn logo(&self) -> Option<&Logo> {
        self.logo.as_ref()
    }

    /// Returns the metaurls of the file element if set
    pub fn meta_urls(&self) -> Option<&Vec<MetaUrl>> {
        self.metaurl.as_ref()
    }

    /// Returns the oses of the file element if set
    pub fn oses(&self) -> Option<&Vec<OS>> {
        self.os.as_ref()
    }

    /// Returns the pieces of the file element if set
    pub fn pieces(&self) -> Option<&Pieces> {
        self.pieces.as_ref()
    }

    /// Returns the Publisher of the file element if set
    pub fn publisher(&self) -> Option<&Publisher> {
        self.publisher.as_ref()
    }

    /// Returns the signature of stored in the file element if set
    pub fn signature(&self) -> Option<&Signature> {
        self.signature.as_ref()
    }

    /// Returns the size of the file referenced by the file element if set
    pub fn size(&self) -> Option<&Size> {
        self.size.as_ref()
    }

    /// Returns the urls of the file element if set
    pub fn urls(&self) -> Option<&Vec<FileUrl>> {
        self.url.as_ref()
    }

    /// Returns the version of the file element if set
    pub fn version(&self) -> Option<&Version> {
        self.version.as_ref()
    }
}

impl std::str::FromStr for File {
    type Err = crate::MetalinkError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(crate::utils::from_str::<File>(s)?)
    }
}

/// Helper type for constructing File elements
#[derive(Debug, Default)]
pub struct FileBuilder {
    name: Option<String>,
    copyright: Option<Copyright>,
    description: Option<Description>,
    hash: Option<Vec<Hash>>,
    identity: Option<Identity>,
    language: Option<Vec<Language>>,
    logo: Option<Logo>,
    metaurl: Option<Vec<MetaUrl>>,
    os: Option<Vec<OS>>,
    pieces: Option<Pieces>,
    publisher: Option<Publisher>,
    signature: Option<Signature>,
    size: Option<Size>,
    url: Option<Vec<FileUrl>>,
    version: Option<Version>,
}

impl FileBuilder {
    /// Create a new FileBuilder
    pub fn new() -> Self {
        Self {
            name: None,
            copyright: None,
            description: None,
            hash: None,
            identity: None,
            language: None,
            logo: None,
            metaurl: None,
            os: None,
            pieces: None,
            publisher: None,
            signature: None,
            size: None,
            url: None,
            version: None,
        }
    }

    /// Construct a File object based on stored parameters.
    pub fn build(self) -> Result<File, MetalinkError> {
        if self.name.is_none() {
            return Err(MetalinkError::MetalinkConstructionError(
                "File elements require a name, set it with with_name(...)".to_owned(),
            ));
        }

        match (&self.url, &self.metaurl) {
            (None, None) => {
                return Err(MetalinkError::MetalinkConstructionError(
                    "File elements require at least one url or one metaurl element".to_owned(),
                ))
            }
            (Some(urls), None) if urls.is_empty() => {
                return Err(MetalinkError::MetalinkConstructionError(
                    "File elements require at least one url or one metaurl element".to_owned(),
                ))
            }
            (None, Some(metaurls)) if metaurls.is_empty() => {
                return Err(MetalinkError::MetalinkConstructionError(
                    "File elements require at least one url or one metaurl element".to_owned(),
                ))
            }
            (Some(urls), Some(metaurls)) if urls.is_empty() && metaurls.is_empty() => {
                return Err(MetalinkError::MetalinkConstructionError(
                    "File elements require at least one url or one metaurl element".to_owned(),
                ))
            }
            _ => {}
        }

        Ok(File {
            name: self.name.unwrap(),
            copyright: self.copyright,
            description: self.description,
            hash: self.hash,
            identity: self.identity,
            language: self.language,
            logo: self.logo,
            metaurl: self.metaurl,
            os: self.os,
            pieces: self.pieces,
            publisher: self.publisher,
            signature: self.signature,
            size: self.size,
            url: self.url,
            version: self.version,
        })
    }

    /// Set the name of the file element
    pub fn with_name(mut self, name: &str) -> Self {
        self.name = Some(String::from(name));
        self
    }

    /// Set the copyright of the file element
    pub fn with_copyright(mut self, copyright: Copyright) -> Self {
        self.copyright = Some(copyright);
        self
    }

    /// Set the description of the file element
    pub fn with_description(mut self, description: Description) -> Self {
        self.description = Some(description);
        self
    }

    /// Set the hashes of the file element
    pub fn with_hashes(mut self, hashes: Vec<Hash>) -> Self {
        self.hash = Some(hashes);
        self
    }

    /// Set the identity of the file element
    pub fn with_identity(mut self, identity: Identity) -> Self {
        self.identity = Some(identity);
        self
    }

    /// Sets the languages of the file element
    pub fn with_languages(mut self, languages: Vec<Language>) -> Self {
        self.language = Some(languages);
        self
    }

    /// Set the logo of the file element
    pub fn with_logo(mut self, logo: Logo) -> Self {
        self.logo = Some(logo);
        self
    }

    /// Set the metaurls of the file element
    pub fn with_metaurls(mut self, metaurls: Vec<MetaUrl>) -> Self {
        self.metaurl = Some(metaurls);
        self
    }

    /// Set the oses of the file element
    pub fn with_oses(mut self, oses: Vec<OS>) -> Self {
        self.os = Some(oses);
        self
    }

    /// Sets the pieces of the file element
    pub fn with_pieces(mut self, pieces: Pieces) -> Self {
        self.pieces = Some(pieces);
        self
    }

    /// Sets the publisher of the file element
    pub fn with_publisher(mut self, publisher: Publisher) -> Self {
        self.publisher = Some(publisher);
        self
    }

    /// Sets the signature of the file element
    pub fn with_signature(mut self, signature: Signature) -> Self {
        self.signature = Some(signature);
        self
    }

    /// Sets the size of the file element
    pub fn with_size(mut self, size: Size) -> Self {
        self.size = Some(size);
        self
    }

    /// Sets the urls of the file element
    pub fn with_urls(mut self, urls: Vec<FileUrl>) -> Self {
        self.url = Some(urls);
        self
    }

    /// Sets the version of the file element
    pub fn with_version(mut self, version: Version) -> Self {
        self.version = Some(version);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TorrentOrMime;
    use iana_registry_enums::{HashFunctionTextualName, OperatingSystemName};

    use std::str::FromStr;

    #[test]
    fn read_file() {
        const FILE: &str = r#"
            <file name="abc/def">
                <hash type="sha-1">abc</hash>
                <hash type="sha-256">def</hash>
                <hash type="sha-512">ghi</hash>
                <description>Description</description>
                <copyright>Copyright</copyright>
                <identity>Test</identity>
                <language>German</language>
                <language>English</language>
                <logo>https://www.google.com/images/branding/googlelogo/1x/googlelogo_light_color_272x92dp.png</logo>
                <metaurl priority="1" mediatype="torrent" name="test/test2/test.tar.gz">https://www.rfc-editor.org/rfc/rfc5854</metaurl>
                <os>MACOS</os>
                <os>LINUX</os>
                <pieces type="sha-1" length="50">
                    <hash>abc</hash>
                    <hash>def</hash>
                </pieces>
                <publisher name="Company Inc." url="https://www.google.com" />
                <signature mediatype="application/pgp-signature">
                   -----BEGIN PGP SIGNATURE-----
                   Version: GnuPG v1.4.10 (GNU/Linux)

                   iEYEABECAAYFAkrxdXQACgkQeOEcayedXJHqFwCfd1p/HhRf/iDvYhvFbTrQPz+p
                   p3oAoO9lKHoOqOE0EMB3zmMcLoYUrNkg
                   =ggAf
                   -----END PGP SIGNATURE-----
                </signature>
                <size>
                    50
                </size>
                <url priority="1" location="de">https://www.google.de</url>
                <url priority="1" location="us">https://www.google.com</url>
                <version>1.0.0</version>
            </file>
        "#;

        let file: File = FILE.parse().unwrap();
        assert_eq!(*file.name(), String::from("abc/def"));
        assert_eq!(
            *file.hashes().unwrap(),
            vec![
                Hash::new(Some(HashFunctionTextualName::Sha1), "abc"),
                Hash::new(Some(HashFunctionTextualName::Sha256), "def"),
                Hash::new(Some(HashFunctionTextualName::Sha512), "ghi")
            ]
        );
        assert_eq!(
            *file.description().unwrap(),
            Description::new("Description")
        );
        assert_eq!(*file.copyright().unwrap(), Copyright::new("Copyright"));
        assert_eq!(*file.identity().unwrap(), Identity::new("Test"));
        assert_eq!(
            *file.languages().unwrap(),
            vec![Language::new("German"), Language::new("English")]
        );
        assert_eq!(*file.logo().unwrap(), Logo::new(url::Url::parse("https://www.google.com/images/branding/googlelogo/1x/googlelogo_light_color_272x92dp.png").unwrap()));
        assert_eq!(
            *file.meta_urls().unwrap(),
            vec![MetaUrl::new(
                url::Url::parse("https://www.rfc-editor.org/rfc/rfc5854").unwrap(),
                TorrentOrMime::Torrent,
                Some(1),
                Some(String::from("test/test2/test.tar.gz"))
            )]
        );
        assert_eq!(
            *file.oses().unwrap(),
            vec![
                OS::new(OperatingSystemName::MacOS),
                OS::new(OperatingSystemName::Linux),
            ]
        );
        assert_eq!(
            *file.pieces().unwrap(),
            Pieces::new(
                HashFunctionTextualName::Sha1,
                50,
                vec![Hash::new(None, "abc"), Hash::new(None, "def")]
            ),
        );
        assert_eq!(
            *file.publisher().unwrap(),
            Publisher::new_with_url(
                "Company Inc.",
                url::Url::parse("https://www.google.com").unwrap()
            )
        );
        assert_eq!(
            *file.signature().unwrap(),
            Signature::new(
                mime::Mime::from_str("application/pgp-signature").unwrap(),
                "-----BEGIN PGP SIGNATURE-----
                   Version: GnuPG v1.4.10 (GNU/Linux)

                   iEYEABECAAYFAkrxdXQACgkQeOEcayedXJHqFwCfd1p/HhRf/iDvYhvFbTrQPz+p
                   p3oAoO9lKHoOqOE0EMB3zmMcLoYUrNkg
                   =ggAf
                   -----END PGP SIGNATURE-----"
            )
        );
        assert_eq!(*file.size().unwrap(), Size::new(50));
        assert_eq!(
            *file.urls().unwrap(),
            vec![
                FileUrl::new(
                    url::Url::parse("https://www.google.de").unwrap(),
                    Some(1),
                    Some(isocountry::CountryCode::DEU)
                ),
                FileUrl::new(
                    url::Url::parse("https://www.google.com").unwrap(),
                    Some(1),
                    Some(isocountry::CountryCode::USA)
                )
            ]
        );
        assert_eq!(*file.version().unwrap(), Version::new("1.0.0"));

        let expected_file = FileBuilder::new()
            .with_name("abc/def")
            .with_hashes(vec![
                Hash::new(Some(HashFunctionTextualName::Sha1), "abc"),
                Hash::new(Some(HashFunctionTextualName::Sha256), "def"),
                Hash::new(Some(HashFunctionTextualName::Sha512), "ghi"),
            ])
            .with_description(Description::new("Description"))
            .with_copyright(Copyright::new("Copyright"))
            .with_identity(Identity::new("Test"))
            .with_languages(vec![Language::new("German"), Language::new("English")])
            .with_logo(Logo::new(url::Url::parse("https://www.google.com/images/branding/googlelogo/1x/googlelogo_light_color_272x92dp.png").unwrap()))
            .with_metaurls(vec![MetaUrl::new(
                url::Url::parse("https://www.rfc-editor.org/rfc/rfc5854").unwrap(),
                TorrentOrMime::Torrent,
                Some(1),
                Some(String::from("test/test2/test.tar.gz"))
            )])
            .with_oses(vec![
                OS::new(OperatingSystemName::MacOS),
                OS::new(OperatingSystemName::Linux),
            ])
            .with_pieces(Pieces::new(
                HashFunctionTextualName::Sha1,
                50,
                vec![Hash::new(None, "abc"), Hash::new(None, "def")]
            ))
            .with_publisher(Publisher::new_with_url(
                "Company Inc.",
                url::Url::parse("https://www.google.com").unwrap()
            ))
            .with_signature(Signature::new(
                mime::Mime::from_str("application/pgp-signature").unwrap(),
                "-----BEGIN PGP SIGNATURE-----
                   Version: GnuPG v1.4.10 (GNU/Linux)

                   iEYEABECAAYFAkrxdXQACgkQeOEcayedXJHqFwCfd1p/HhRf/iDvYhvFbTrQPz+p
                   p3oAoO9lKHoOqOE0EMB3zmMcLoYUrNkg
                   =ggAf
                   -----END PGP SIGNATURE-----")
            )
            .with_urls(vec![
                FileUrl::new(
                    url::Url::parse("https://www.google.de").unwrap(),
                    Some(1),
                    Some(isocountry::CountryCode::DEU)
                ),
                FileUrl::new(
                    url::Url::parse("https://www.google.com").unwrap(),
                    Some(1),
                    Some(isocountry::CountryCode::USA)
                )
            ])
            .with_size(Size::new(50))
            .with_version(Version::new("1.0.0"))
            .build()
            .unwrap();

        assert_eq!(file, expected_file);
    }

    #[test]
    fn file_builder_error_for_file_without_name() {
        assert!(FileBuilder::new().build().is_err());
    }

    #[test]
    fn file_builder_error_for_file_without_urls_and_metaurls() {
        assert!(FileBuilder::new().with_name("test").build().is_err());
        assert!(FileBuilder::new().with_name("test").build().is_err());
        assert!(FileBuilder::new()
            .with_name("test")
            .with_urls(vec![])
            .build()
            .is_err());
        assert!(FileBuilder::new()
            .with_name("test")
            .with_metaurls(vec![])
            .build()
            .is_err());
        assert!(FileBuilder::new()
            .with_name("test")
            .with_urls(vec![])
            .with_metaurls(vec![])
            .build()
            .is_err());
    }
}

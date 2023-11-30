// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use anyhow::Error;
use asn1_rs::oid;
use std::io::{BufReader, Cursor};
use std::path::Path;
use x509_parser::certificate::X509Certificate;
use x509_parser::extensions::{GeneralName, ParsedExtension};
use x509_parser::pem::Pem;

pub(crate) struct RawCosignBundle {
    pem: Pem,
}

impl RawCosignBundle {
    pub(crate) fn load(path: &Path) -> Result<Self, Error> {
        let contents = std::fs::read(path)?;
        let bundle: OnDiskBundle = serde_json::from_slice(&contents)?;

        let raw_pem = BufReader::new(Cursor::new(base64::decode(&bundle.cert)?));
        let pem = Pem::read(raw_pem)?.0;
        Ok(Self { pem })
    }

    pub(crate) fn parse(&self) -> Result<CosignBundle<'_>, Error> {
        Ok(CosignBundle { certificate: self.pem.parse_x509()? })
    }
}

pub(crate) struct CosignBundle<'raw> {
    certificate: X509Certificate<'raw>,
}

impl CosignBundle<'_> {
    pub(crate) fn email(&self) -> Result<String, Error> {
        let ext = self
            .certificate
            .get_extension_unique(&oid!(2.5.29.17))?
            .ok_or_else(|| anyhow::anyhow!("missing subject alternative name"))?
            .parsed_extension();

        let ParsedExtension::SubjectAlternativeName(san) = ext else {
            anyhow::bail!("invalid extension: {ext:?}");
        };
        let [GeneralName::RFC822Name(email)] = san.general_names[..] else {
            anyhow::bail!("invalid subject alternative name content: {san:?}");
        };
        Ok(email.to_string())
    }

    pub(crate) fn idp(&self) -> Result<String, Error> {
        let ext = self
            .certificate
            .get_extension_unique(&oid!(1.3.6.1.4.1.57264.1.1))?
            .ok_or_else(|| anyhow::anyhow!("missing fulcio idp"))?;
        Ok(std::str::from_utf8(&ext.value)?.to_string())
    }
}

#[derive(serde::Deserialize)]
pub(crate) struct OnDiskBundle {
    cert: String,
}

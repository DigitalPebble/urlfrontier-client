//! Connection to the Frontier, in plaintext or over TLS

use std::error::Error;
use std::fs;
use std::net::IpAddr;
use std::path::{Path, PathBuf};

use clap::Args;
use tonic::transport::{Certificate, ClientTlsConfig, Endpoint, Identity};

use crate::Client;

// options securing the connection with TLS; a doc comment here would replace the about of uf
#[derive(Args, Debug)]
pub struct TlsArgs {
    /// Connects to the Frontier over TLS. Implied by the other --tls-* options
    #[arg(long)]
    tls: bool,

    /// PEM file with the certificates trusted to sign the server certificate, in addition
    /// to the ones of the system
    #[arg(long, value_name = "PEM")]
    tls_trust_cert: Option<PathBuf>,

    /// PEM file with the client certificate chain sent for mutual TLS, with --tls-key
    #[arg(long, value_name = "PEM", requires = "tls_key")]
    tls_cert: Option<PathBuf>,

    /// PEM file with the unencrypted private key of the client certificate, PKCS#8 or
    /// PKCS#1 (RSA), with --tls-cert
    #[arg(long, value_name = "PEM", requires = "tls_cert")]
    tls_key: Option<PathBuf>,

    /// Name checked against the server certificate, if it is not the host connected to,
    /// for instance when going through a tunnel
    #[arg(long, value_name = "NAME")]
    tls_server_name: Option<String>,
}

impl TlsArgs {
    fn enabled(&self) -> bool {
        self.tls
            || self.tls_trust_cert.is_some()
            || self.tls_cert.is_some()
            || self.tls_key.is_some()
            || self.tls_server_name.is_some()
    }

    fn config(&self, host: &str) -> Result<ClientTlsConfig, String> {
        let name = self.tls_server_name.as_deref().unwrap_or(host);
        // an IPv6 address is given in brackets in the URI but not in the certificate
        let name = name.trim_start_matches('[').trim_end_matches(']');
        // the TLS library checks host names only, not the IP addresses of a certificate
        if name.parse::<IpAddr>().is_ok() {
            return Err(format!(
                "TLS needs the host name of the Frontier, not its IP address {}: \
                 connect to the host name or give it with --tls-server-name",
                name
            ));
        }
        let mut config = ClientTlsConfig::new().domain_name(name);
        if let Some(ca) = &self.tls_trust_cert {
            config =
                config.ca_certificate(Certificate::from_pem(read_certs(ca, "--tls-trust-cert")?));
        }
        if let (Some(cert), Some(key)) = (&self.tls_cert, &self.tls_key) {
            config = config.identity(Identity::from_pem(
                read_certs(cert, "--tls-cert")?,
                read(key, "--tls-key")?,
            ));
        }
        Ok(config)
    }
}

fn read(path: &Path, option: &str) -> Result<Vec<u8>, String> {
    fs::read(path).map_err(|e| format!("Cannot read {} {}: {}", option, path.display(), e))
}

/// Reads a PEM file which must hold certificates; the TLS library would otherwise
/// ignore a file without any and only fail once connected
fn read_certs(path: &Path, option: &str) -> Result<Vec<u8>, String> {
    let pem = read(path, option)?;
    let marker = b"-----BEGIN CERTIFICATE-----";
    if !pem.windows(marker.len()).any(|w| w == marker) {
        return Err(format!(
            "No PEM certificate in {} {}",
            option,
            path.display()
        ));
    }
    Ok(pem)
}

/// Formats an error with its causes, which the TLS errors keep their details in
fn with_causes(e: &dyn Error) -> String {
    let mut message = e.to_string();
    let mut source = e.source();
    while let Some(cause) = source {
        message.push_str(": ");
        message.push_str(&cause.to_string());
        source = cause.source();
    }
    message
}

/// Returns the endpoint of the Frontier at `host:port`, which every connection is made to
pub fn endpoint(host: &str, port: usize, tls: &TlsArgs) -> Result<Endpoint, String> {
    let scheme = if tls.enabled() { "https" } else { "http" };
    let uri = format!("{}://{}:{}", scheme, host, port);
    let mut endpoint = Endpoint::from_shared(uri.clone())
        .map_err(|e| format!("Invalid address {}: {}", uri, e))?;
    if tls.enabled() {
        endpoint = endpoint
            .tls_config(tls.config(host)?)
            .map_err(|e| format!("Invalid TLS configuration: {}", with_causes(&e)))?;
    }
    Ok(endpoint)
}

/// Opens a connection to the Frontier
pub async fn connect(endpoint: &Endpoint) -> Result<Client, tonic::transport::Error> {
    Ok(Client::new(endpoint.connect().await?))
}

pub mod error;
pub use error::{Result, StarbloxError};

use rustls::{ClientConfig, RootCertStore, pki_types::ServerName};
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio_rustls::TlsConnector;
use webpki_roots::TLS_SERVER_ROOTS;

#[derive(Clone)]
pub struct Account {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}

pub async fn test_imap_rustls(account: &Account) -> Result<()> {
    let roots = RootCertStore::from_iter(TLS_SERVER_ROOTS.iter().cloned());

    let config = ClientConfig::builder()
        .with_root_certificates(roots)
        .with_no_client_auth();
    let connector = TlsConnector::from(Arc::new(config));
    let server_name = ServerName::try_from(account.host.clone()).expect("invalid DNS name");

    let tcp = TcpStream::connect((account.host.as_str(), account.port)).await?;
    let tls = connector.connect(server_name, tcp).await.unwrap();

    let client = async_imap::Client::new(tls);
    let mut session = client
        .login(&account.username, &account.password)
        .await
        .map_err(|(e, _)| e)?;

    session.select("INBOX").await?;
    session.logout().await?;
    Ok(())
}

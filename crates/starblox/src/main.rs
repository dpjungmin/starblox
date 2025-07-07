use anyhow::Context;
use starblox_core::{Account, test_imap_rustls};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let account = Account {
        host: std::env::var("SBX_IMAP_HOST").context("missing SBX_IMAP_HOST")?,
        port: std::env::var("SBX_IMAP_PORT")?.parse()?,
        username: std::env::var("SBX_IMAP_USER")?,
        password: std::env::var("SBX_IMAP_PASS")?,
    };

    test_imap_rustls(&account)
        .await
        .context("IMAP login failed")?;
    println!("IMAP login success!");

    Ok(())
}

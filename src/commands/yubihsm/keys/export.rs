//! Create encrypted backups of YubiHSM2 keys

use super::*;
use abscissa_core::{Command, Runnable};
use std::{fs::OpenOptions, io::Write, os::unix::fs::OpenOptionsExt, path::PathBuf, process};
use subtle_encoding::base64;

/// The `yubihsm keys export` subcommand: create encrypted backups of keys
#[derive(Command, Debug, Default, Options)]
pub struct ExportCommand {
    /// Path to configuration file
    #[options(short = "c", long = "config", help = "path to tmkms.toml")]
    pub config: Option<PathBuf>,

    /// ID of the key to export
    #[options(short = "i", long = "id", help = "key to export in encrypted form")]
    pub key_id: u16,

    /// ID of the wrap key to encrypt the exported key under
    #[options(
        short = "w",
        long = "wrapkey",
        help = "wrap key to encrypt exported key"
    )]
    pub wrap_key_id: Option<u16>,

    /// Path to write the resulting file to
    #[options(free, help = "path where ciphertext of exported key will be written")]
    pub path: PathBuf,
}

impl Runnable for ExportCommand {
    fn run(&self) {
        let wrap_key_id = self.wrap_key_id.unwrap_or(DEFAULT_WRAP_KEY);

        let wrapped_bytes = crate::yubihsm::client()
            .export_wrapped(
                wrap_key_id,
                yubihsm::object::Type::AsymmetricKey,
                self.key_id,
            )
            .unwrap_or_else(|e| {
                status_err!(
                    "couldn't export key {} under wrap key {}: {}",
                    self.key_id,
                    wrap_key_id,
                    e
                );
                process::exit(1);
            });

        let mut export_file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .mode(0o600)
            .open(&self.path)
            .unwrap_or_else(|e| {
                status_err!("couldn't export to {} ({})", &self.path.display(), e);
                process::exit(1);
            });

        export_file
            .write_all(&base64::encode(&wrapped_bytes.into_vec()))
            .unwrap_or_else(|e| {
                status_err!("error exporting {}: {}", &self.path.display(), e);
                process::exit(1);
            });

        status_ok!(
            "Exported",
            "key 0x{:04x} (encrypted under wrap key 0x{:04x}) to {}",
            self.key_id,
            wrap_key_id,
            self.path.display()
        );
    }
}

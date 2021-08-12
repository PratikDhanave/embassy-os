use std::fs::File;
use std::io::Write;
use std::path::Path;

use ed25519_dalek::Keypair;
use rpc_toolkit::command;

use crate::context::EitherContext;
use crate::util::display_none;
use crate::{Error, ResultExt};

#[command(cli_only, blocking, display(display_none))]
pub fn init(#[context] ctx: EitherContext) -> Result<(), Error> {
    let ctx = ctx.as_cli().unwrap();
    if !ctx.developer_key_path.exists() {
        let parent = ctx.developer_key_path.parent().unwrap_or(Path::new("/"));
        if !parent.exists() {
            std::fs::create_dir_all(parent)
                .with_ctx(|_| (crate::ErrorKind::Filesystem, parent.display().to_string()))?;
        }
        log::info!("Generating new developer key...");
        let keypair = Keypair::generate(&mut rand::thread_rng());
        log::info!("Writing key to {}", ctx.developer_key_path.display());
        let mut dev_key_file = File::create(&ctx.developer_key_path)?;
        dev_key_file.write_all(&keypair.to_bytes())?;
        dev_key_file.sync_all()?;
    }
    Ok(())
}
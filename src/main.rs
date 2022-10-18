
mod db;
mod runner;
use anyhow::{Context, Result};
use std::env;

fn main() -> Result<()> {
    let conn = db::open(
        &env::var("GOGODB_PATH").with_context(|| format!("{} not set!", "GOGO_DB_PATH"))?,
    )?;
    runner::run(&conn)?;
    Ok(())
}

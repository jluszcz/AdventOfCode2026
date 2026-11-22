use anyhow::Result;
use log::debug;

fn main() -> Result<()> {
    let input = aoc_util::init()?;
    debug!("Read {} lines", input.len());

    Ok(())
}

// run with: `RUST_BACKTRACE=1 cargo run --example bubble`

use anyhow::{ensure, Result};

fn main() -> Result<()> {
    aa()?;
    Ok(())
}

fn aa() -> Result<()> {
    bb(33)?;
    bb(77)?;
    bb(5)?;
    Ok(())
}

fn bb(p: i32) -> Result<i32> {
    ensure!(p >= 10, "param not big enough!");
    Ok(p)
}

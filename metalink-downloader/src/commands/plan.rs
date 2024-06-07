use crate::types::Plan;
use crate::Result;

use log::info;
use std::path::PathBuf;

pub async fn plan(metalink_file: PathBuf, target_dir: PathBuf) -> Result<()> {
    info!("File: {:?}, Target: {:?}", metalink_file, target_dir);
    let plan = Plan::new(metalink_file, target_dir)?;
    println!("{:#?}", plan);

    let minimized_plan = plan.minimize_plan()?;
    println!("{:#?}", minimized_plan);
    Ok(())
}

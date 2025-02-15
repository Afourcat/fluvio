use semver::Version;
use structopt::StructOpt;

use crate::progress::ProgressBarFactory;
use crate::{ClusterChecker};
use crate::check::{SysChartCheck, ClusterCheckError};
use crate::charts::ChartConfig;

#[derive(Debug, StructOpt)]
pub struct CheckOpt {}

impl CheckOpt {
    pub async fn process(self, platform_version: Version) -> Result<(), ClusterCheckError> {
        use colored::*;
        println!("{}", "Running pre-startup checks...".bold());
        let sys_config: ChartConfig = ChartConfig::sys_builder()
            .build()
            .map_err(|err| ClusterCheckError::Other(format!("chart config error: {:#?}", err)))?;

        let pb = ProgressBarFactory::new(false);
        ClusterChecker::empty()
            .with_preflight_checks()
            .with_check(SysChartCheck::new(sys_config, platform_version))
            .run(&pb, false)
            .await?;

        Ok(())
    }
}

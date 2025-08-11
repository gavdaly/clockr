mod daily;
mod hourly;
mod monthly;
mod weekly;

use tokio_cron_scheduler::{JobScheduler, JobSchedulerError};
use tracing::info;

pub async fn jobs() -> Result<(), JobSchedulerError> {
    let mut sched = JobScheduler::new().await?;

    // Add code to be run during/after shutdown
    sched.set_shutdown_handler(Box::new(|| {
        Box::pin(async move {
            info!("Shut down jobs");
        })
    }));

    sched.add(daily::create()?).await?;
    sched.add(hourly::create()?).await?;
    sched.add(weekly::create()?).await?;
    sched.add(monthly::create()?).await?;

    sched.start().await?;

    Ok(())
}


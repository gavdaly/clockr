use tokio_cron_scheduler::{Job, JobBuilder, JobSchedulerError};
use tracing::{info, instrument};

/// Create a new job that runs every day at 4am
#[instrument]
pub fn create() -> Result<Job, JobSchedulerError> {
    JobBuilder::new()
        .with_cron_job_type()
        .with_schedule("0 0 4 * * *")
        .unwrap()
        .with_run_async(Box::new(|_uuid, mut _l| {
            Box::pin(async move {
                info!("I run async day at 4am UTC");
                // Send messages to users that have invalid timesheets
            })
        }))
        .build()
}

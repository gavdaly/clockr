use tokio_cron_scheduler::{Job, JobBuilder, JobSchedulerError};
use tracing::info;

/// Create a new job that runs every hour
pub fn create() -> Result<Job, JobSchedulerError> {
    JobBuilder::new()
        .with_cron_job_type()
        .with_schedule("0 * * * *")
        .unwrap()
        .with_run_async(Box::new(|_uuid, mut _l| {
            Box::pin(async move {
                info!("I run async every hour");
            })
        }))
        .build()
}

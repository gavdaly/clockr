use tokio_cron_scheduler::{Job, JobBuilder, JobSchedulerError};
use tracing::{info, instrument};

/// Create a new job that runs every month at 8:00 AM UTC on the first day of the month
#[instrument]
pub fn create() -> Result<Job, JobSchedulerError> {
    JobBuilder::new()
        .with_cron_job_type()
        .with_schedule("0 8 1 * *")
        .unwrap()
        .with_run_async(Box::new(|_uuid, mut _l| {
            Box::pin(async move {
                info!("I run async every month");
            })
        }))
        .build()
}

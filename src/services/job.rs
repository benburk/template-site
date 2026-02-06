use sqlx::SqlitePool;
use tokio::sync::mpsc;
use tracing::{info, warn};
use uuid::Uuid;

/// Background jobs processed asynchronously.
///
/// Add variants as needed for your application.
#[derive(Debug)]
pub enum Job {
    /// Example job: process something by ID.
    Process(Uuid),
    /// Example job: send an email.
    SendEmail {
        to: String,
        subject: String,
        body: String,
    },
}

/// Runs the job processor, receiving jobs from the channel.
///
/// Spawn this in main.rs:
/// ```ignore
/// let (job_tx, job_rx) = mpsc::unbounded_channel();
/// tokio::spawn(services::job::run(pool.clone(), job_rx));
/// ```
pub async fn run(pool: SqlitePool, mut rx: mpsc::UnboundedReceiver<Job>) {
    info!("Job processor started");

    while let Some(job) = rx.recv().await {
        match job {
            Job::Process(id) => {
                info!(?id, "Processing job");
                // Example: fetch from database and process
                // let item = Item::get_by_id(&pool, id).await;
                let _ = &pool; // Suppress unused warning
            }
            Job::SendEmail { to, subject, body } => {
                info!(?to, ?subject, "Sending email");
                // Example: call email service
                let _ = body; // Suppress unused warning
            }
        }
    }

    warn!("Job processor shutting down - channel closed");
}

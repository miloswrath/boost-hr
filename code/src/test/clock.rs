
use tokio::time::{advance, Duration, sleep};

#[tokio::test(start_paused = true)]
async fn periodic_task_runs() {
    let mut interval = tokio::time::interval(Duration::from_secs(10));
    // tick 1: immediate
    interval.tick().await;
    // fast-forward 10 seconds
    advance(Duration::from_secs(10)).await;
    interval.tick().await;  // fires without real delay
}
``` :contentReference[oaicite:3]{index=3}


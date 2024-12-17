use tokio::time;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::time::Duration;

#[tokio::main]
async fn main() {
    println!("Starting the worker example...");

    // Create a shared cancellation flag using Arc and AtomicBool
    // This allows us to safely share the flag across multiple tasks
    let is_running = Arc::new(AtomicBool::new(true));

    // Spawn the worker in a background task
    let worker_running = Arc::clone(&is_running); // Clone the Arc to share with the worker
    let worker_handle = tokio::spawn(async move {
        // The worker performs its task in a loop, checking the cancellation flag
        worker(worker_running).await;
    });

    // Simulate some main task that does work
    tokio::spawn(async {
        for i in 1..=10 {
            println!("Main task doing work: Iteration {i}");
            time::sleep(Duration::from_secs(1)).await; // Simulate work by sleeping
        }
    });

    // Let the main task run for 5 seconds before stopping the worker
    time::sleep(Duration::from_secs(5)).await;

    // Stop the worker by setting the flag to false
    is_running.store(false, Ordering::SeqCst);
    println!("Requested worker to stop...");

    // Wait for the worker to finish gracefully
    let _ = worker_handle.await;

    println!("Worker has been stopped. Exiting program...");
}

// The worker function that will perform its task until the flag is set to false
async fn worker(is_running: Arc<AtomicBool>) {
    // Run the worker while the `is_running` flag is true
    while is_running.load(Ordering::SeqCst) {
        println!("Worker is performing its task...");
        // Simulate doing some work (e.g., sleep for 2 seconds)
        time::sleep(Duration::from_secs(2)).await;
    }

    // When the flag is false, the worker exits the loop and stops
    println!("Worker has detected stop signal and is exiting...");
}

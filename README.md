# Simple Background Worker in Rust using Tokio

This project demonstrates how to build a simple background worker in Rust using the Tokio async runtime. The worker performs background tasks periodically while the main program continues running other tasks.

## Code Walkthrough

### Main Flow

1. **Shared Cancellation Flag:**
   - We use an `Arc<AtomicBool>` to share a cancellation flag between the main task and the background worker. This ensures safe access across multiple asynchronous tasks.
   - `Arc` allows safe sharing of ownership, and `AtomicBool` ensures atomic updates to the flag.

2. **Starting the Worker:**
   - The worker is spawned using `tokio::spawn`, which creates a new asynchronous task running the `worker` function.
   - Inside the `worker` function, it continuously performs some task (in this case, printing a message and sleeping for 2 seconds) while the cancellation flag is true.

3. **Main Task:**
   - Simultaneously, the main task does some work (in this case, printing messages every second).
   - The main task runs for 5 seconds before stopping the worker.

4. **Stopping the Worker:**
   - After 5 seconds, the main task sets the cancellation flag (`is_running`) to `false` to stop the worker.
   - The worker checks this flag on each iteration and exits the loop once the flag is false.

5. **Graceful Shutdown:**
   - After requesting the worker to stop, the main task waits for the worker to finish using `worker_handle.await`.

### Key Concepts

- **`Arc<AtomicBool>`**: Used to safely share and modify a flag across multiple asynchronous tasks.
- **`tokio::spawn`**: Used to run the worker in the background.
- **Graceful Shutdown**: The worker monitors the `is_running` flag and exits once it is set to `false`.

## Running the Example

1. Clone or download the repository.
2. Add the following dependency to your `Cargo.toml` file:

    ```toml
    [dependencies]
    tokio = { version = "1", features = ["full"] }
    ```

3. Build and run the application:

    ```bash
    cargo run
    ```

## Possible Extensions

- **Real Worker Task**: Instead of just printing, you could have the worker perform tasks like database cleanup, email sending, etc.
- **Error Handling**: Add error handling to ensure the worker can recover from or report issues.
- **Logging**: Use crates like `tracing` for more detailed logging.


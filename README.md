# simple-pool-actix
This is a simple Rust project that demonstrates how to use Actix Web and a thread pool to handle background job requests.
## Project Structure
```
.
├── Cargo.lock
├── Cargo.toml
├── .gitignore
└── src
    └── main.rs
```

## Dependencies

This project uses the following dependencies:

- `actix-web` for the web server
- `actix-rt` for the Actix runtime
- `serde` and `serde_json` for JSON serialization and deserialization
- `threadpool` for managing a pool of worker threads

## Running the Project

To run the project, use the following command:

```sh
cargo run
```

The server will start and listen on `127.0.0.1:8080`.

## API Endpoints

This project exposes the following API endpoints:

- `POST /submit_jobs`: Accepts a JSON payload to submit jobs to the thread pool.

### Example Request

```sh
curl -X POST http://127.0.0.1:8080/submit_jobs -H "Content-Type: application/json" -d '{"job_ids": [1,2,3,4,5,6]}'
```

### Example Response

```json
{
    "job_id": 123,
    "status": "Job submitted successfully"
}
```
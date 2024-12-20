use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use threadpool::ThreadPool;
use std::sync::Arc;
use std::thread;

#[derive(Deserialize)]
struct JobRequest {
    job_ids: Vec<u32>,
}

#[derive(Serialize)]
struct JobResponse {
    job_id: u32,
    status: String,
}

async fn handle_jobs(
    pool: web::Data<Arc<ThreadPool>>,
    job_request: web::Json<JobRequest>,
) -> impl Responder {
    for &job_id in &job_request.job_ids {
        let pool = pool.clone();
        pool.execute(move || {
            println!("Processing job {}", job_id);
            thread::sleep(std::time::Duration::from_secs(2)); // Simulate work
            println!("Finished job {}", job_id);
        });
    }

    let responses: Vec<JobResponse> = job_request
        .job_ids
        .iter()
        .map(|&id| JobResponse {
            job_id: id,
            status: "Job is being processed".to_string(),
        })
        .collect();

    HttpResponse::Ok().json(responses)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = Arc::new(ThreadPool::new(3));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/submit_jobs", web::post().to(handle_jobs))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

use clap::Parser;
use futures::stream::{self, StreamExt};
use rand::Rng;
use reqwest::Client;
use std::time::Instant;

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value = "http://104.211.88.12:80/api/v1/predict")]
    url: String,
    #[arg(short, long, default_value_t = 1000)]
    requests: usize,
    #[arg(short, long, default_value_t = 100)]
    concurrency: usize,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let client = Client::new();
    let start = Instant::now();

    let stream = stream::iter(0..args.requests)
        .map(|_| {
            let (client, url) = (client.clone(), args.url.clone());
            async move {
                let mut rng = rand::thread_rng();
                let payload = serde_json::json!({
                    "hour": rng.gen_range(0.0..24.0),
                    "is_weekend": if rng.gen_bool(0.28) { 1.0 } else { 0.0 },
                    "demand": rng.gen_range(0.0..10.0),
                    "our_price": rng.gen_range(10.0..2000.0),
                    "competitor_price": rng.gen_range(10.0..2000.0)
                });
                client.post(&url).json(&payload).send().await
            }
        })
        .buffer_unordered(args.concurrency);

    let results = stream.collect::<Vec<_>>().await;
    let successes = results
        .iter()
        .filter(|r| matches!(r, Ok(res) if res.status().is_success()))
        .count();

    let el = start.elapsed().as_secs_f64();
    println!(
        "Done in {:.2}s | {}/{} OK | {:.2} req/s",
        el,
        successes,
        args.requests,
        args.requests as f64 / el
    );
}

use std::{fs::File, future::ready, io::BufReader, path::PathBuf, sync::Arc, time::Instant};

use axum::{
    Json, Router,
    extract::State,
    routing::{get, post},
};
use metrics_exporter_prometheus::PrometheusBuilder;
use serde::{Deserialize, Serialize};
use smartcore::{
    ensemble::random_forest_classifier::RandomForestClassifier, linalg::basic::matrix::DenseMatrix,
};
use tokio::net::TcpListener;

type Model = RandomForestClassifier<f64, i32, DenseMatrix<f64>, Vec<i32>>;

#[derive(Deserialize)]
struct PricingRequest {
    demand: f64,
    hour: f64,
    is_weekend: f64,
    our_price: f64,
    competitor_price: f64,
}

#[derive(Serialize)]
struct PricingResponse {
    will_sell: bool,
}

use clap::Parser;
use tower_http::trace::TraceLayer;

#[derive(Parser)]
#[command( version, about, long_about = None )]
struct Cli {
    #[arg(short, long, default_value = "../../../model.msgpack")]
    model: PathBuf,
}

#[tokio::main]
async fn main() {
    let builder = PrometheusBuilder::new();
    let handle = builder.install_recorder().unwrap();

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    let args = Cli::parse();

    let model_file = File::open(args.model).unwrap();
    let reader = BufReader::new(model_file);
    let model: Model = rmp_serde::from_read(reader).unwrap();

    let shared_model = Arc::new(model);

    let app = Router::new()
        .route("/api/v1/predict", post(predict_sale))
        .route("/metrics", get(move || ready(handle.render())))
        .with_state(shared_model)
        .layer(TraceLayer::new_for_http());

    let listner = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listner, app).await.unwrap();
}

async fn predict_sale(
    State(model): State<Arc<Model>>,
    Json(payload): Json<PricingRequest>,
) -> Json<PricingResponse> {
    let start = Instant::now();

    let features = vec![
        payload.hour,
        payload.is_weekend,
        payload.demand,
        payload.our_price,
        payload.competitor_price,
    ];

    let x_matrix = DenseMatrix::new(1, features.len(), features, false).unwrap();
    let prediction = model.predict(&x_matrix).unwrap();

    metrics::histogram!("inference_duration_seconds").record(start.elapsed().as_secs_f64());

    if prediction[0] == 1 {
        metrics::counter!("predictions_total", "outcome"=>"sale").increment(1);
    } else {
        metrics::counter!("predictions_total", "outcome"=>"no_sale").increment(1);
    }

    Json(PricingResponse {
        will_sell: prediction[0] == 1,
    })
}

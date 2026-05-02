use std::{fs::File, io::BufReader, path::PathBuf, sync::Arc};

use axum::{Json, Router, extract::State, routing::post};
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
        .with_state(shared_model)
        .layer(TraceLayer::new_for_http());

    let listner = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listner, app).await.unwrap();
}

async fn predict_sale(
    State(model): State<Arc<Model>>,
    Json(payload): Json<PricingRequest>,
) -> Json<PricingResponse> {
    let features = vec![
        payload.hour,
        payload.is_weekend,
        payload.demand,
        payload.our_price,
        payload.competitor_price,
    ];

    let x_matrix = DenseMatrix::new(1, features.len(), features, false).unwrap();
    let prediction = model.predict(&x_matrix).unwrap();
    Json(PricingResponse {
        will_sell: prediction[0] == 1,
    })
}

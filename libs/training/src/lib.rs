use polars::prelude::*;
use serde::Serialize;
use smartcore::ensemble::random_forest_classifier::RandomForestClassifier;
use smartcore::linalg::basic::matrix::DenseMatrix;
use smartcore::metrics::accuracy;
use smartcore::model_selection::train_test_split;
use std::fs::File;
use std::io::{BufReader, BufWriter};

pub fn training() {
    let mut file = File::open("pricing_data.parquet").unwrap();
    let df = ParquetReader::new(&mut file).finish().unwrap();

    let df_64 = df
        .lazy()
        .with_columns(vec![
            col("hours").cast(DataType::Float64),
            col("is_weekend").cast(DataType::Float64),
            col("demand").cast(DataType::Float64),
            col("our_prices").cast(DataType::Float64),
            col("did_it_sell").cast(DataType::Int32),
            col("competitor_price").cast(DataType::Float64),
        ])
        .collect()
        .unwrap();

    let features_df = df_64.drop("did_it_sell").unwrap();
    let targer_series = df_64.column("did_it_sell").unwrap();

    let rows = features_df.height();
    let cols = features_df.width();

    let mut x_data = Vec::with_capacity(rows * cols);

    for row_idx in 0..rows {
        for col_idx in 0..cols {
            let val = features_df
                .column(features_df.get_column_names()[col_idx])
                .unwrap()
                .f64()
                .unwrap()
                .get(row_idx)
                .unwrap_or(0.0);
            x_data.push(val);
        }
    }

    let x_matrix = DenseMatrix::new(rows, cols, x_data, false).unwrap();
    let y_vec: Vec<i32> = targer_series.i32().unwrap().into_no_null_iter().collect();

    let (x_train, x_test, y_train, y_test) = train_test_split(&x_matrix, &y_vec, 0.2, true, None);

    let model = RandomForestClassifier::fit(&x_train, &y_train, Default::default()).unwrap();

    let prediction = model.predict(&x_test).unwrap();
    let acc = accuracy(&y_test, &prediction);

    println!("Accuracy : {:.2}%", acc * 100.0);

    let model_file = File::create("model.msgpack").unwrap();
    let mut writter = BufWriter::new(model_file);

    model
        .serialize(&mut rmp_serde::Serializer::new(&mut writter))
        .unwrap();
    println!("Model saved to pricving");
}

pub fn load_and_predict(features: Vec<f64>) -> i32 {
    let model_file = File::open("model.msgpack").unwrap();
    let reader = BufReader::new(model_file);

    let model: RandomForestClassifier<f64, i32, DenseMatrix<f64>, Vec<i32>> =
        rmp_serde::from_read(reader).unwrap();
    let x_matrix = DenseMatrix::new(1, features.len(), features, false).unwrap();
    let prediction = model.predict(&x_matrix).unwrap();

    prediction[0]
}

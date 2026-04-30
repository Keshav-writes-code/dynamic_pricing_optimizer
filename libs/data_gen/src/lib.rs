use polars::prelude::*;
use rand_distr::{Bernoulli, Distribution, Normal, Uniform};
use std::{env, fs::File};

pub fn gen_data() {
    let arg = env::args().nth(1).unwrap();
    let num_rows = arg.parse().unwrap();

    let mut rng = rand::rng();

    //INitalize Distribution
    let hour_dist = Uniform::new(0, 24).unwrap();
    let weekend_dist = Bernoulli::new(0.28).unwrap();
    let demand = Normal::new(1.0, 0.3).unwrap();
    let base_price_dist = Normal::new(40.0, 10.0).unwrap();

    // Preallocate Vectors
    let mut hours = Vec::with_capacity(num_rows);
    let mut is_weekends = Vec::<bool>::with_capacity(num_rows);
    let mut demands = Vec::<f32>::with_capacity(num_rows);
    let mut our_prices = Vec::<f32>::with_capacity(num_rows);
    let mut did_it_sell = Vec::<bool>::with_capacity(num_rows);
    let mut competitor_prices = Vec::<f32>::with_capacity(num_rows);

    for _ in 0..num_rows {
        let hour = hour_dist.sample(&mut rng);
        let is_weekend = weekend_dist.sample(&mut rng);
        let comp_price: f32 = f32::max(base_price_dist.sample(&mut rng), 10.0);

        //demand spikes higher during evenings and weekends
        let mut demand: f32 = demand.sample(&mut rng);

        if hour > 17 && hour < 21 {
            demand += 0.4
        }
        if is_weekend {
            demand += 1.0
        }

        // NOTE: We try out diffrent strateges here for best price
        let our_price = Uniform::new(30.0, 50.0).unwrap().sample(&mut rng);

        // NOTE: Calculate the did_it_sell
        // Basically if
        // - demand is higher and
        // - competetor price is higher and
        // - our price is lower
        // the sold probability will be higher

        let weekend_mul = if is_weekend { 0.5 } else { 0.0 };
        let logits = 0.0 - (0.20 * our_price) + (0.15 * comp_price) + (2.5 * demand) + weekend_mul;

        let probability = 1.0 / (1.0 + (-logits).exp());

        let sold = Bernoulli::new(probability as f64).unwrap().sample(&mut rng);

        hours.push(hour);
        is_weekends.push(is_weekend);
        demands.push(demand);
        our_prices.push(our_price);
        did_it_sell.push(sold);
        competitor_prices.push(comp_price);
    }

    let mut df = DataFrame::new(
        num_rows,
        vec![
            Series::new("hours".into(), hours).into(),
            Series::new("is_weekend".into(), is_weekends).into(),
            Series::new("demand".into(), demands).into(),
            Series::new("our_prices".into(), our_prices).into(),
            Series::new("did_it_sell".into(), did_it_sell).into(),
            Series::new("competitor_price".into(), competitor_prices).into(),
        ],
    )
    .unwrap();
    let mut file = File::create("pricing_data.csv").expect("Cannot create a file");
    // ParquetWriter::new(&mut file).finish(&mut df).unwrap();

    CsvWriter::new(&mut file).finish(&mut df).unwrap();

    println!(
        "Successfully generated pricing_data.parquet with {} rows",
        num_rows
    );
}

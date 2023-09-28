use warp::Filter;
use serde::Deserialize;

use rust_bert::pipelines::sentiment::{SentimentModel, SentimentPolarity};

thread_local! {
    static MODEL: SentimentModel = SentimentModel::new(Default::default()).expect("Failed to load model!");
}

#[derive(Deserialize, Debug)]
struct SentimentInput {
    input: Vec<String>,
}

pub fn get_route() -> impl Filter<Extract = (String,), Error = warp::Rejection> + Clone {
    warp::post()
        .and(warp::path("v1"))
        .and(warp::path("sentiment"))
        .and(warp::path::end())
        .and(warp::body::content_length_limit(1024 * 16)) // TODO: Figure out a sensible size?
        .and(warp::body::json())
        .map(|data: SentimentInput| {
            println!("data: {:#?}", data);
            MODEL.with(|model| {
                let output = model.predict(&data.input.iter().map(|s| s.as_str()).collect::<Vec<&str>>());
                serde_json::to_string(&output.into_iter().map(|sentiment| sentiment.score * (if sentiment.polarity == SentimentPolarity::Negative { -1.0 } else { 1.0 })).collect::<Vec<f64>>()).unwrap()
            })
        })
}

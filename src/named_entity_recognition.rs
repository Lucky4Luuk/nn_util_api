use warp::Filter;
use serde::Deserialize;

use rust_bert::pipelines::ner::NERModel;

thread_local! {
    static MODEL: NERModel = NERModel::new(Default::default()).expect("Failed to load model!");
}

#[derive(Deserialize, Debug)]
struct NERInput {
    input: Vec<String>,
}

pub fn get_route() -> impl Filter<Extract = (String,), Error = warp::Rejection> + Clone {
    warp::post()
        .and(warp::path("v1"))
        .and(warp::path("ner"))
        .and(warp::path::end())
        .and(warp::body::content_length_limit(1024 * 16)) // TODO: Figure out a sensible size?
        .and(warp::body::json())
        .map(|data: NERInput| {
            println!("data: {:#?}", data);
            MODEL.with(|ner_model| {
                let output = ner_model.predict(&data.input);
                println!("output: {:?}", output);
                "hello!".to_string()
            })
        })
}

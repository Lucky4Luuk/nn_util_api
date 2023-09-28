use warp::Filter;

mod named_entity_recognition;
mod sentiment_analysis;

#[tokio::main]
async fn main() {
    // GET /ping/
    let ping = warp::path!("ping" / String)
        .map(|| "Pong!");

    let routes = ping
        .or(named_entity_recognition::get_route())
        .or(sentiment_analysis::get_route());

    warp::serve(routes)
        .run(([0, 0, 0, 0], 3030))
        .await;
}

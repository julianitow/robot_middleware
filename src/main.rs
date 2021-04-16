use warp::Filter;

pub struct Client {
}



pub struct Event {
    topic: String,
    user_id: Option<usize>,
    message: String,
}



fn main() {
    println!("WS server");

    let ws_route = warp::path("ws")
    .and(warp::ws())
    .and(warp::path::param());

}
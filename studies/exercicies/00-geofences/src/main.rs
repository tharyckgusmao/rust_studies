use std::{ sync::{Arc, Mutex}, collections::HashMap, net::SocketAddr};


use redis::Client;
use warp::Filter;

fn run(conn: Client) {
    tokio::task::spawn(async move {
        let publish_conn = conn.get_async_connection().await.unwrap();
        let mut pubsub_conn = publish_conn.into_pubsub();

        let subscribed = pubsub_conn.subscribe("geo").await;
        println!("Subscribed on :{:?}", subscribed);
        
        while let Some(result) = pubsub_conn.on_message().next().await {
            let payload = result.get_payload::<String>().unwrap();
            // let payloadJson: Comand = serde_json::from_str(&payload).unwrap();

            println!("<MSG>: {:?}", payload);
        }
    });
}

#[tokio::main]
async fn main() {

    let conn= redis::Client::open("redis://127.0.0.1:9851").unwrap();

    run(conn);
    
    println!("connected");

    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    warp::serve(hello)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

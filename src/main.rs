mod store;

use store::{Store, Entity};

use std::env;
use std::net::SocketAddr;

use warp::{http, Filter};
use warp::body::bytes;
use warp::filters::path::FullPath;
use warp::hyper::body::Bytes;



async fn create_entity(
    mut store: Store,
    path: FullPath,
    entity: Entity
    ) -> Result<impl warp::Reply, warp::Rejection> {
    store.create_entity(path.as_str().to_owned(), entity);
    Ok(warp::reply::with_status(
            "OK",
            http::StatusCode::CREATED,
            ))
}

async fn get_entities(
    store: Store,
    path: FullPath
    ) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
        let result = store.get_entities(&path.as_str().to_owned());
        Ok(Box::new(warp::reply::json(&result)))
}

fn json_body() -> impl Filter<Extract = (Entity,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16)
        .and(bytes())
        .and_then(|bytes : Bytes| async move {
            serde_json::from_slice(&bytes).map_err(|_err| {
                warp::reject::reject()
            })
        })
}

#[tokio::main]
async fn main() {
    let socket : SocketAddr = env::var("HUMUS_SOCKET_ADDRESS").ok()
        .and_then(|var| { var.parse().ok() })
        .unwrap_or(([127, 0 ,0, 1], 3030).into());
    pretty_env_logger::init();
    let store = Store::new();
    let store_filter =  warp::any().map(move || store.clone());

    let create_entity_route = warp::post()
        .and(store_filter.clone())
        .and(warp::path::full())
        .and(json_body())
        .and_then(create_entity);

    let get_entities_route = warp::get()
        .and(store_filter.clone())
        .and(warp::path::full())
        .and_then(get_entities);

    let routes = create_entity_route.or(get_entities_route);

    warp::serve(routes.with(warp::log("humus::main")))
        .run(socket)
        .await;
}

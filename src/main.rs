mod store;

use store::{SynchronizedStore, Entity};

use warp::{http, Filter};



async fn create_entity(
    key: String,
    entity: Entity,
    store: SynchronizedStore
    ) -> Result<impl warp::Reply, warp::Rejection> {
    store.store.write().entities.insert(key, entity);
    Ok(warp::reply::with_status(
            "OK",
            http::StatusCode::CREATED,
            ))
}

async fn get_entities(
    store: SynchronizedStore
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let store_lock = store.store.read();
        let result = store_lock.entities.clone();
        Ok(warp::reply::json(&result))
}

async fn get_entity(
    key: String,
    store: SynchronizedStore
    ) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
        let store_lock = store.store.read();
        let result = store_lock.entities.get(&key);
        match result {
            Some(entity) => Ok(Box::new(warp::reply::json(&*entity))),
            None => Ok(Box::new(warp::reply::with_status(
                "Not found",
                http::StatusCode::NOT_FOUND
                ))),
        }
}

fn json_body() -> impl Filter<Extract = (Entity,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

#[tokio::main]
async fn main() {
    let store = SynchronizedStore::new();
    let store_filter =  warp::any().map(move || store.clone());

    let create_entity_route = warp::put()
        .and(warp::path::param())
        .and(warp::path::end())
        .and(json_body())
        .and(store_filter.clone())
        .and_then(create_entity);

    //let update_entity_route = warp::patch()
    //    .and(warp::path::param())
    //    .and(warp::path::end())
    //    .and(json_body())
    //    .and(store_filter.clone())
    //    .and_then(update_entity);

    let get_entities_route = warp::get()
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(get_entities);

    let get_entity_route = warp::get()
        .and(warp::path::param())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(get_entity);

    let routes = create_entity_route
        //.or(update_entity_route)
        .or(get_entity_route)
        .or(get_entities_route);

    warp::serve(routes)
        .run(([127,0,0,1], 3030))
        .await;
}

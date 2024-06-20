mod store;

use store::{Store, Entity};

use warp::{http, Filter};



async fn create_entity(
    key: String,
    entity: Entity,
    mut store: Store
    ) -> Result<impl warp::Reply, warp::Rejection> {
    store.create_entity(key, entity);
    Ok(warp::reply::with_status(
            "OK",
            http::StatusCode::CREATED,
            ))
}

async fn get_entities(
    store: Store
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let result = store.get_entities();
        Ok(warp::reply::json(&result))
}

async fn get_entity(
    key: String,
    store: Store
    ) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
        let result = store.get_entity(key);
        match result {
            Some(entity) => Ok(Box::new(warp::reply::json(&entity))),
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
    let store = Store::new();
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

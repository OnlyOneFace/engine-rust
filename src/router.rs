use actix_web::{post, web, Responder};
use std::collections::HashMap;
use serde::Serialize;


#[derive(Serialize)]
enum AnyThing<T> {
    BaseType(T),
    Map(HashMap<String, String>),
}

#[post("/{id}/{name}")]
pub async fn index<T>(web::Path((id, name)): web::Path<(u32, String)>,
                      query: web::Query<HashMap<String, String>>,
                      body: web::Json<HashMap<String, String>>) -> impl Responder {
    let mut temp_map: HashMap<&str, AnyThing<T>> = HashMap::new();
    temp_map.insert("hello", AnyThing::BaseType(name));
    temp_map.insert("id", AnyThing::BaseType(id));
    temp_map.insert("query", AnyThing::Map(query.into_inner()));
    //temp_map.insert("header");
    temp_map.insert("body", AnyThing::Map(body.into_inner()));
    web::Json(temp_map)
}
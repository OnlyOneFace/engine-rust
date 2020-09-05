use actix_web::{post, web, Responder};
use std::collections::HashMap;

enum JsonValue<T> {
    tv(T),
    AnotherHashMap(HashMap<String, String>),
}

#[post("/{id}/{name}")]
pub async fn index(web::Path((id, name)): web::Path<(u32, String)>, query: web::Query<HashMap<String, String>>, body: web::Json<HashMap<String, String>>) -> impl Responder {
    let mut temp_map: HashMap<&str, JsonValue<T>> = HashMap::new();
    temp_map.insert("hello", JsonValue::tv(name));
    temp_map.insert("id", JsonValue::tv(id));
    temp_map.insert("query", JsonValue::AnotherHashMap(query.into_inner()));
    temp_map.insert("body", JsonValue::AnotherHashMap(body.into_inner()));
    web::Json(temp_map)
}
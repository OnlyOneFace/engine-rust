use actix_web::{post, web, dev, Responder, HttpMessage};
use std::collections::HashMap;
use std::any::Any;
use serde::Serialize;

#[derive(Serialize)]
struct AnyThing {
    name: Box<dyn Any>,
}

impl AnyThing {
    fn new(a: Box<dyn Any>) -> AnyThing {
        AnyThing {
            name: a,
        }
    }
}

#[post("/{id}/{name}")]
pub async fn index(web::Path((id, name)): web::Path<(u32, String)>,
                   query: web::Query<HashMap<String, String>>,
                   body: web::Json<HashMap<String, String>>) -> impl Responder {
    let mut temp_map: HashMap<&str, AnyThing> = HashMap::new();
    temp_map.insert("hello", AnyThing::new(Box::new(name)));
    temp_map.insert("id", AnyThing::new(Box::new(id)));
    temp_map.insert("query", AnyThing::new(Box::new(query.into_inner())));
    //temp_map.insert("header");
    temp_map.insert("body", AnyThing::new(Box::new(body.into_inner())));
    web::Json(temp_map)
}
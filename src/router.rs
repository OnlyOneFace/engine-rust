use actix_web::{post, web, Responder};
use std::collections::HashMap;
use serde::{Serialize, Serializer};
use serde::ser::SerializeMap;


enum AnyThing {
    String(String),
    U32(u32),
    MapIter(HashMap<String, String>),
}

impl Serialize for AnyThing {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where
        S: Serializer {
        match *self {
            AnyThing::U32(value) => serializer.serialize_u32(value),
            AnyThing::String(ref value) => serializer.serialize_str(value),
            AnyThing::MapIter(ref map_var) => {
                let mut map = serializer.serialize_map(Some(map_var.len()))?;
                for (k, v) in map_var {
                    map.serialize_entry(k, v)?;
                }
                map.end()
            }
        }
    }
}

#[post("/{id}/{name}")]
pub async fn index(web::Path((id, name)): web::Path<(u32, String)>,
                   query: web::Query<HashMap<String, String>>,
                   body: web::Json<HashMap<String, String>>) -> impl Responder {
    let mut temp_map: HashMap<&str, AnyThing> = HashMap::new();
    temp_map.insert("hello", AnyThing::String(name));
    temp_map.insert("id", AnyThing::U32(id));
    temp_map.insert("query", AnyThing::MapIter(query.into_inner()));
    //temp_map.insert("header");
    temp_map.insert("body", AnyThing::MapIter(body.into_inner()));
    web::Json(temp_map)
}
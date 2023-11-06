use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct MyMessage {
    message: String,
}

// when the post method is called, the Json Request Body will be extracted and sent
// to this give_my_json(), the Json, it can deserialize request bodies into some type
// that implements serde::Deserialize
pub async fn give_my_json(Json(message): Json<MyMessage>) -> Json<MyMessage> {
    // the message var is of type MyMessage, so the Json from request is deserialized into
    // MyMessage
    Json(message)
}

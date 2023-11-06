use axum::Json;
use serde::{Deserialize, Serialize};

// making our struct type to implement serialize and deserialize
#[derive(Deserialize, Serialize)]
pub struct MyMessage {
    message: String,
}

#[derive(Serialize)]
pub struct MyResponse {
    message: String,
    reply: String,
}

// when the post method is called, the json in Request Body will be extracted and sent
// to this give_my_json(), the Json() will deserialize request bodies into some type
// that implements serde::Deserialize
pub async fn give_my_json(Json(mymessage): Json<MyMessage>) -> Json<MyResponse> {
    // the message var is of type MyMessage, so the Json from request is deserialized into
    // MyMessage

    // as our MyResponse struct can be serialized into a Json, we can send it back as response
    Json(MyResponse {
        message: mymessage.message,
        reply: "Hi from Server".to_string(),
    })
}

// when the post method is called, the Text from the Request Body will be sent
// to this give_my_string() fn and will be stored in body
pub async fn give_my_string(body: String) -> String {
    body
}

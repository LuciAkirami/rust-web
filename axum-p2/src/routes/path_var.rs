use axum::extract::Path;

// Path extractor captures path variables from url and parses them using serde
pub async fn path_var(Path(name): Path<String>) -> String {
    name
}

// Even though the above function is defined, when a user goes to /path_var/Lucifer
// only the below fn will be called and above fn will not be called
// hence we can separate the path_var to use different fn for a single entity like /path_var/Lucifer
pub async fn path_var_hardcoded() -> String {
    "Hey it's you, Lucifer".to_string()
}

#[tokio::main]
async fn main() {
    let mut map = std::collections::HashMap::new();
    map.insert("lang", "rust");
    map.insert("body", "json");

    let url = "https://jsonplaceholder.typicode.com/posts";
    let client = reqwest::Client::new();
    let response = client.post(url).json(&map).send().await.unwrap();
    println!("{:#?}", response);
}

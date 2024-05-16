use std::collections::HashMap;

#[tokio::test]
async fn test_root() {
    let client = reqwest::Client::new();
    let res = client
        .get("http://127.0.0.1:3000/")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    assert_eq!(res, "Hello, World!");
}

#[tokio::test]
async fn get_review() {
    let client = reqwest::Client::new();
    let res = client
        .get("http://127.0.0.1:3000/review/1")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    assert_eq!(res, "{\"id\":1,\"name\":\"Hello\",\"review\":\"World\"}")
}

#[tokio::test]
async fn create_review() {
    let mut map = HashMap::new();
    map.insert("name", "hello");
    map.insert("review", "world");
    let client = reqwest::Client::new();
    let res = client
        .post("http://127.0.0.1:3000/review")
        .json(&map)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    assert_eq!(res, "{\"id\":1,\"name\":\"Hello\",\"review\":\"World\"}")
}

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
    println!("{}", res);
}

#[tokio::test]
async fn test_root() {
    let client = reqwest::Client::new();
    let res = client
        .get("http://localhost:3000/")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    println!("{}", res);
}

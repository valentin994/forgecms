extern crate forgecms;

use forgecms::routes::reviews::Review;
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

    assert_eq!(res, String::from("Hello, World!"));
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
        .expect("Should return created review.")
        .json::<Review>()
        .await
        .unwrap();
    println!("Running first query!");
    println!("{res:?}");
    assert_eq!(res , Review{ id: res.id, name: String::from("hello"), review: String::from("world")});
}


// #[tokio::test]
// async fn get_review() {
//     let client = reqwest::Client::new();
//     let res = client
//         .get("http://127.0.0.1:3000/review/1")
//         .send()
//         .await
//         .unwrap()
//         .text()
//         .await
//         .unwrap();
//     assert_eq!(res, "{\"id\":1,\"name\":\"Hello\",\"review\":\"World\"}")
// }

// #[tokio::test]
// async fn update_review() {
//     let mut map = HashMap::new();
//     map.insert("name", "hello");
//     map.insert("review", "world2");
//     let client = reqwest::Client::new();
//     let res = client
//         .patch("http://127.0.0.1:3000/review/1")
//         .json(&map)
//         .send()
//         .await
//         .unwrap()
//         .text()
//         .await
//         .unwrap();
//     assert_eq!(res, "{\"id\":1,\"name\":\"hello\",\"review\":\"world2\"}")
// }


// #[tokio::test]
// async fn delete_review() {
//     let client = reqwest::Client::new();
//     let res = client
//         .delete("http://127.0.0.1:3000/review/1")
//         .send()
//         .await
//         .unwrap()
//         .text()
//         .await
//         .unwrap();
//     assert_eq!(res, "Deleted review with id: 1".to_string())
// }



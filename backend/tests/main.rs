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

    assert_eq!(
        res,
        Review {
            id: res.id,
            name: String::from("hello"),
            review: String::from("world")
        }
    );
}

#[tokio::test]
async fn get_review() {
    let mut map = HashMap::new();
    map.insert("name", "hello");
    map.insert("review", "world");
    let client = reqwest::Client::new();
    let created_review = client
        .post("http://127.0.0.1:3000/review")
        .json(&map)
        .send()
        .await
        .expect("Should return created review.")
        .json::<Review>()
        .await
        .unwrap();
    let res = client
        .get(format!(
            "http://127.0.0.1:3000/review/{}",
            created_review.id
        ))
        .send()
        .await
        .expect("Should return created review")
        .json::<Review>()
        .await
        .unwrap();
    assert_eq!(res, created_review)
}

#[tokio::test]
async fn update_review() {
    let mut map = HashMap::new();
    map.insert("name", "hello");
    map.insert("review", "world");
    let client = reqwest::Client::new();
    let created_review = client
        .post("http://127.0.0.1:3000/review")
        .json(&map)
        .send()
        .await
        .expect("Should return created review.")
        .json::<Review>()
        .await
        .unwrap();

    let mut new_name = HashMap::new();
    new_name.insert("name", "new");
    let client = reqwest::Client::new();
    let res = client
        .patch(format!(
            "http://127.0.0.1:3000/review/{}",
            created_review.id
        ))
        .json(&new_name)
        .send()
        .await
        .expect("Update the name")
        .json::<Review>()
        .await
        .unwrap();
    assert_eq!(
        res,
        Review {
            id: created_review.id,
            name: String::from("new"),
            review: String::from("world")
        }
    );
}

#[tokio::test]
async fn delete_review() {
    let mut map = HashMap::new();
    map.insert("name", "hello");
    map.insert("review", "world");
    let client = reqwest::Client::new();
    let created_review = client
        .post("http://127.0.0.1:3000/review")
        .json(&map)
        .send()
        .await
        .expect("Should return created review.")
        .json::<Review>()
        .await
        .unwrap();

    let res = client
        .delete(format!(
            "http://127.0.0.1:3000/review/{}",
            created_review.id
        ))
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    assert_eq!(
        res,
        format!("Deleted review with id: {}", created_review.id)
    )
}

use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use serde_wasm_bindgen::{to_value, from_value};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Post {
    pub slug: String,
    pub title: String,
    pub excerpt: String,
    pub category: String,
    pub featured: bool,
    pub popular: bool,
    pub cover: String,
    pub created: String,
}

#[wasm_bindgen]
pub fn filter_posts(posts: JsValue, search_word: &str) -> JsValue {
    let posts: Vec<Post> = from_value(posts).unwrap();
    let mut filtered_posts: Vec<Post> = Vec::new();

    for post in posts.iter() {
        if post.title.to_lowercase().contains(&search_word.to_lowercase()) {
            filtered_posts.push(post.clone());
        }
        if filtered_posts.len() >= 5 {
            break;
        }
    }

    let filtered_posts_js = to_value(&filtered_posts).unwrap();
    filtered_posts_js
}

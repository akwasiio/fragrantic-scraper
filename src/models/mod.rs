use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Category {
    pub id: u64,
    name: String,
    votes_up: usize,
    votes_down: usize,
    category_type: String,
    slug: String,
    leaders: Vec<Nomination>
}

#[derive(Deserialize, Debug)]
pub struct CategoryDetail {
    pub id: String,
    pub name: String,
    pub nominations: Vec<Nomination>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Nomination {
    pub id: u64,
    pub name: String,
    pub item_id: u64,
    pub category_id: u64,
    pub votes_up: usize,
    pub votes_down: usize,
    pub sort_score: isize,
    pub url: String,
    pub picture: String,
}

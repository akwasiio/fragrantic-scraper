use reqwest::{blocking::Client, header};

use crate::models::{Category, CategoryDetail, Nomination};

mod models;

fn main() {
    // reqwest::blocking::get("https://httpbin.org/ip").unwrap()
    //     .json::<HashMap<String, String>>()?;
    let awards_url = "https://www.fragrantica.com/awards.php?show_mode=json_selected_categories";

    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::USER_AGENT,
        header::HeaderValue::from_static("Mozilla/5.0...."),
    );
    let client = reqwest::blocking::Client::builder()
        // .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3")
        .default_headers(headers)
        .build()
        .unwrap();

    let categories = get_categories(&client, awards_url);
    let first_category = categories.first().unwrap();
    let nominations = get_nominations_per(first_category.id, &client);

    let first_nomination = nominations.first().unwrap();
    let nom_url = "https://www.fragrantica.com/";
    let req = client
        .get(format!("{}{}", nom_url, &first_nomination.url))
        .send()
        .unwrap();
    let html_content = req.text().unwrap();
    let document = scraper::Html::parse_document(&html_content);
    let selector = scraper::Selector::parse("div.accord-bar").unwrap();
    let accord_boxes = document.select(&selector);
    let texts: Vec<_> = accord_boxes
        .map(|element| element.text().collect::<Vec<_>>().join(""))
        .collect();

    println!("Size: {}", &texts.len());

    for text in texts {
        println!("{}", text)
    }

    let levels = vec!["top", "middle", "base"];

    for level in levels {
        let selector = scraper::Selector::parse(&format!(
            "pyramid-level[notes='{}'] div[style*='flex-direction: column']",
            level
        ))
        .unwrap();

        println!("Pyramid Level: {}", level);

        for div in document.select(&selector) {
            let text = div.text().collect::<Vec<_>>().join("").trim().to_owned();
            println!("{}", text);
        }

        println!();
    }
}

fn get_categories(client: &Client, url: &str) -> Vec<Category> {
    let res = client.get(url).send().unwrap();

    res.json().ok().unwrap()
}

fn get_nominations_per(category_id: u64, client: &Client) -> Vec<Nomination> {
    let url = format!(
        "https://www.fragrantica.com/awards.php?show_mode=json_nominations&category_id={}",
        category_id
    );
    let res = client.get(url).send().unwrap();

    // println!("Category ID: {}", category_id);
    let cat_detail: CategoryDetail = res.json().unwrap();

    cat_detail.nominations
}

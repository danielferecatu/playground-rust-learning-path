use std::collections::HashMap;

fn main() {
    let mut reviews: HashMap<String, String> = HashMap::new();

    reviews.insert(String::from("Ancient Roman History"), String::from("Very accurate."));
    reviews.insert(String::from("Cooking with Rhubarb"), String::from("Sweet recipes."));
    reviews.insert(String::from("Programming in Rust"), String::from("Great examples."));

    // Look for a specific review
    let book: &str = "Programming in Rust";
    println!("Review for \'{}\': {:?}", book, reviews.get(book));


    // Remove book review
    let obsolete: &str = "Ancient Roman History";
    reviews.remove(obsolete);
    println!("\'{}\' removed.", obsolete);

    // Confirm book review removed
    println!("Review for \'{}\': {:?}", obsolete, reviews.get(obsolete));
}

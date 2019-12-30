use std::collections::HashMap;

/// Use a HashMap to store the page content while there are only a few, slow changing pages.
pub fn get_content(page_name: String) -> Option<String> {
    let page_content: HashMap<String, String> = [
        (
            String::from("afrita"),
            include_str!("../../content/afrita.html").to_string(),
        ),
        (
            String::from("tech"),
            include_str!("../../content/tech.html").to_string(),
        ),
        (
            String::from("sports"),
            include_str!("../../content/sports.html").to_string(),
        ),
        (
            String::from("crafts"),
            include_str!("../../content/crafts.html").to_string(),
        ),
        (
            String::from("travel"),
            include_str!("../../content/travel.html").to_string(),
        ),
        (
            String::from("exploring"),
            include_str!("../../content/exploring.html").to_string(),
        ),
        (
            String::from("about"),
            include_str!("../../content/about.html").to_string(),
        ),
    ]
    .iter()
    .cloned()
    .collect();
    let found = page_content.get(&page_name);
    match found {
        Some(page_data) => Some(page_data.clone()),
        None => None,
    }
}

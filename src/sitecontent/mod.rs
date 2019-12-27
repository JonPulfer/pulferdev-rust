use std::collections::HashMap;

/// Use a HashMap to store the page content while there are only a few, slow changing pages.
pub fn get_content(page_name: String) -> Option<String> {
    let page_content: HashMap<String, String> = [
        (String::from("afrita"), String::from("stuff about afrita")),
        (String::from("about"), String::from("<p class=\"centered\">
            I'm a distributed systems programmer (predominantly Rust and Go) living in Ipswich and currently commuting into London.
            Although this does mean I travel more than some, it enables me to work with some really interesting
            people using technology that challenges me. I enjoy helping people and fixing things, which provides
            me with plenty to learn and helps keep me energised.
            </p>
            <p class=\"centered\">
            I'm always happy to hear about new opportunities and welcome connections via one of the social
            accounts in the links above.
            </p>
            <p class=\"centered\">
            When I'm not working, the main draw on my time is <a href=\"/afrita\">Afrita</a>.
            </p>"))
    ].iter().cloned().collect();
    let found = page_content.get(&page_name);
    match found {
        Some(page_data) => Some(page_data.clone()),
        None => None,
    }
}

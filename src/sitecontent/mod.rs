use std::collections::HashMap;

/// Use a HashMap to store the page content while there are only a few, slow changing pages.
pub fn get_content(page_name: String) -> Option<String> {
    let page_content: HashMap<String, String> = [
        (String::from("afrita"), String::from("<div class=\"container-fluid\">
        <div class=\"row\">
        <div class=\"col centered\">
        <img src=\"/static/images/afrita.png\" alt=\"Afrita sailing in the river Orwell\">
        <p><a href=\"http://co32.org\">Afrita - Contessa 32</a></p>
        <ul class=\"list-unstyled\">
        <li>Length (LOA): 32 ft / 9.75m</li>
        <li>Width (Beam): 10ft / 3m</li>
        <li>Keel (Draft): 5ft 6in / 1.75m</li>
        <li>Year (Launched): 1978</li>
        </ul>
        </div>
        <div class=\"col\">
        <p class=\"centered\">Afrita was completed in 1978 by the Jeremy Rogers boat yard in Lymington. Originally she was called Sunrise and then, at some point before 1984, her name was changed. I purchased Afrita from a lovely family in Scotland in 2016 and have been enjoying sailing her ever since.</p>

        <p class=\"centered\">Currently Afrita is berthed at Ipswich Haven marina in the historic dock in town. This is only a few minutes walk from home, or to one of the local cafes/restaurants or shops.</p>

        <p class=\"centered\">I'm still building up my offshore experience with the goal to eventually cross oceans. My long term dream is to explore the pacific islands. Particularly Galapagos, Hawaii, French Polynesia and Fiji before arriving in New Zealand. Like all sailing plans, it's very loosely defined and I would be just as happy spending time sailing around the UK!</p>
        </div>
        </div>
        <div class=\"row\">
        <p>This is Afrita's last reported position</p>
        <script type=\"text/javascript\">
        // Map appearance
        var width=\"100%\";         // width in pixels or percentage
    var height=\"300\";         // height in pixels
    var names=true;           // always show ship names (defaults to false)

    // Single ship tracking
    var mmsi=\"235118438\";        // display latest position (by IMO, overrides MMSI)
    var show_track=true;      // display track line (last 24 hours)
    </script>
        <script type=\"text/javascript\" src=\"https://www.vesselfinder.com/aismap.js\"></script>
        </div>
        </div>")),
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

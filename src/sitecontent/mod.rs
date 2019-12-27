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
        (String::from("tech"), String::from("<div class=\"container-fluid\">
                    <div class=\"row\">
                        <div class=\"col centered\">
                            <p>
                                I originally qualified as a wooden boat builder and had no plans to follow a career in
                                technology. This all changed when I found that the luxury goods market was collapsing
                                and boats definitely fell into this market at the time.
                            </p>
                            <p>
                                I had been messing around with various computer based things from an early age and so
                                decided to try and make a living from it. I started a course in computer science but
                                only managed to complete the first year before life pressed me to get earning. I
                                registered with an agency and started a temporary position at Jeyes in Thetford, UK.
                            </p>
                            <p>
                                I was really lucky to get that position as my first one and the team there were really
                                supportive. I got involved in all I could, learning as much as I was able to absorb. It
                                was a great time for me and I ended up moving from a temporary contract to a permanent one.
                            </p>
                            <p>
                                Anyway, enough of my origin story, here are some of the things I'm messing around with
                                at the moment...
                            </p>
                        </div>
                    </div>
                    <div class=\"row\">
                        <div class=\"col centered\">
                            <h1>Rust</h1>
                        </div>
                        <div class=\"col centered\">
                            <p>
                                In the last few years I have been developing more systems using the Rust language. For
                                a lot of the systems I work on, it's a great fit and I really enjoy writing code using it.
                            </p>
                            <p>
                                More recently I have been looking at both WASM and embedded systems and Rust is proving
                                to be a really nice fit here too.
                            </p>
                        </div>
                    </div>
                    <div class=\"row\">
                        <div class=\"col centered\">
                            <p>
                                I started writing Go programs fairly early on in it's existence and really enjoyed the
                                simple yet productive nature of the language. I still appreciate this and I find it
                                compliments Rust really well since where one starts being awkward the other picks up.
                            </p>
                            <p>
                                I mainly use Go when I'm interacting with multiple API services or the system doesn't
                                involve very loosely defined data objects.
                            </p>
                        </div>
                        <div class=\"col centered\">
                            <h1>Go</h1>
                        </div>
                    </div>
                    <div class=\"row\">
                        <div class=\"col centered\">
                            <h1>Robotics</h1>
                        </div>
                        <div class=\"col centered\">
                            <p>
                                I'm currently researching building a battle robot as a team effort involving a few
                                friends. I am hoping that I can connect with a local high school to get a group together
                                to design, build and compete the robot.
                            </p>
                        </div>
                    </div>
                </div>")),
        (String::from("sports"), String::from("<div class=\"container-fluid\">
                    <p class=\"centered\">
                        I enjoy most sports and have competed in a few different ones both regionally and nationally.
                    </p>
                    <div class=\"row\">
                        <div class=\"col centered\">

                            <img src=\"/static/images/phantom_going_over.png\" alt=\"Phantom going over\">
                        </div>
                        <div class=\"col centered\">
                        <h1>Dinghy sailing</h1>
                            <p>
                                Sailing is a major passion of mine and for a number of years I have raced a
                                <a href=\"https://phantomclass.org.uk\">Phantom dinghy</a>.
    </p>
        <p>
        This is a fast single handed hiking dinghy. It's a real handful and has a huge grin
    factor on a reach. It's also seems to have a fairly vindictive nature and has made
    several attempts to drown and generally injure me. Definitely demands respect and seems
    to get jealous easily when time and attention are lacking ...
        </p>
        </div>
        </div>
        <div class=\"row\">
        <div class=\"col centered\">
        <h1>Cycling</h1>
        <p>
        As a teen I joined my father and brother riding bikes. I raced in a few different
    disciplines (grass, time-trial and later road-racing) and have also ridden some interesting
    mountain passes.
        </p>
        </div>
        <div class=\"col centered\">

        <img src=\"/static/images/cycling_last_ramp_galibier.png\" alt=\"cycling up the last ramp of the galibier\">
        </div>
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

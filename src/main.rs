extern crate env_logger;
extern crate warp;
extern crate yarte;

use warp::{http::status::StatusCode, http::Response, Filter};
use yarte::Template;
use std::env;

mod sitecontent;

fn main() {
    env_logger::init();

    let with_html = warp::reply::with::header("Content-Type", "text/html");

    // GET /
    // This just replies with the base index page and includes the basic about information.
    let index = warp::any()
        .map(|| -> String { get_index() })
        .with(&with_html);

    // GET /static/..
    // This serves the static content directly from the ../static/ directory.
    let web_static = warp::path("static").and(warp::fs::dir("./static/"));

    // GET /favicon.ico
    // return the favicon from the static location.
    let favicon = warp::path("favicon.ico").and(warp::fs::file("./static/favicon/favicon.ico"));

    // GET /sitemap.xml
    // return the sitemap from the static location.
    let sitemap = warp::path("sitemap.xml").and(warp::fs::file("./static/sitemap.xml"));

    // GET /{page_name}
    // This uses the base template and populates it with the content pertinent to the
    // requested page.
    let pages = warp::any()
        .and(warp::path::param())
        .map(|page_name: String| -> warp::http::Response<String> {
            let content_resp = get_content(page_name);
            match content_resp {
                Some(content) => Response::builder().body(content).unwrap(),
                None => Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(String::from("not found"))
                    .unwrap(),
            }
        })
        .with(&with_html);

    let routes = web_static.or(favicon).or(sitemap).or(pages).or(index);

    let listen_port = env::var("PORT");
    match listen_port {
        Ok(suggested_port) => {
            println!("listening on {}", &suggested_port);
            let port: u16 = suggested_port.parse().unwrap();
            warp::serve(routes).run(([0, 0, 0, 0], port));
        }
        Err(_) => {
            println!("listening on 8081");
            warp::serve(routes).run(([0, 0, 0, 0], 8081));
        }
    }
}

#[derive(Template)]
#[template(path = "index.hbs")]
struct IndexTemplate {
    body: String,
}

/// Just return the basic template populated with about information to give a nice landing page.
fn get_index() -> String {
    let page_body = sitecontent::get_content(String::from("about"));
    match page_body {
        Some(page_content) => {
            let root_template = IndexTemplate { body: page_content };
            root_template.call().unwrap()
        }
        None => String::from("oops"),
    }
}

/// Use the `page_name` to provide the appropriate content to inject into the base template.
fn get_content(page_name: String) -> Option<String> {
    if page_name.len() > 0 {
        let page_body = sitecontent::get_content(String::from(page_name.clone()));
        match page_body {
            Some(page_content) => {
                let page_template = IndexTemplate { body: page_content };
                Some(page_template.call().unwrap())
            }
            None => {
                println!("couldn't get page_name: {}", page_name);
                None
            }
        }
    } else {
        None
    }
}

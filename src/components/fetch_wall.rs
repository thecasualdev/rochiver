use std::{io::{self, Write}, thread::{sleep}, time::Duration, u64};

use serde::Deserialize;
use ureq::Error;

use crate::{rprint, rprintln};


#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
struct WallPosts {
    previous_page_cursor: Option<String>,
    next_page_cursor: Option<String>,
    data: Vec<Comment>
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
struct Comment {
    id: u64,
    poster: Option<Poster>,
    body: String,
    created: String,
    updated: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
struct Poster {
    user: User,
    role: Role,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
struct User {
    has_verified_badge: bool,
    user_id: u64,
    username: String,
    display_name: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
struct Role {
    id: u64,
    name: String,
    rank: u32,
}

pub fn fetch(group_id: &str) {

    let group_id: &str = group_id;
    let cookie = get_auth();

    let mut next_page_cursor: Option<String> = None;
    let mut ratelimit_reset: u64 = 60;

    loop {

        let response;

        let url = match &next_page_cursor {
            Some(cursor) => format!(
                "https://groups.roblox.com/v2/groups/{}/wall/posts?limit=10&sortOrder=Desc&cursor={}",
                group_id, cursor
            ),
            None => format!(
                "https://groups.roblox.com/v2/groups/{}/wall/posts?limit=10&sortOrder=Desc",
                group_id
            ),
        };

        response = ureq::get(&url)
            .header("accept", "application/json")
            .header("cookie", cookie.clone())
            .call();

        match response {
            Ok(mut rep) => {
                
                // for (header, value) in rep.headers() {
                    
                //     if header == "x-ratelimit-reset" {
                //         ratelimit_reset = match value.to_str() {
                //             Ok(s) => {
                //                 match s.parse::<u64>() {
                //                     Ok(parsed) => parsed,
                //                     Err(_) => 60
                //                 }
                //             },
                //             Err(_) => 60
                //         }
                //     }

                // }

                match rep.body_mut().read_json::<WallPosts>() {

                    Ok(json) => {

                        if let Some(cursor) = json.next_page_cursor {
                            rprintln!(
                                "{}",
                                format!(
                                    "{} {}",
                                    "[CURSOR]".bright_yellow(),
                                    cursor.green()

                                ).blue()
                            );
                            next_page_cursor = Some(cursor);
                        } else {
                            break;
                        }

                    },
                    Err(e) => {
                        rprintln!(
                            "{}",
                            format!("[ERROR] {e}").red()
                        );
                        break;
                    }

                }

            },
            Err(Error::StatusCode(code)) => {
                
                if code == 429 {

                    rprintln!("{}",
                        format!(
                            "[!!!!!!] RATE LIMIT REACHED, RESTING THREAD FOR {ratelimit_reset} SECONDS"
                        ).red()
                    );

                    sleep(Duration::from_secs(ratelimit_reset));

                    rprintln!("{}",
                        "[!!!!!!] WAKING THREAD".green()
                    );
                    
                    ratelimit_reset = 60;

                    continue;

                }

                break;

            }
            Err(e) => {
                rprintln!(
                   "{}",
                    format!("[ERROR] {e}").red()
                );
                break;
            }
        }

    }

}

fn get_auth() -> String {

    let mut cookie = String::new();

    rprint!(
        "{}",
        format!(
            "Z"
        ).red()
    );

    io::stdout()
        .flush()
        .unwrap();
    io::stdin()
        .read_line(&mut cookie)
        .expect("Rahhh!! Somethin broke");

    return cookie.trim().to_owned();

}
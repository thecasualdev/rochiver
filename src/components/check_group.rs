use crate::rprintln;

use serde::Deserialize;
use ureq::Error;

#[derive(Deserialize, Debug)]
struct Group {
    #[allow(dead_code)]
    id: u64,
    name: String,
}

#[derive(Deserialize)]
struct GroupsResponse {
    data: Vec<Group>
}

pub fn verify(group_id: &str) -> bool  {

    let url = format!("https://groups.roblox.com/v2/groups?groupIds={}", group_id);
    match ureq::get(&url).header("accept", "application/json").call() {
        Ok(mut response) => {
            match response.body_mut().read_json::<GroupsResponse>() {
                Ok(json) => {
                    
                    if let Some(group) = json.data.first() {

                        rprintln!(
                            "{}",
                            format!(
                                "Successfully verified {}!",
                                group.name.green()
                            ).blue()
                        );

                        return true;
                    } else {

                        rprintln!(
                            "{}",
                            format!("Could not verify group.").red()
                        );

                        return false;
                    }

                },
                Err(e) => {
                    rprintln!(
                        "{}",
                        format!(
                            "[ERROR] could not verify group, {e}"
                        ).red()
                    );
                    return false;
                }
            }
        },
        Err(Error::StatusCode(code)) => {
            
            if code == 429 {
                rprintln!(
                    "{}",
                    format!(
                        "[ERROR] : [{code}] Too many requests."
                    ).red()
                );
            } else {
                rprintln!(
                    "{}",
                    format!(
                        "[ERROR] could not verify group, got {code}"
                    ).red()
                );
            }   

            return false;
        }
        Err(e) => {
            rprintln!(
                "{}",
                format!(
                    "[ERROR] could not verify group, {e}"
                ).red()
            );
            return false;
        }
    }

}
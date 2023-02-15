use anyhow::Context;
use chrono::Local;
use clap::ValueEnum;
use reqwest::Response;
use strum::Display;

use crate::{error_trace, Payload, WorkingInfo, WorkingStatus};

#[derive(ValueEnum, Debug, Clone, Copy, Display, PartialEq, Eq)]
#[strum(serialize_all = "camelCase")]
pub enum Methods {
    Gm,
    Gn,
    #[strum(serialize = "rest")]
    Afk,
    #[strum(serialize = "rest")]
    Back,
    #[strum(serialize = "getInfo")]
    Info,
    Auth,
    Refresh,
}

impl Methods {
    pub fn print_accepted_message(&self) {
        match *self {
            Methods::Gm => println!(" Ok, now prepare to start your time..."),
            Methods::Gn => println!(" Ok, now prepare to finish your time..."),
            Methods::Afk => println!(" Ok, now prepare to stop your time..."),
            Methods::Back => println!(" Ok, now prepare to restart your time..."),
            Methods::Info => println!(" Ok, now fetching your information..."),
            _ => (),
        }
    }

    pub async fn handle_response(&self, res: Response) -> anyhow::Result<()> {
        tracing::debug!("Response: {:#?}", res);

        let res_str = res
            .text()
            .await
            .with_context(|| error_trace!("Response does not have body."))?;

        let payload: Payload = serde_json::from_str(&res_str).with_context(|| {
            error_trace!("Cannot deserialize response body from: {:?}", res_str)
        })?;

        let working_info = WorkingInfo::from(payload.response.result);

        println!();
        println!("{working_info}");

        match *self {
            Methods::Afk => {
                if *working_info.status() == WorkingStatus::Working {
                    tracing::warn!(" You called afk but still on working!");
                }
            }
            Methods::Back => {
                if *working_info.status() == WorkingStatus::Breaking {
                    tracing::warn!(" You called back but still on breaking!");
                }
            }
            _ => (),
        }

        Ok(())
    }

    // TODO: Add a message when forgot time stamped
    pub fn print_result_message(&self) {
        match *self {
            Methods::Gm => {
                println!(" Now you can start your work! Good luck!");
                println!();
                println!(" Current Time: {}", now());
                println!(" You can confirm your time on Google Spread Sheet:");
                println!("   https://docs.google.com/spreadsheets/d/1BSRnh5MU6OIW9eFAQxgS2KLC5nxQYMQzuQGqX65kqFI/edit#gid=1849163114")
            }
            Methods::Gn => {
                println!(" Now your work ended! Nice job ! ;-)");
                println!();
                println!(" Current Time: {}", now());
                println!(" You can confirm your time on Google Spread Sheet:");
                println!("   https://docs.google.com/spreadsheets/d/1BSRnh5MU6OIW9eFAQxgS2KLC5nxQYMQzuQGqX65kqFI/edit#gid=1849163114")
            }
            Methods::Afk => {
                println!(" Now you can go to out! Hava a nice break :-)");
                println!();
                println!(" Current Time: {}", now());
                println!(" You can confirm your time on Google Spread Sheet:");
                println!("   https://docs.google.com/spreadsheets/d/1BSRnh5MU6OIW9eFAQxgS2KLC5nxQYMQzuQGqX65kqFI/edit#gid=1849163114")
            }
            Methods::Back => {
                println!(" Now you can back to work! Good luck!");
                println!();
                println!(" Current Time: {}", now());
                println!(" You can confirm your time on Google Spread Sheet:");
                println!("   https://docs.google.com/spreadsheets/d/1BSRnh5MU6OIW9eFAQxgS2KLC5nxQYMQzuQGqX65kqFI/edit#gid=1849163114")
            }
            Methods::Info => {
                println!(" Current Time: {}", now());
                println!(" You can confirm your time on Google Spread Sheet:");
                println!("   https://docs.google.com/spreadsheets/d/1BSRnh5MU6OIW9eFAQxgS2KLC5nxQYMQzuQGqX65kqFI/edit#gid=1849163114")
            }
            _ => (),
        }
    }
}

fn now() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_string_test() {
        assert_eq!(Methods::Gm.to_string(), "gm");
        assert_eq!(Methods::Gn.to_string(), "gn");
        assert_eq!(Methods::Afk.to_string(), "rest");
        assert_eq!(Methods::Back.to_string(), "rest");
    }
}

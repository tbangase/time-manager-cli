use clap::ValueEnum;
use strum::Display;

#[derive(ValueEnum, Debug, Clone, Copy, Display)]
#[strum(serialize_all = "camelCase")]
pub enum Methods {
    Gm,
    Gn,
    #[strum(serialize = "rest")]
    Afk,
    #[strum(serialize = "rest")]
    Back,
}

impl Methods {
    pub fn print_accepted_message(&self) {
        match *self {
            Methods::Gm => println!(" Ok, now prepare to start your time..."),
            Methods::Gn => println!(" Ok, now prepare to finish your time..."),
            Methods::Afk => println!(" Ok, now prepare to stop your time..."),
            Methods::Back => println!(" Ok, now prepare to restart your time..."),
        }
    }

    // TODO: Add a message when forgot time stamped
    pub fn print_result_message(&self) {
        match *self {
            Methods::Gm => {
                println!(" Now you can start your work! Good luck!");
                println!(" You can confirm your time on Google Spread Sheet:");
                println!("   https://docs.google.com/spreadsheets/d/1BSRnh5MU6OIW9eFAQxgS2KLC5nxQYMQzuQGqX65kqFI/edit#gid=1849163114")
            }
            Methods::Gn => {
                println!(" Now your work ended! Nice job ! ;-)");
                println!(" You can confirm your time on Google Spread Sheet:");
                println!("   https://docs.google.com/spreadsheets/d/1BSRnh5MU6OIW9eFAQxgS2KLC5nxQYMQzuQGqX65kqFI/edit#gid=1849163114")
            }
            Methods::Afk => {
                println!(" Now you can go to out! Hava a nice break :-)");
                println!(" You can confirm your time on Google Spread Sheet:");
                println!("   https://docs.google.com/spreadsheets/d/1BSRnh5MU6OIW9eFAQxgS2KLC5nxQYMQzuQGqX65kqFI/edit#gid=1849163114")
            }
            Methods::Back => {
                println!(" Now you can back to work! Good luck!");
                println!(" You can confirm your time on Google Spread Sheet:");
                println!("   https://docs.google.com/spreadsheets/d/1BSRnh5MU6OIW9eFAQxgS2KLC5nxQYMQzuQGqX65kqFI/edit#gid=1849163114")
            }
        }
    }
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

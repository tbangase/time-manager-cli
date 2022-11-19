use clap::Parser;
use derive_getters::Getters;
use reqwest::Client;
use time_manager::SCRIPT_ID;
use time_manager::{get_access_token, AppScriptRequest, Methods};

#[derive(Parser, Debug, Getters)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Method you want to use
    // #[arg(short, long, value_enum)]
    #[arg(value_enum)]
    method: Methods,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let access_token = get_access_token().await?;

    let args = Args::parse();

    print_accepted_message(args.method());

    let client = Client::new();
    let body = AppScriptRequest::builder()
        .function(args.method().to_string())
        .build();

    tracing::debug!("AppScript Request: {:#?}", body);

    let req = client
        .post(format!(
            "https://script.googleapis.com/v1/scripts/{SCRIPT_ID}:run"
        ))
        .header("Authorization", format!("Bearer {access_token}"))
        .body(serde_json::to_string(&body)?)
        .build()?;

    tracing::debug!("Request Information: {:#?}", req);
    tracing::debug!("Request Body: {:#?}", req.body().unwrap());

    let res = client.execute(req).await?;

    tracing::debug!("Response: {:#?}", res);
    tracing::debug!("Response Body: {}", res.text().await?);

    print_result_message(args.method());

    Ok(())
}

fn print_accepted_message(method: &Methods) {
    match method {
        Methods::Gm => println!(" Ok, now prepare to start your time..."),
        Methods::Gn => println!(" Ok, now prepare to finish your time..."),
        Methods::Afk => println!(" Ok, now prepare to stop your time..."),
        Methods::Back => println!(" Ok, now prepare to restart your time..."),
    }
}

// TODO: Add a message when forgot time stamped
fn print_result_message(method: &Methods) {
    match method {
        Methods::Gm => println!(" Now you can start your work! Good luck!"),
        Methods::Gn => println!(" Now your work ended! Nice job ! ;-)"),
        Methods::Afk => println!(" Now you can go to out! Hava a nice break :-)"),
        Methods::Back => println!(" Now you can back to work! Good luck!"),
    }
}

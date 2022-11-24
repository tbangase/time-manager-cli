use clap::Parser;
use derive_getters::Getters;
use reqwest::Client;
use time_manager::SCRIPT_ID;
use time_manager::{get_access_token, AppScriptRequest, Methods};

#[derive(Parser, Debug, Getters)]
#[command(author, version, about, long_about = None)]
struct Args {
    // TODO: Add a way to return remain time
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

    args.method().print_accepted_message();

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

    args.method().print_result_message();

    Ok(())
}

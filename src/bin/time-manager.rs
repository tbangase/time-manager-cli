use clap::Parser;
use derive_getters::Getters;
use reqwest::{Client, Response};
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

    let args = Args::parse();
    let method = args.method();

    method.print_accepted_message();

    if method == &Methods::Auth {
        unimplemented!();
    } else {
        let res = call_gas(*method).await?;
        method.handle_response(res).await?;
    }

    method.print_result_message();

    Ok(())
}

async fn call_gas(method: Methods) -> anyhow::Result<Response> {
    let access_token = get_access_token().await?;

    let client = Client::new();
    let body = AppScriptRequest::builder()
        .function(method.to_string())
        .build();

    tracing::debug!("AppScript Request: {:#?}", body);

    // Send request to Google AppScript
    let req = client
        .post(format!(
            "https://script.googleapis.com/v1/scripts/{SCRIPT_ID}:run"
        ))
        .header("Authorization", format!("Bearer {access_token}"))
        .body(serde_json::to_string(&body)?)
        .build()?;

    tracing::debug!("Request Information: {:#?}", req);
    tracing::debug!("Request Body: {:#?}", req.body().unwrap());

    Ok(client.execute(req).await?)
}

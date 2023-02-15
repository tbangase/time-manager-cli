use anyhow::Context;
use clap::Parser;
use derive_getters::Getters;
use reqwest::{Client, Response};
use time_manager::{
    error_trace, get_access_token, overwrite_refresh_token, AppScriptRequest, Methods,
};
use time_manager::{OAUTH_CLIENT_ID, REDIRECT_URI, SCOPE, SCRIPT_ID};

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

    match method {
        Methods::Auth => {
            let url = format!("https://accounts.google.com/o/oauth2/v2/auth?response_type=code&client_id={OAUTH_CLIENT_ID}&redirect_uri={REDIRECT_URI}&scope={SCOPE}&access_type=offline");
            open::that(url).with_context(|| error_trace!("Fail to open auth url"))?;
        }
        Methods::Refresh => {
            overwrite_refresh_token().await?;
        }
        _ => {
            let res = call_gas(*method).await?;
            method.handle_response(res).await?;
        }
    };
    // if method == &Methods::Auth {
    //     let url = format!("https://accounts.google.com/o/oauth2/v2/auth?response_type=code&client_id={OAUTH_CLIENT_ID}&redirect_uri={REDIRECT_URI}&scope={SCOPE}&access_type=offline");
    //     open::that(url).with_context(|| error_trace!("Fail to open auth url"))?;
    // } else if method == &Methods::Refresh {
    //     unimplemented!();
    // } else {
    //     let res = call_gas(*method).await?;
    //     method.handle_response(res).await?;
    // }

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

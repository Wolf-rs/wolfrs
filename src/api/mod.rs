// TODO: Eventually spin off the API functionality into it's own, properly documented libre crate
pub mod admin;
pub mod comment;
pub mod community;
pub mod etc;
pub mod federation;
pub mod modlog;
pub mod posts;
pub mod private_message;
pub mod search;
pub mod site;
pub mod structs;
pub mod user;

use anyhow::{anyhow, Result};
use leptos::{Scope, Serializable};

use crate::api::structs::ApiUrlConstructor;
use crate::components::instance::*;

// This function is used to construct the correct url to use for the api
pub fn api_url_builder<T: Serializable + serde::Serialize>(
    cx: Scope,
    url_constructor: ApiUrlConstructor,
    form: T,
) -> String {
    let params_str =
        serde_html_form::to_string(&form).unwrap_or(url_constructor.endpoint.to_string());
    format!(
        "{}/api/{}/{}?{}",
        get_instance_details().unwrap().url,
        get_instance_details().unwrap().api_version,
        url_constructor.endpoint,
        params_str,
    )
}

// These API functions originate with the Leptos `hackernews_axum` example from the GitHub repo. The Lemy devs simply altered them to work with the Lemmy API, and that's what I have done as well.
// POST and PUT functionality may be built into these functions as well, or made their own seperate functions. Will determine later on
#[cfg(not(feature = "ssr"))]
pub async fn api_get<Response>(cx: Scope, path: &str) -> Result<Response>
where
    Response: Serializable,
{
    let abort_controller = web_sys::AbortController::new().ok();
    let abort_signal = abort_controller.as_ref().map(|a| a.signal());

    let json = gloo_net::http::Request::get(path)
        .abort_signal(abort_signal.as_ref())
        .send()
        .await?
        .text()
        .await?;

    // abort in-flight requests if the Scope is disposed
    // i.e., if we've navigated away from this page
    leptos::on_cleanup(cx, move || {
        if let Some(abort_controller) = abort_controller {
            abort_controller.abort()
        }
    });

    // This really isn't good... It has no error handling, but the way it was handled below kept returning false(?) Err even though the data was there
    Ok(Response::de(&json).unwrap())

    // Return the error response json as an error
    //Response::de(&json).map_err(|_| anyhow!(json.clone()))
}

#[cfg(feature = "ssr")]
pub async fn api_get<Response>(_cx: Scope, path: &str) -> Result<Response>
where
    Response: Serializable,
{
    let client = reqwest::Client::new();

    let json = client.get(path).send().await?.text().await?;

    // Use this to test JSON outputs before unwrapping in case of errors
    //println!("Test in mod.rs: {:#?}", json.clone());

    // This really isn't good... It has no error handling, but the way it was handled below kept returning false(?) Err even though the data was there
    Ok(Response::de(&json).unwrap())

    // Return the error response json as an error
    //Response::de(&json).map_err(|_| anyhow!(json.clone()))
}

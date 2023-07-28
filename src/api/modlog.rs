use anyhow::Result;
use leptos::Scope;

use crate::api::structs::*;
use crate::api::*;

// This is a helper function for api_get that then returns the GetModlogResponse from the API Result.
pub async fn get_mod_log(cx: Scope, endpoint: &str) -> Result<GetModlogResponse> {
    api_get::<GetModlogResponse>(cx, endpoint).await
}

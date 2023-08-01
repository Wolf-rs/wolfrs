use anyhow::Result;
use leptos::Scope;

use crate::api::structs::*;
use crate::api::*;

// This is a helper function for api_get that then returns the GetCommunitiesResponse from the API Result.
// This is also how the Lemmy devs do it as well, though it feels a bit unnecessary. I feel like this can be integrated into the api_get function itself, but that can be done later on as this is at least working.
pub async fn list_communities(cx: Scope, endpoint: &str) -> Result<ListCommunitiesResponse> {
    api_get::<ListCommunitiesResponse>(cx, endpoint).await
}

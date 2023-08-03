use leptos::*;

use crate::api::structs::*;
use crate::api::*;

// This is a helper function for api_get that then returns the GetPostsResponse from the API Result.
// This is also how the Lemmy devs do it as well, though it feels a bit unnecessary. I feel like this can be integrated into the api_get function itself, but that can be done later on as this is at least working.
pub async fn get_person_details(cx: Scope, endpoint: &str) -> Result<GetPersonDetailsResponse> {
    api_get::<GetPersonDetailsResponse>(cx, endpoint).await
}

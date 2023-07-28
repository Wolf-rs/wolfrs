use anyhow::Result;
use leptos::Scope;

use crate::api::structs::*;
use crate::api::*;

// This is a helper function for api_get that then returns the GetCommentsReponse from the API Result.
pub async fn get_comments(cx: Scope, endpoint: &str) -> Result<GetCommentsResponse> {
    api_get::<GetCommentsResponse>(cx, endpoint).await
}

// Not actually used yet, simply added for future use
pub async fn get_comment(cx: Scope, endpoint: &str) -> Result<CommentResponse> {
    api_get::<CommentResponse>(cx, endpoint).await
}

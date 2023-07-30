use anyhow::Result;
use leptos::Scope;

use crate::api::structs::*;
use crate::api::*;

// This is a helper function for api_get that then returns the GedFederatedInstancesResponse from the API Result.
pub async fn get_federated_instances(
    cx: Scope,
    endpoint: &str,
) -> Result<GetFederatedInstancesResponse> {
    api_get::<GetFederatedInstancesResponse>(cx, endpoint).await
}

use reqwest::Client;

use crate::api::structs::*;
use crate::api::*;

// pub async fn login() -> Result<LoginResponse, reqwest::Error> {
//     let url = api_url_constructor(api_endpoints::PostEndpoint::LOGIN, "");
//     let response = match Client::new().post(&url).send().await {
//         Ok(response) => response,
//         Err(e) => return Err(e),
//     };

//     Ok(match response.json::<LoginResponse>().await {
//         Ok(it) => it,
//         Err(err) => return Err(err),
//     })
// }

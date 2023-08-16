use leptos::*;
use leptos_router::*;

use crate::api::structs::router_endpoints;
use crate::api::structs::*;
use crate::api::user::get_person_details;
use crate::api::*;
use crate::components::{feed::Feed, sidecards::user::Sidecard};

// TODO - user.rs:
// Sidecard component still needs to be built
// Real styling with Bootstrap
// Ensure mobile layout works as expected

// The main page for viewing a user and their feed.

#[component]
pub fn User(cx: Scope) -> impl IntoView {
    let query = use_query_map(cx);
    let page = move || {
        query
            .with(|q| q.get("page").and_then(|page| page.parse::<i32>().ok()))
            .unwrap_or(1)
    };

    let params = use_params_map(cx);
    let username = move || {
        params
            .with(|params| params.get("username").cloned())
            .unwrap_or_default()
            .parse::<String>()
            .unwrap()
    };

    let sidebar = create_resource(cx, page, move |page| async move {
        // This constructs the proper API URL for GetPosts or GetPersonDetails
        let url_constructor = ApiUrlConstructor {
            endpoint: api_endpoints::GetEndpoint::GET_PERSON_DETAILS.to_string(),
            id: None,
            params: None,
        };

        let get_form = GetPersonDetails {
            auth: None,
            community_id: None,
            limit: None,
            page: None,
            person_id: None,
            saved_only: None,
            sort: None,
            username: Some(username()),
        };

        get_person_details(cx, &api_url_builder(cx, url_constructor, get_form))
            .await
            .ok()
    });

    view! { cx,
        <div class="container overflow-hidden">
            <div class="row gx-4">
                // Feed Column
                <div class="col-md-9">
                    <br/>
                    <Transition fallback=move || {
                        // Handles the loading screen while waiting for a reply from the API
                        view! { cx,
                            <div class="d-flex align-items-center">
                                <h1>
                                    Loading...
                                </h1>
                                <div
                                    class="spinner-grow ms-auto"
                                    role="status"
                                    aria-hidden="true"
                                ></div>
                            </div>
                        }
                    }>
                        <Feed endpoint=router_endpoints::RouterEndpoint::USER/>
                    </Transition>
                </div>

                // Sidecard Column
                <div class="col-12 col-md-3">
                    <br/>
                    <Transition fallback=move || {
                        // Handles the loading screen while waiting for a reply from the API
                        view! { cx,
                            <div class="d-flex align-items-center">
                                <h1>
                                    Loading...
                                </h1>
                                <div
                                    class="spinner-grow ms-auto"
                                    role="status"
                                    aria-hidden="true"
                                ></div>
                            </div>
                        }
                    }>
                        <Sidecard sidebar=sidebar/>
                    </Transition>

                </div>
            </div>
        </div>
    }
}

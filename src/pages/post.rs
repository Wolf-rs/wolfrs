use leptos::*;
use leptos_router::*;

use crate::api::posts::get_post;
use crate::api::structs::*;
use crate::api::*;
use crate::components::{post_view::PostView, sidecards::post::Sidecard};

// TODO - post.rs:
// Sidecard component still needs to be built
// Real styling with Bootstrap
// Ensure mobile layout works as expected

// The main page for viewing a post and its comments.

#[component]
pub fn Post(cx: Scope) -> impl IntoView {
    let params = use_params_map(cx);
    let id = move || {
        params
            .with(|params| params.get("id").cloned())
            .unwrap_or_default()
            .parse::<i32>()
            .unwrap()
    };

    // Variable that holds the returned GetPostResponse from the API
    let post = create_resource(cx, id, move |id| async move {
        // This constructs the proper API URL for GetPosts
        let url_constructor = ApiUrlConstructor {
            endpoint: api_endpoints::GetEndpoint::GET_POST.to_string(),
            id: Some(id),
            params: None,
        };

        // This assembles the GetPost request form
        let get_form = GetPost {
            auth: None,
            comment_id: None,
            id: Some(id),
        };

        // This is where the API is called for GetPost and the GetPostResponse is returned
        get_post(cx, &api_url_builder(cx, url_constructor, get_form))
            .await
            .ok()
    });

    view! { cx,
        <div class="container overflow-hidden">
            <div class="row gx-4">
                // Feed Column
                <div class="col-md-9">
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
                        <PostView post=post/>
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
                        <Sidecard sidebar=post/>
                    </Transition>
                </div>
            </div>
        </div>
    }
}

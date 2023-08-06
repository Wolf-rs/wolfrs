use leptos::*;
use leptos_router::*;
use markdown::*;

use crate::api::posts::get_post;
use crate::api::structs::*;
use crate::api::*;
use crate::components::comments::Comments;
use crate::components::feed::FeedItem;

// TODO - post_view.rs:
// Handle when there is no actual post body for an external link
// Fix clicking on the PostItem title taking the page to an unreachable route
// onclick functionality for voting, favoriting, crossposting, and reporting
// Better handling for mobile layouts
// Implement community avatars for posts in sensible manner
// Finish fleshing out PostItem for stuff like language, edited status, date, etc
// Finish implementing Markdown styling for blockquotes and superscripts (Apparently that's not a gfm thing?)
// Improve overall styling for the post component box itself

// The component box that holds the post body and contents itself
#[component]
pub fn PostView(cx: Scope) -> impl IntoView {
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

    let err_msg = "Error loading this post: ";

    view! { cx,
        <div class="container overflow-hidden">
            <Transition fallback=move || {
                // Handles the loading screen while waiting for a reply from the API
                view! { cx,
                    <div class="d-flex align-items-center">
                        <h1>
                            Loading...
                        </h1>
                        <div class="spinner-grow ms-auto" role="status" aria-hidden="true"></div>
                    </div>
                }
            }>
                {move || {
                    post.read(cx)
                        .map(|res| match res {
                            None => {
                                view! { cx, <div>{format!("{err_msg}")}</div> }
                            }
                            Some(res) => {
                                let post_body = match res.post_view.post.body.clone() {
                                    Some(text) => {
                                        markdown::to_html_with_options(
                                                text.as_str(),
                                                &Options::gfm(),
                                            )
                                            .unwrap()
                                    }
                                    None => "".to_string(),
                                };

                                view! { cx,
                                    <div>
                                        <div>
                                            <br/>
                                            <div class="card">
                                                <div class="card-body">
                                                    <div class="row gx-4">
                                                        <FeedItem post_view=leptos::MaybeSignal::Static(
                                                            res.post_view.clone(),
                                                        )/>
                                                    </div>
                                                    <Suspense fallback=move || {
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
                                                        // This is where the Markdown content of the post is rendered.
                                                        <div inner_html=post_body.clone()></div>
                                                    </Suspense>
                                                </div>
                                            </div>
                                        </div>
                                        <br/>
                                        <Comments post_info=res.post_view.clone()/>
                                    </div>
                                }
                            }
                        })
                }}

            </Transition>
        </div>
    }
}

use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use markdown::*;

use crate::api::structs::*;
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
pub fn PostView(cx: Scope, post: Resource<i32, Option<GetPostResponse>>) -> impl IntoView {
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
                                                <div class="card-header pb-0">
                                                    <FeedItem post_view=leptos::MaybeSignal::Static(
                                                        res.post_view.clone(),
                                                    )/>
                                                </div>
                                                <div class="card-body">
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
                                                        // This is where the Markdown content of the post is rendered.
                                                        <div class="markdown" inner_html=post_body.clone()></div>
                                                    </Transition>
                                                </div>
                                            </div>
                                        </div>
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
                                            // This is where the Markdown content of the post is rendered.
                                            <Comments post_info=res.post_view.clone()/>
                                        </Transition>
                                    </div>
                                }
                            }
                        })
                }}

            </Transition>
        </div>
    }
}

use leptos::*;
use leptos_router::*;

use crate::api::comment::*;
use crate::api::structs::*;
use crate::api::*;

// TODO - comments.rs:
// Add functionality for pulling in replies (IDK how to do this...)
// onclick functionality for voting, favoriting, crossposting, and reporting
// Better handling for mobile layouts
// Build media popups for images(?)
// Build actual styling for comments
// Implement a nesting system for comments
// Finish styling comments component box itself.

// The component box for the comments on a post
#[component]
pub fn Comments(cx: Scope, post_info: PostView) -> impl IntoView {
    let params = use_params_map(cx);
    let id = move || {
        params
            .with(|params| params.get("id").cloned())
            .unwrap()
            .parse::<i32>()
            .unwrap()
    };

    let local_or_external = match !post_info.community.local {
        true => {
            let url_regex = regex::Regex::new(r"https://(.*)/c/(.*)").unwrap();

            let external_community_link = match url_regex.captures(&post_info.community.actor_id) {
                Some(external_community_link) => external_community_link,
                None => {
                    println!("url regex error: {:?}", &post_info.community.actor_id);
                    url_regex
                        .captures(&post_info.community.actor_id)
                        .unwrap_or_else(|| url_regex.captures("https://error.com/c/error").unwrap())
                }
            };
            format!(
                "{}@{}",
                post_info.community.name.clone(),
                &external_community_link[1]
            )
        }
        false => post_info.community.name.clone(),
    };

    let updater = create_memo(cx, move |_| (id(), local_or_external.clone()));

    // Variable that holds the returned GetCommentsResponse from the API
    let comments = create_resource(
        cx,
        move || updater,
        move |updater| async move {
            // This constructs the proper API URL for GetPosts
            let url_constructor = ApiUrlConstructor {
                endpoint: api_endpoints::GetEndpoint::GET_COMMENTS.to_string(),
                id: Some(updater.get().0),
                params: None,
            };

            // This assembles the GetComments request form
            let get_form = GetComments {
                auth: None,
                community_id: None,
                community_name: Some(updater.get().1),
                limit: None,
                max_depth: None,
                page: None,
                parent_id: None,
                post_id: Some(updater.get().0),
                saved_only: None,
                sort: None,
                type_: None,
            };

            // This is where the API is called for GetComments and the GetCommentsResponse is returned
            get_comments(cx, &api_url_builder(cx, url_constructor, get_form))
                .await
                .ok()
        },
    );

    let err_msg = "Error loading these comments: ";

    view! { cx,
        <div>
        <Transition fallback=move || {
            // Handles the loading screen while waiting for a reply from the API
            view! { cx,
                <div class="d-flex align-items-center">
                    <h1>Loading...</h1>
                    <div class="spinner-grow ms-auto" role="status" aria-hidden="true"></div>
                </div>
             }
        }>
            {move || {
                comments
                    .read(cx)
                    .map(|res| match res {
                        None => {
                            view! { cx, <div>{format!("{err_msg}{:?}", comments.read(cx))}</div> }
                        }
                        Some(res) => {
                            view! { cx,
                                <div>
                                    <CommentsList comments=res.comments.into()/>
                                </div>
                            }
                        }
                    })
            }}
        </Transition>
        </div>
    }
}

// The component for the comments list
// May need another component for holding a comment and its replies, TBD...
#[component]
fn CommentsList(cx: Scope, comments: MaybeSignal<Vec<CommentView>>) -> impl IntoView {
    view! { cx,

      {comments.get().into_iter()
        .map(|comment| view! { cx, <CommentItem comment_item=leptos::MaybeSignal::Static(comment) />})
        .collect_view(cx)}

    }
}

// The component for a single comment
#[component]
pub fn CommentItem(cx: Scope, comment_item: MaybeSignal<CommentView>) -> impl IntoView {
    let comment = comment_item.get();

    view! { cx,
        <div>
            <div class="card">
                <div class="card-header">
                    {comment.creator.name}
                </div>
                <div class="card-body">
                    {comment.comment.content}
                </div>
            </div>
        </div>
    }
}

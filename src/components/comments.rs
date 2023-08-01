use leptos::*;
use leptos_router::*;

use crate::api::comment::*;
use crate::api::structs::*;
use crate::api::*;

// TODO - comments.rs:
// onclick functionality for voting, favoriting, crossposting, and reporting
// Better handling for mobile layouts
// Build media popups for images(?)
// Build actual styling for comments
// Implement a nesting system for comments
// Finish styling comments component box itself.

// The component box for the comments on a post
#[component]
pub fn Comments(cx: Scope) -> impl IntoView {
    let params = use_params_map(cx);
    let id = move || {
        params
            .with(|params| params.get("id").cloned())
            .unwrap()
            .parse::<i32>()
            .unwrap()
    };

    // Variable that holds the returned GetCommentsResponse from the API
    let comments = create_resource(cx, id, move |id| async move {
        // This constructs the proper API URL for GetPosts
        let url_constructor = ApiUrlConstructor {
            endpoint: api_endpoints::GetEndpoint::GET_COMMENTS.to_string(),
            id: Some(id),
            params: None,
        };

        // This assembles the GetComments request form
        let get_form = GetComments {
            auth: None,
            community_id: None,
            community_name: None,
            limit: None,
            max_depth: None,
            page: None,
            parent_id: None,
            post_id: Some(id),
            saved_only: None,
            sort: None,
            type_: None,
        };

        // This is where the API is called for GetComments and the GetCommentsResponse is returned
        get_comments(cx, &api_url_builder(cx, url_constructor, get_form))
            .await
            .ok()
    });

    let err_msg = "Error loading these comments: ";

    view! { cx,
        <div class="container overflow-hidden">
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
            {comment.comment.content}
            <br />
            "----------"
        </div>
    }
}

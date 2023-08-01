use leptos::*;
use leptos_router::*;
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use serde_json::Value;

use crate::api::posts::get_posts;
use crate::api::structs::*;
use crate::api::*;
use crate::components::pagination::Pagination;
// TODO - feed.rs:
// onclick functionality for voting, favoriting, crossposting, and reporting
// Better handling for mobile layouts, including possibly removing voting buttons on mobile
// Implement community avatars for posts in sensible manner
// Build media popups for images
// Implement URL preview images for external links (This seems to already be handled by the backend?)
// Finish fleshing out PostItem for stuff like language, edited status, date, etc

// The home page feed column that shows the Posts list
#[component]
pub fn Feed(cx: Scope) -> impl IntoView {
    let query = use_query_map(cx);
    let page = move || {
        query
            .with(|q| q.get("page").and_then(|page| page.parse::<i32>().ok()))
            .unwrap_or(1)
    };

    // Variable that holds the returned GetPostsResponse from the API
    let posts = create_resource(cx, page, move |page| async move {
        // This constructs the proper API URL for GetPosts
        let url_constructor = ApiUrlConstructor {
            endpoint: api_endpoints::GetEndpoint::GET_POSTS.to_string(),
            id: None,
            params: None,
        };

        // This assembles the GetPosts request form
        let get_form = GetPosts {
            auth: None,
            community_id: None,
            community_name: None,
            limit: Some(20),
            page: Some(page),
            post_id: None,
            saved_only: None,
            sort: None,
            type_: None,
        };

        // This is where the API is called for GetPosts and the GetPostsResponse is returned
        get_posts(cx, &api_url_builder(cx, url_constructor, get_form))
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
                            <h1>Loading...</h1>
                            <div class="spinner-grow ms-auto" role="status" aria-hidden="true"></div>
                        </div>
                     }
                }>
                {move || {
                    posts
                        .read(cx)
                        .map(|res| match res {
                            None => {
                                view! { cx, <div>{format!("{err_msg}")}</div> }
                            }
                            Some(res) => {
                                view! { cx,
                                    <div>
                                        <PostsList posts=res.posts.into()/>
                                    </div>
                                }
                            }
                        })
                }}
                </Transition>

                <Pagination />
            </div>

    }
}

// The Posts list containing the Post items
// Suggestion: Implement collapsing accordions for multiple posts in a row from the same community?
#[component]
fn PostsList(cx: Scope, posts: MaybeSignal<Vec<PostView>>) -> impl IntoView {
    view! { cx,

      {posts.get().into_iter()
        .map(|post| view! { cx, <PostItem post_view=leptos::MaybeSignal::Static(post) />})
        .collect_view(cx)}

    }
}

// The individual Post items, may need to be a seperate component if it grows too large...
#[component]
pub fn PostItem(cx: Scope, post_view: MaybeSignal<PostView>) -> impl IntoView {
    // These set the varaibles from the PostView struct to make insetion into the view easier
    let post = post_view.get();
    let post_link = format!("post/{}", post.post.id);
    let total_votes = post.counts.upvotes - post.counts.downvotes;
    // Checks to see if a post thumbnail exists, if it does not it setts it to a default image
    let post_thumbnail = match post.post.thumbnail_url {
        Some(_) => post.post.thumbnail_url,
        _ => Option::Some("/static/default_assets/comment.png".to_string()),
    };
    let post_title = post.post.name;
    let post_pin = post.post.featured_local;
    // Checks to see if a user has an avatar set, if not it assigns a default one
    let creator_avatar = match post.creator.avatar {
        Some(_) => post.creator.avatar,
        _ => Option::Some("/static/default_assets/default-profile.png".to_string()),
    };
    let creator_name = post.creator.name;
    // This needs a similar check as above, I still need to make a default placeholder for a community avatar
    let community_avatar = post.community.icon;
    // This needs to be handled better, it thinks that some local communities are external, possibly due to cross-posting
    let community_name = if post.post.local {
        post.community.name
    } else {
        format!("{}@{}", post.community.name, post.community.title)
    };
    let comment_count = post.counts.comments;

    view! { cx,
      <div class="row">

        // Voting Column
        <div class="col-sm-1">
            <div class="row">
                <i class="bi bi-caret-up text-center"></i>
            </div>
            <div class="row">
            <div class="text-center">
            {post.counts.upvotes - post.counts.downvotes}
            </div>
            </div>
            <div class="row">
            <i class="bi bi-caret-down text-center"></i>
            </div>
        </div>

        <div class="col-sm-11">
            <div class="row">
                <div class="col-sm-2">
                    <div style="">
                        <img src={post_thumbnail} width="75" height="75" class="rounded" alt="..." style="" />
                    </div>
                </div>
                <div class="col-sm-10">
                    <div class="row">
                        <h5>
                        <A href={post_link.clone()} class="link-offset-2 link-underline link-underline-opacity-0">
                        {post_title}
                        </A>
                        {if post_pin {
                            view! { cx, <i class="bi bi-pin-angle-fill"></i>}
                        } else {
                            view! { cx, <i></i>}
                        }}
                        </h5>
                    </div>
                    <div class="row">
                        <div class="col-sm-1">
                            <img src={creator_avatar} alt="mdo" width="32" height="32" class="rounded" />
                        </div>
                        <div class="col-sm-11">
                            <p>
                                {creator_name}" in "{community_name}
                            </p>
                        </div>
                    </div>
                    <div class="row">
                        <span>
                            <A href={post_link} class="link-secondary link-offset-2 link-underline link-underline-opacity-0"><i class="bi bi-chat-right-text"></i>" "{comment_count}" Comments"</A>
                            "   "
                            <i class="bi bi-bookmark-star text-secondary"></i>
                            "   "
                            <i class="bi bi-signpost-split text-secondary"></i>
                            "   "
                            <i class="bi bi-flag text-secondary"></i>
                        </span>


                    </div>
                </div>
            </div>
        </div>

        <hr />
      </div>
    }
}

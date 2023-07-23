use anyhow::{anyhow, Result};
use leptos::*;
use leptos_router::*;
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use serde_json::Value;

use crate::api::post::get_posts;
use crate::api::structs::*;
use crate::api::*;
use crate::components::*;

// The home page feed column that shows the Posts list
#[component]
pub fn Feed(cx: Scope) -> impl IntoView {
    let query = use_query_map(cx);
    let page = move || {
        query
            .with(|q| q.get("page").and_then(|page| page.parse::<i32>().ok()))
            .unwrap_or(1)
    };

    // Component signal to handle pagination
    let (page_count, set_count) = create_signal(cx, 0);

    // Variable that holds the returned GetPostsResponse from the API
    let posts = create_resource(cx, page, move |page| async move {
        // This constructs the proper API URL for GetPosts
        let url_constructor = ApiUrlConstructor {
            endpoint: api_endpoints::GetEndpoint::GET_POSTS.to_string(),
            params: "",
        };

        // This assembles the GetPosts request form
        let get_form = GetPosts {
            auth: None,
            community_id: None,
            community_name: None,
            limit: None,
            page: Some(page),
            post_id: None,
            saved_only: None,
            sort: None,
            type_: None,
        };

        // This is where the API is called for GetPosts and the GetPostsResponse is returned
        get_posts(cx, &api_url_constructor(cx, url_constructor, get_form))
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

                // Pagination for the feed. currently broken while trying to figure out proper signalling
                // <nav aria-label="Feed Page Navigation">
                //     <ul class="pagination justify-content-center">
                //         <li class="page-item"><A class="page-link" href={format!("?page={}", page_count)} on:click=move |_| {set_count.update(|n| *n -= 1);} >Previous</A></li>
                //         <li class="page-item"><A class="page-link" href={format!("?page={}", page_count)}>Page: {page}</A></li>
                //         <li class="page-item"><A class="page-link" href={format!("?page={}", page_count)} on:click=move |_| {set_count.update(|n| *n += 1);} >Next</A></li>
                //     </ul>
                // </nav>
            </div>

    }
}

// The Posts list containing the Post items
#[component]
pub fn PostsList(cx: Scope, posts: MaybeSignal<Vec<PostView>>) -> impl IntoView {
    view! { cx,
      <ul>
      {posts.get().into_iter()
        .map(|post| view! { cx, <PostItem post_view=leptos::MaybeSignal::Static(post) />})
        .collect_view(cx)}
      </ul>
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
    let community_name = post.community.name;
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
                                {creator_name}" @ "{community_name}
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

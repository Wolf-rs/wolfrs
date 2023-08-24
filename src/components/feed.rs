use leptos::*;
use leptos_router::*;

use crate::api::posts::get_posts;
use crate::api::structs::*;
use crate::api::user::get_person_details;
use crate::api::*;
use crate::components::pagination::Pagination;

// TODO - feed.rs:
// Improve support for KBin, which is currently somewhat... Broken
// onclick functionality for voting, favoriting, crossposting, and reporting
// Better handling for mobile layouts, including possibly removing voting buttons on mobile
// Finish fleshing out PostItem for stuff like language, edited status, date, etc

// The feed column that shows the Posts list, used for Home, Community, and User pages
#[component]
pub fn Feed(cx: Scope, endpoint: &'static str) -> impl IntoView {
    let query = use_query_map(cx);
    let page = move || {
        query
            .with(|q| q.get("page").and_then(|page| page.parse::<i32>().ok()))
            .unwrap_or(1)
    };

    let params = use_params_map(cx);
    let community_name = move || {
        params
            .with(|params| params.get("community_name").cloned())
            .unwrap_or_default()
            .parse::<String>()
            .unwrap()
    };

    let username = move || {
        params
            .with(|params| params.get("username").cloned())
            .unwrap_or_default()
            .parse::<String>()
            .unwrap()
    };

    // Creates the signals needed for handling the tab reactivity on the feed
    // Needs to be made to ignore/change the tabs when looking at a user or community page feed
    let (subscribed_active, set_subscribed_active) = create_signal(cx, false);
    let (local_active, set_local_active) = create_signal(cx, false);
    let (all_active, set_all_active) = create_signal(cx, true);
    let (active_tab, set_active_tab) = create_signal(cx, ListingType::All);
    let subscribed_disabled = true;

    let (invisible_listing, set_invisible_listing) = create_signal(cx, true);

    if endpoint == "home" {
        set_invisible_listing.update(|value| *value = false);
    }

    let (sorting, set_sorting) = create_signal(cx, SortType::Active);

    // Creates the derived signal from page and active_tab for the create_resource function below
    let updater = move || (page(), active_tab.get(), sorting.get());

    // Variable that holds the returned PostView from the API for either GetPostsResponse or GetPersonDetailsResponse
    let posts = create_resource(cx, updater, move |updater| async move {
        // This constructs the proper API URL for GetPosts or GetPersonDetails
        let url_constructor = match endpoint {
            "community" | "home" => ApiUrlConstructor {
                endpoint: api_endpoints::GetEndpoint::GET_POSTS.to_string(),
                id: None,
                params: None,
            },
            "user" => ApiUrlConstructor {
                endpoint: api_endpoints::GetEndpoint::GET_PERSON_DETAILS.to_string(),
                id: None,
                params: None,
            },
            &_ => unreachable!(),
        };

        if endpoint.ne("user") {
            // This assembles the GetPosts request form for either the home feed or community feed
            let get_form = match endpoint {
                "community" => GetPosts {
                    auth: None,
                    community_id: None,
                    community_name: Some(community_name()),
                    limit: Some(20),
                    page: Some(updater.0),
                    post_id: None,
                    saved_only: None,
                    sort: Some(updater.2),
                    type_: None,
                },
                "home" => GetPosts {
                    auth: None,
                    community_id: None,
                    community_name: None,
                    limit: Some(20),
                    page: Some(updater.0),
                    post_id: None,
                    saved_only: None,
                    sort: Some(updater.2),
                    type_: Some(updater.1),
                },
                &_ => unreachable!(),
            };
            // This is where the API is called for GetPosts and the GetPostsResponse is returned and converted to PostView
            return get_posts(cx, &api_url_builder(cx, url_constructor, get_form))
                .await
                .unwrap()
                .posts
                .into();
        }

        // This assembles the GetPersonDetails request form for the user feed
        let get_form = GetPersonDetails {
            auth: None,
            community_id: None,
            limit: Some(20),
            page: Some(updater.0),
            person_id: None,
            saved_only: None,
            sort: Some(updater.2),
            username: Some(username()),
        };

        // This is where the API is called for GetPersonDetails and the GetPersonDetailsResponse is returned and converted to PostView
        get_person_details(cx, &api_url_builder(cx, url_constructor, get_form))
            .await
            .unwrap()
            .posts
            .into()
    });

    let err_msg = "Error loading this post: ";

    view! { cx,
        <div class="card text-left">
            <div class="card-header">
                <div class="row">
                    <div class="col-sm-6">
                        <ul
                            class="nav nav-tabs card-header-tabs"
                            class:invisible=move || invisible_listing.get()
                        >
                            <li class="nav-item">
                                <a
                                    class="nav-link"
                                    class:disabled=move || subscribed_disabled
                                    aria-disabled=subscribed_disabled
                                    href="#"
                                >
                                    "Subscribed"
                                </a>
                            </li>
                            <li
                                class="nav-item"
                                on:click=move |_| {
                                    set_local_active.update(|value| *value = true);
                                    set_subscribed_active.update(|value| *value = false);
                                    set_all_active.update(|value| *value = false);
                                    set_active_tab.update(|value| *value = ListingType::Local)
                                }
                            >

                                <a
                                    class="nav-link"
                                    class:active=move || local_active.get()
                                    href="#"
                                >
                                    "Local"
                                </a>
                            </li>
                            <li
                                class="nav-item"
                                on:click=move |_| {
                                    set_all_active.update(|value| *value = true);
                                    set_local_active.update(|value| *value = false);
                                    set_subscribed_active.update(|value| *value = false);
                                    set_active_tab.update(|value| *value = ListingType::All)
                                }
                            >

                                <a class="nav-link" class:active=move || all_active.get() href="#">
                                    "All"
                                </a>
                            </li>
                        </ul>
                    </div>
                    <div class="col-sm-6">
                        <div class="dropstart text-end">
                            <button
                                class="btn btn-secondary dropdown-toggle"
                                style="margin-bottom: -0.50rem; margin-top: -0.25rem;"
                                type="button"
                                data-bs-toggle="dropdown"
                                aria-expanded="false"
                            >
                                {move || sorting.get().to_str()}
                            </button>
                            <ul class="dropdown-menu">
                                <li>
                                    <a
                                        class="dropdown-item"
                                        href="#"
                                        on:click=move |_| {
                                            set_sorting.update(|value| *value = SortType::Active)
                                        }
                                    >

                                        "Active"
                                    </a>
                                </li>
                                <li>
                                    <a
                                        class="dropdown-item"
                                        href="#"
                                        on:click=move |_| {
                                            set_sorting.update(|value| *value = SortType::Hot)
                                        }
                                    >

                                        "Hot"
                                    </a>
                                </li>
                                <li>
                                    <a
                                        class="dropdown-item"
                                        href="#"
                                        on:click=move |_| {
                                            set_sorting.update(|value| *value = SortType::New)
                                        }
                                    >

                                        "New"
                                    </a>
                                </li>
                                <li>
                                    <a
                                        class="dropdown-item"
                                        href="#"
                                        on:click=move |_| {
                                            set_sorting.update(|value| *value = SortType::Old)
                                        }
                                    >

                                        "Old"
                                    </a>
                                </li>
                                <li>
                                    <hr class="dropdown-divider"/>
                                </li>
                                <li>
                                    <a
                                        class="dropdown-item"
                                        href="#"
                                        on:click=move |_| {
                                            set_sorting.update(|value| *value = SortType::TopHour)
                                        }
                                    >

                                        "Top - Hour"
                                    </a>
                                </li>
                                <li>
                                    <a
                                        class="dropdown-item"
                                        href="#"
                                        on:click=move |_| {
                                            set_sorting.update(|value| *value = SortType::TopTwelveHour)
                                        }
                                    >

                                        "Top - 12 Hours"
                                    </a>
                                </li>
                                <li>
                                    <a
                                        class="dropdown-item"
                                        href="#"
                                        on:click=move |_| {
                                            set_sorting.update(|value| *value = SortType::TopDay)
                                        }
                                    >

                                        "Top - Day"
                                    </a>
                                </li>
                                <li>
                                    <a
                                        class="dropdown-item"
                                        href="#"
                                        on:click=move |_| {
                                            set_sorting.update(|value| *value = SortType::TopWeek)
                                        }
                                    >

                                        "Top - Week"
                                    </a>
                                </li>
                                <li>
                                    <a
                                        class="dropdown-item"
                                        href="#"
                                        on:click=move |_| {
                                            set_sorting.update(|value| *value = SortType::TopMonth)
                                        }
                                    >

                                        "Top - Month"
                                    </a>
                                </li>
                                <li>
                                    <a
                                        class="dropdown-item"
                                        href="#"
                                        on:click=move |_| {
                                            set_sorting
                                                .update(|value| *value = SortType::TopThreeMonths)
                                        }
                                    >

                                        "Top - 3 Months"
                                    </a>
                                </li>
                                <li>
                                    <a
                                        class="dropdown-item"
                                        href="#"
                                        on:click=move |_| {
                                            set_sorting.update(|value| *value = SortType::TopSixMonths)
                                        }
                                    >

                                        "Top - 6 Months"
                                    </a>
                                </li>
                                <li>
                                    <a
                                        class="dropdown-item"
                                        href="#"
                                        on:click=move |_| {
                                            set_sorting.update(|value| *value = SortType::TopYear)
                                        }
                                    >

                                        "Top - Year"
                                    </a>
                                </li>
                                <li>
                                    <a
                                        class="dropdown-item"
                                        href="#"
                                        on:click=move |_| {
                                            set_sorting.update(|value| *value = SortType::TopAll)
                                        }
                                    >

                                        "Top - All Time"
                                    </a>
                                </li>
                            </ul>
                        </div>
                    </div>
                </div>
            </div>
            <div class="card-body">
                <div class="container overflow-hidden">
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
                                                <FeedList posts=res.into()/>
                                            </div>
                                        }
                                    }
                                })
                        }}

                    </Transition>
                </div>

            </div>

            <Pagination/>
        </div>
    }
}

// The Posts list containing the Post items
// Suggestion: Implement collapsing accordions for multiple posts in a row from the same community in the home feed?
#[component]
fn FeedList(cx: Scope, posts: MaybeSignal<Vec<PostView>>) -> impl IntoView {
    view! { cx,
        {posts
            .get()
            .into_iter()
            .map(|post| view! { cx, <FeedItem post_view=leptos::MaybeSignal::Static(post)/> })
            .collect_view(cx)}
    }
}

// The individual Post items, may need to be a seperate component if it grows too large...
#[component]
pub fn FeedItem(cx: Scope, post_view: MaybeSignal<PostView>) -> impl IntoView {
    // These set the varaibles from the PostView struct to make insetion into the view easier
    let post = post_view.get();

    let post_link = format!("/post/{}", post.post.id);

    // Currently not used, may be used later on
    //let total_votes = post.counts.upvotes - post.counts.downvotes;

    // Checks to see if a post thumbnail exists, if it does not it setts it to a default image
    // This needs to handle external links as well with the external-link.png file
    let post_thumbnail = match post.post.thumbnail_url {
        Some(_) => post.post.thumbnail_url,
        _ => Option::Some("/static/default_assets/comment.png".to_string()),
    };

    let post_title = post.post.name;

    let post_pin = post.post.featured_local;

    // This truncates the body of a post to a set number of characters (defined with `body_length`) or less for the preview in the feed
    // This neds to be improved to not cut off words
    let post_truncated = match post.post.body {
        Some(body) => {
            let body_length = 250;
            let mut truncated_text = body;
            if truncated_text.len() > body_length {
                truncated_text = truncated_text
                    .chars()
                    .take(body_length)
                    .collect::<String>()
                    .trim()
                    .to_string();
                truncated_text += "...";
                truncated_text
            } else {
                truncated_text
            }
        }
        _ => "".to_string(),
    };

    // Checks to see if a user has an avatar set, if not it assigns a default one
    let creator_avatar = match post.creator.avatar {
        Some(_) => post.creator.avatar,
        _ => Option::Some("/static/default_assets/default-profile.png".to_string()),
    };

    let creator_name = post.creator.name.clone();

    let creator_link = if post.creator.local {
        format!("/user/{}", post.creator.name)
    } else {
        let url_regex = regex::Regex::new(r"https://(.*)/u/(.*)").unwrap();
        let external_creator_link = match url_regex.captures(&post.creator.actor_id) {
            Some(external_creator_link) => external_creator_link,
            None => {
                println!("url regex error: {:?}", &post.creator.actor_id);
                url_regex
                    .captures(&post.creator.actor_id)
                    .unwrap_or_else(|| url_regex.captures("https://error.com/u/error").unwrap())
            }
        };

        format!("/user/{}@{}", post.creator.name, &external_creator_link[1])
    };

    // This needs a similar check as above, I still need to make a default placeholder for a community avatar
    let community_avatar = match post.community.icon {
        Some(_) => post.community.icon,
        _ => Option::Some("/static/default_assets/default-community.png".to_string()),
    };

    // This needs to be handled better, it thinks that some local communities are external, possibly due to cross-posting?
    let community_name = if post.community.local {
        post.community.name.clone()
    } else {
        let url_regex = regex::Regex::new(r"https://(.*)/c/(.*)").unwrap();

        let external_community_link = match url_regex.captures(&post.community.actor_id) {
            Some(external_community_link) => external_community_link,
            None => {
                println!("url regex error: {:?}", &post.community.actor_id);
                url_regex
                    .captures(&post.community.actor_id)
                    .unwrap_or_else(|| url_regex.captures("https://error.com/c/error").unwrap())
            }
        };
        format!(
            "{}@{}",
            post.community.name.clone(),
            &external_community_link[1]
        )
    };

    let community_link = if post.community.local {
        format!("/community/{}", post.community.name)
    } else {
        let url_regex = regex::Regex::new(r"https://(.*)/c/(.*)").unwrap();

        let external_community_link = match url_regex.captures(&post.community.actor_id) {
            Some(external_community_link) => external_community_link,
            None => {
                println!("url regex error: {:?}", &post.community.actor_id);
                url_regex
                    .captures(&post.community.actor_id)
                    .unwrap_or_else(|| url_regex.captures("https://error.com/c/error").unwrap())
            }
        };

        format!(
            "/community/{}@{}",
            post.community.name, &external_community_link[1]
        )
    };

    let comment_count = post.counts.comments;

    view! { cx,
        <div class="row">

            // OLD LAYOUT CODE - MAY BE DELETED EVENTUALLY IF UNUSED
            // <div class="col-sm-1">
            // <div class="row">
            // <i class="bi bi-caret-up text-center"></i>
            // </div>
            // <div class="row">
            // <div class="text-center text-nowrap">
            // {post.counts.upvotes - post.counts.downvotes}
            // </div>
            // </div>
            // <div class="row">
            // <i class="bi bi-caret-down text-center"></i>
            // </div>
            // </div>

            // <div class="col-sm-11">
            // <div class="row">
            // <div class="col-sm-2">
            // <div style="">
            // <img
            // src=post_thumbnail
            // class="rounded overflow-hidden"
            // alt="..."
            // style="max-width: 75px; max-height: 75px;"
            // />
            // </div>
            // </div>
            // <div class="col-sm-10">
            // <div class="row">
            // <h5>
            // <a
            // href=post_link.clone()
            // class="link-offset-2 link-underline link-underline-opacity-0"
            // >
            // {post_title}
            // </a>
            // {if post_pin {
            // view! { cx, <i class="bi bi-pin-angle-fill"></i> }
            // } else {
            // view! { cx, <i></i> }
            // }}

            // </h5>
            // </div>
            // <div class="row">
            // <div class="col-sm-12">
            // <p>
            // <a href=creator_link>
            // <img
            // src=creator_avatar
            // alt="mdo"
            // width="32"
            // height="32"
            // class="rounded"
            // />
            // "  "
            // {creator_name}
            // </a>
            // " | "
            // <a href=community_link>
            // <img
            // src=community_avatar
            // alt="mdo"
            // width="32"
            // height="32"
            // class="rounded"
            // />
            // "  "
            // {community_name}
            // </a>
            // </p>
            // </div>
            // </div>
            // <div class="card-footer">
            // <span>
            // <a
            // href=post_link
            // class="link-secondary link-offset-2 link-underline link-underline-opacity-0"
            // >
            // <i class="bi bi-chat-right-text"></i>
            // " "
            // {comment_count}
            // " Comments"
            // </a>
            // "   "
            // <i class="bi bi-bookmark-star text-secondary"></i>
            // "   "
            // <i class="bi bi-signpost-split text-secondary"></i>
            // "   "
            // <i class="bi bi-flag text-secondary"></i>
            // </span>

            // </div>
            // </div>
            // </div>
            // </div>
            // <hr/>

            <div class="card mb-3 pe-0 ps-0">
                <div class="row g-0">
                    // Voting Column
                    // <div class="col-md-1 align-self-center">
                    // <div class="row align-self-center">
                    // <h4 class="text-center">
                    // <i class="bi bi-caret-up text-center"></i>
                    // </h4>
                    // </div>
                    // <div class="row align-self-center">
                    // <h4 class="text-center text-nowrap">
                    // {post.counts.upvotes - post.counts.downvotes}
                    // </h4>
                    // </div>
                    // <div class="row align-self-center">
                    // <h4 class="text-center">
                    // <i class="bi bi-caret-down text-center"></i>
                    // </h4>
                    // </div>
                    // </div>

                    // Image/Thumbnail Column
                    <div class="col-md-3 align-self-center text-center">
                        <img
                            src=post_thumbnail
                            class="img-fluid rounded-start"
                            alt="..."
                            style="max-height: 300px;"
                        />
                    </div>

                    // Content Column
                    <div
                        class="col-md-9"
                        style="display: flex; min-height: 1vh; flex-direction: column;"
                    >
                        <div class="card-body" style="flex: 1 0 auto;">
                            <h5 class="card-title">
                                <a
                                    href=post_link.clone()
                                    class="link-offset-2 link-underline link-underline-opacity-0"
                                >
                                    {post_title}
                                </a>
                                {if post_pin {
                                    view! { cx, <i class="bi bi-pin-angle-fill"></i> }
                                } else {
                                    view! { cx, <i></i> }
                                }}

                            </h5>
                            <p class="card-text">{post_truncated}</p>
                            <br/>
                        </div>
                        <span class="m-3">
                            <a href=creator_link class="">
                                <img
                                    src=creator_avatar
                                    alt="mdo"
                                    width="32"
                                    height="32"
                                    class="rounded"
                                />
                                "  "
                                {creator_name}
                            </a>
                        </span>
                        <div class="card-footer me-0">
                            <div class="row align-items-center">
                                <div class="col-sm-5">
                                    <span class="text-nowrap">
                                        <i class="bi bi-caret-up text-center"></i>
                                        " "
                                        {post.counts.upvotes - post.counts.downvotes}
                                        " "
                                        <i class="bi bi-caret-down text-center"></i>
                                        "   "
                                        <div class="vr"></div>
                                        "   "
                                        <a
                                            href=post_link
                                            class="link-secondary link-offset-2 link-underline link-underline-opacity-0"
                                        >
                                            <i class="bi bi-chat-right-text"></i>
                                            " "
                                            {comment_count}
                                            " Comments"
                                        </a>
                                        "   "
                                        <i class="bi bi-bookmark-star text-secondary"></i>
                                        "   "
                                        <i class="bi bi-signpost-split text-secondary"></i>
                                        "   "
                                        <i class="bi bi-flag text-secondary"></i>
                                    </span>
                                </div>
                                <div class="col-sm-7 text-end">
                                    <a href=community_link>
                                        <img
                                            src=community_avatar
                                            alt="mdo"
                                            width="32"
                                            height="32"
                                            class="rounded"
                                        />
                                        "  "
                                        {community_name}
                                    </a>
                                </div>
                            </div>

                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

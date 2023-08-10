use leptos::*;
use leptos_router::*;
use markdown::*;

use crate::api::community::{get_community, list_communities};
use crate::api::posts::get_posts;
use crate::api::site::get_site;
use crate::api::structs::*;
use crate::api::user::get_person_details;
use crate::api::*;
use crate::components::instance::*;

#[component]
pub fn Sidebar(cx: Scope, endpoint: &'static str) -> impl IntoView {
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

    let post_id = move || {
        params
            .with(|params| params.get("id").cloned())
            .unwrap_or_default()
            .parse::<i32>()
            .unwrap()
    };

    if endpoint.eq("community") {
        let sidebar = create_resource(cx, page, move |page| async move {
            // This constructs the proper API URL for GetPosts or GetPersonDetails
            let url_constructor = ApiUrlConstructor {
                endpoint: api_endpoints::GetEndpoint::GET_COMMUNITY.to_string(),
                id: None,
                params: None,
            };

            let get_form = GetCommunity {
                auth: None,
                id: None,
                name: Some(community_name()),
            };

            get_community(cx, &api_url_builder(cx, url_constructor, get_form))
                .await
                .ok()
        });

        let err_msg = "Error loading this post: ";

        view! { cx,
            <div class="card text-left">
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
                        sidebar
                            .read(cx)
                            .map(|res| match res {
                                None => {
                                    view! { cx, <div>{format!("{err_msg}")}</div> }
                                }
                                Some(res) => {
                                    let sidebar = match res
                                        .community_view
                                        .community
                                        .description
                                        .clone()
                                    {
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
                                            // Community sidecard
                                            <div class="card text-left">
                                                <div class="card-header">
                                                    <img
                                                        src=res.community_view.community.banner
                                                        class="card-img-top"
                                                        alt="Banner for the Lemmy instance"
                                                    />
                                                </div>
                                                <div class="card-body">
                                                    <h4 class="card-title text-center text-nowrap">
                                                        {res.community_view.community.title}
                                                    </h4>
                                                    <hr/>
                                                    <div inner_html=sidebar></div>
                                                    <hr/>
                                                    <h6>"Moderators"</h6>
                                                    <ul class="list-group list-group-flush">
                                                        {res
                                                            .moderators
                                                            .into_iter()
                                                            .map(|moderator| {
                                                                view! { cx,
                                                                    // Checks to see if a user has an avatar set, if not it assigns a default one

                                                                    {
                                                                        let admin_avatar = match moderator.moderator.avatar {
                                                                            Some(_) => moderator.moderator.avatar,
                                                                            _ => {
                                                                                Option::Some(
                                                                                    "/static/default_assets/default-profile.png".to_string(),
                                                                                )
                                                                            }
                                                                        };

                                                                        view! { cx,
                                                                            <li class="list-group-item">
                                                                                <a href=format!("/user/{}", moderator.moderator.name)>
                                                                                    <img
                                                                                        src=admin_avatar
                                                                                        alt="mdo"
                                                                                        width="32"
                                                                                        height="32"
                                                                                        class="rounded"
                                                                                    />
                                                                                    "  "
                                                                                    {moderator.moderator.name}
                                                                                </a>
                                                                            </li>
                                                                        }
                                                                    }
                                                                }
                                                            })
                                                            .collect::<Vec<_>>()}
                                                    </ul>
                                                </div>
                                            </div>
                                            <br/>
                                            <div class="card text-left">
                                                <div class="card-header">
                                                    <h5 class="card-title text-center">"Statistics"</h5>
                                                </div>
                                                <div class="card-body">
                                                    <table class="table table-dark">
                                                        <thead>
                                                            <tr>
                                                                <th scope="col">"Users"</th>
                                                                <th scope="col">
                                                                    <div class="vr"></div>
                                                                </th>
                                                                <th scope="col">"Activity"</th>
                                                            </tr>
                                                        </thead>
                                                        <tbody>
                                                            <tr>
                                                                <td>
                                                                    {format!("{} Users", res.community_view.counts.subscribers)}
                                                                </td>
                                                                <td>
                                                                    <div class="vr"></div>
                                                                </td>
                                                                <td>
                                                                    {format!("{} Communities", res.community_view.counts.posts)}

                                                                </td>
                                                            </tr>
                                                            <tr>
                                                                <td>
                                                                    {format!(
                                                                        "{} / Day", res.community_view.counts.users_active_day
                                                                    )}
                                                                </td>
                                                                <td>
                                                                    <div class="vr"></div>
                                                                </td>
                                                                <td>
                                                                    {format!("{} Posts", res.community_view.counts.posts)}
                                                                </td>
                                                            </tr>
                                                            <tr>
                                                                <td>
                                                                    {format!(
                                                                        "{} / Month", res.community_view.counts.users_active_month
                                                                    )}

                                                                </td>
                                                                <td>
                                                                    <div class="vr"></div>
                                                                </td>
                                                                <td>
                                                                    {format!(
                                                                        "{:?} Comments", res.community_view.counts.comments
                                                                    )}

                                                                </td>
                                                            </tr>
                                                        </tbody>
                                                    </table>
                                                </div>
                                            </div>
                                        </div>
                                    }
                                }
                            })
                    }}

                </Transition>
            </div>
        }
    } else if endpoint.eq("home") {
        let sidebar = create_resource(cx, page, move |page| async move {
            // This constructs the proper API URL for GetPosts or GetPersonDetails
            let url_constructor = ApiUrlConstructor {
                endpoint: api_endpoints::GetEndpoint::GET_SITE.to_string(),
                id: None,
                params: None,
            };

            let get_form = GetSite { auth: None };

            get_site(cx, &api_url_builder(cx, url_constructor, get_form))
                .await
                .ok()
        });

        let err_msg = "Error loading this post: ";

        view! { cx,
            <div>
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
                        sidebar
                            .read(cx)
                            .map(|res| match res {
                                None => {
                                    view! { cx, <div>{format!("{err_msg}")}</div> }
                                }
                                Some(res) => {
                                    let sidebar = match res.site_view.site.sidebar.clone() {
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
                                            // Instance sidecard
                                            <div class="card text-left">
                                                <div class="card-header">
                                                    <img
                                                        src=res.site_view.site.banner
                                                        class="card-img-top"
                                                        alt="Banner for the Lemmy instance"
                                                    />
                                                </div>
                                                <div class="card-body">
                                                    <h4 class="card-title text-center text-nowrap">
                                                        {res.site_view.site.name}
                                                    </h4>
                                                    <h6 class="text-center">
                                                        {res.site_view.site.description}
                                                    </h6>
                                                    <hr/>
                                                    <div inner_html=sidebar></div>
                                                    <hr/>
                                                    <h6>"Admins"</h6>
                                                    <ul class="list-group list-group-flush">
                                                        {res
                                                            .admins
                                                            .into_iter()
                                                            .map(|admin| {
                                                                view! { cx,
                                                                    // Checks to see if a user has an avatar set, if not it assigns a default one

                                                                    {
                                                                        let admin_avatar = match admin.person.avatar {
                                                                            Some(_) => admin.person.avatar,
                                                                            _ => {
                                                                                Option::Some(
                                                                                    "/static/default_assets/default-profile.png".to_string(),
                                                                                )
                                                                            }
                                                                        };

                                                                        view! { cx,
                                                                            <li class="list-group-item">
                                                                                <a href=format!("/user/{}", admin.person.name)>
                                                                                    <img
                                                                                        src=admin_avatar
                                                                                        alt="mdo"
                                                                                        width="32"
                                                                                        height="32"
                                                                                        class="rounded"
                                                                                    />
                                                                                    "  "
                                                                                    {admin.person.name}
                                                                                </a>
                                                                            </li>
                                                                        }
                                                                    }
                                                                }
                                                            })
                                                            .collect::<Vec<_>>()}
                                                    </ul>
                                                </div>
                                            </div>
                                            <br/>
                                            <TrendingCommunities/>
                                            <br/>
                                            // Statistics Card
                                            <div class="card text-left">
                                                <div class="card-header">
                                                    <h5 class="card-title text-center">"Statistics"</h5>
                                                </div>
                                                <div class="card-body">
                                                    <table class="table table-dark">
                                                        <thead>
                                                            <tr>
                                                                <th scope="col">"Users"</th>
                                                                <th scope="col">
                                                                    <div class="vr"></div>
                                                                </th>
                                                                <th scope="col">"Activity"</th>
                                                            </tr>
                                                        </thead>
                                                        <tbody>
                                                            <tr>
                                                                <td>{format!("{} Users", res.site_view.counts.users)}</td>
                                                                <td>
                                                                    <div class="vr"></div>
                                                                </td>
                                                                <td>
                                                                    {format!(
                                                                        "{} Communities", res.site_view.counts.communities
                                                                    )}

                                                                </td>
                                                            </tr>
                                                            <tr>
                                                                <td>
                                                                    {format!("{} / Day", res.site_view.counts.users_active_day)}
                                                                </td>
                                                                <td>
                                                                    <div class="vr"></div>
                                                                </td>
                                                                <td>{format!("{} Posts", res.site_view.counts.posts)}</td>
                                                            </tr>
                                                            <tr>
                                                                <td>
                                                                    {format!(
                                                                        "{} / Month", res.site_view.counts.users_active_month
                                                                    )}

                                                                </td>
                                                                <td>
                                                                    <div class="vr"></div>
                                                                </td>
                                                                <td>
                                                                    {format!(
                                                                        "{:?} Comments", res.site_view.counts.comments.unwrap()
                                                                    )}

                                                                </td>
                                                            </tr>
                                                        </tbody>
                                                    </table>
                                                </div>
                                            </div>
                                        </div>
                                    }
                                }
                            })
                    }}

                </Transition>
            </div>
        }
    } else if endpoint.eq("user") {
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

        let err_msg = "Error loading this post: ";

        view! { cx,
            <div class="card text-left">
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
                        sidebar
                            .read(cx)
                            .map(|res| match res {
                                None => {
                                    view! { cx, <div>{format!("{err_msg}")}</div> }
                                }
                                Some(res) => {
                                    let sidebar = match res.person_view.person.bio.clone() {
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
                                            // User sidecard
                                            <div class="card text-left">
                                                <div class="card-header">
                                                    <img
                                                        src=res.person_view.person.avatar
                                                        class="card-img-top"
                                                        alt="Banner for the Lemmy instance"
                                                    />
                                                </div>
                                                <div class="card-body">
                                                    <h4 class="card-title text-center text-nowrap">
                                                        {res.person_view.person.name}
                                                    </h4>
                                                    <hr/>
                                                    <div inner_html=sidebar></div>
                                                    <hr/>
                                                // Need to add showing what groups someone moderates, if any.
                                                // <h6>"Moderates"</h6>
                                                // <ul class="list-group list-group-flush">
                                                // {res
                                                // .admins
                                                // .into_iter()
                                                // .map(|admin| {
                                                // view! { cx,
                                                // 

                                                // {
                                                // let admin_avatar = match admin.person.avatar {
                                                // Some(_) => admin.person.avatar,
                                                // _ => {
                                                // Option::Some(
                                                // "/static/default_assets/default-profile.png".to_string(),
                                                // )
                                                // }
                                                // };

                                                // view! { cx,
                                                // <li class="list-group-item">
                                                // <a href=format!("/user/{}", admin.person.name)>
                                                // <img
                                                // src=admin_avatar
                                                // alt="mdo"
                                                // width="32"
                                                // height="32"
                                                // class="rounded"
                                                // />
                                                // "  "
                                                // {admin.person.name}
                                                // </a>
                                                // </li>
                                                // }
                                                // }
                                                // }
                                                // })
                                                // .collect::<Vec<_>>()}
                                                // </ul>
                                                </div>
                                            </div>
                                            <br/>
                                            // User Statistics
                                            <div class="card text-left">
                                                <div class="card-header">
                                                    <h5 class="card-title text-center">"Statistics"</h5>
                                                </div>
                                                <div class="card-body">
                                                    <table class="table table-dark">
                                                        <thead>
                                                            <tr>
                                                                <th scope="col">"Activity"</th>
                                                                <th scope="col">
                                                                    <div class="vr"></div>
                                                                </th>
                                                                <th scope="col">"Karma"</th>
                                                            </tr>
                                                        </thead>
                                                        <tbody>
                                                            <tr>
                                                                <td>
                                                                    {format!("{} Posts", res.person_view.counts.post_count)}
                                                                </td>
                                                                <td>
                                                                    <div class="vr"></div>
                                                                </td>
                                                                <td>{format!("{} Karma", res.person_view.counts.post_score)}
                                                                </td>
                                                            </tr>
                                                            <tr>
                                                                <td>
                                                                    {format!(
                                                                        "{} Comments", res.person_view.counts.comment_count
                                                                    )}
                                                                </td>
                                                                <td>
                                                                    <div class="vr"></div>
                                                                </td>
                                                                <td>
                                                                    {format!("{} Karma", res.person_view.counts.comment_score)}
                                                                </td>
                                                            </tr>
                                                        </tbody>
                                                    </table>
                                                </div>
                                            </div>
                                        </div>
                                    }
                                }
                            })
                    }}

                </Transition>
            </div>
        }
    } else {
        unreachable!()
    }
}

#[component]
pub fn TrendingCommunities(cx: Scope) -> impl IntoView {
    let query = use_query_map(cx);
    let page = move || {
        query
            .with(|q| q.get("page").and_then(|page| page.parse::<i32>().ok()))
            .unwrap_or(1)
    };

    // Variable that holds the returned ListCommunitiesResponse from the API
    let communities = create_resource(cx, page, move |page| async move {
        // This constructs the proper API URL for GetPosts
        let url_constructor = ApiUrlConstructor {
            endpoint: api_endpoints::GetEndpoint::LIST_COMMUNITIES.to_string(),
            id: None,
            params: None,
        };

        // This assembles the ListCommunities request form
        let get_form = ListCommunities {
            auth: None,
            limit: Some(10),
            page: Some(1),
            show_nsfw: None,
            sort: Some(SortType::TopMonth),
            type_: Some(ListingType::Local),
        };

        // This is where the API is called for ListCommunities and the ListCommunitiesResponse is returned
        list_communities(cx, &api_url_builder(cx, url_constructor, get_form))
            .await
            .ok()
    });

    let err_msg = "Error loading this post: ";

    view! { cx,
        <div>
            <div class="card text-left">
                <div class="card-header">
                    <h5 class="card-title text-center">"Trending Communities"</h5>
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
                        {move || {
                            communities
                                .read(cx)
                                .map(|res| match res {
                                    None => {
                                        view! { cx, <div>{format!("{err_msg}")}</div> }
                                    }
                                    Some(res) => {

                                        view! { cx,
                                            <div>
                                                <TrendingCommunityItems communities=res.communities/>
                                            </div>
                                        }
                                    }
                                })
                        }}

                    </Transition>
                </div>
            </div>
            <br/>
        </div>
    }
}

#[component]
fn TrendingCommunityItems(cx: Scope, communities: Vec<CommunityView>) -> impl IntoView {
    view! { cx,
        <table class="table table-dark">
            <tbody>
                {communities
                    .into_iter()
                    .map(|item| {
                        view! { cx,
                            <tr>
                                <td>
                                    <a href=format!(
                                        "/community/{}", item.community.name
                                    )>{format!("{}", item.community.title)}</a>
                                </td>
                            </tr>
                        }
                    })
                    .collect_view(cx)}
            </tbody>
        </table>
    }
}

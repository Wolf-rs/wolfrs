use leptos::*;
use leptos_router::*;
use markdown::*;

use crate::api::community::{get_community, list_communities};
use crate::api::posts::{get_post, get_posts};
use crate::api::site::get_site;
use crate::api::structs::*;
use crate::api::user::get_person_details;
use crate::api::*;
use crate::components::instance::*;

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

use leptos::*;
use leptos_router::*;

use crate::api::community::list_communities;
use crate::api::structs::*;
use crate::api::*;
use crate::components::pagination::Pagination;

// TODO - communities.rs:
// Improve styling to deal with the small overflow that happens with mobile views
// Add more sorting options for Total Subs, MAU's, Posts, etc
// Add functionality for filtering by subscribed, local, and all
// Potentially move logic from the page to an actual component to maintain seperation of page from components (Optional)

#[component]
pub fn Communities(cx: Scope) -> impl IntoView {
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
            limit: Some(50),
            page: Some(page),
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

    view!(cx,
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
                communities
                    .read(cx)
                    .map(|res| match res {
                        None => {
                            view! { cx, <div>{format!("{err_msg}")}</div> }
                        }
                        Some(res) => {
                            view! { cx,
                                <div class="row">
                                    <CommunitiesList communities=res.communities />
                                </div>
                            }
                        }
                    })
            }}
            </Transition>

            <Pagination />
        </div>
    )
}

#[component]
pub fn CommunitiesList(cx: Scope, communities: Vec<CommunityView>) -> impl IntoView {
    view! { cx,
        <div class="col-md-12">
            <div class="container">
                <table class="table table-dark table-striped">
                    <thead>
                        <tr>
                            <th colspan="3"><h3>"Communites List"</h3></th>
                        </tr>
                        <tr>
                            <th scope="col">"Name"</th>
                            <th scope="col">"Subscribers"</th>
                            <th scope="col">"Active Users"</th>
                            <th scope="col">"Posts"</th>
                            <th scope="col">"Comments"</th>
                            <th scope="col">"Subscription"</th>
                        </tr>
                    </thead>
                    <CommunitiesListItem communities=communities />
                </table>
            </div>
        </div>
    }
}

#[component]
pub fn CommunitiesListItem(cx: Scope, communities: Vec<CommunityView>) -> impl IntoView {
    view! { cx,

      <tbody>
      {communities.into_iter()
        .map(|item| view! { cx,
            <tr>
            <td><a href={format!("{}", item.community.actor_id)}>{format!("{}", item.community.title)}</a></td>
            <td>{format!("{}", item.counts.subscribers)}</td>
            <td>{format!("{}", item.counts.users_active_month)}</td>
            <td>{format!("{}", item.counts.posts)}</td>
            <td>{format!("{}", item.counts.comments)}</td>
            // Needs to be dynamic and allow for subscribing from here?
            <td>{format!("{:?}", item.subscribed)}</td>
            </tr>})
        .collect_view(cx)}
      </tbody>

    }
}

use leptos::*;
use leptos_router::*;
use serde_json::Value;

use crate::api::modlog::get_mod_log;
use crate::api::structs::*;
use crate::api::*;

// TODO - mod_log.rs:
// Actually implement the mod log functionality
// Figure out how to implement a unified mod log in chronological format
// Build in sorting based on categories
// Figure out good limit for mod log actions per page (My guess is 10, which works out to 100 actions per page)

/// The component box for the mod log of the local and federated instances.
#[component]
pub fn ModLog(cx: Scope) -> impl IntoView {
    let query = use_query_map(cx);
    let page = move || {
        query
            .with(|q| q.get("page").and_then(|page| page.parse::<i32>().ok()))
            .unwrap_or(1)
    };

    // Variable that holds the returned GetPostsResponse from the API
    let modlog = create_resource(cx, page, move |page| async move {
        // This constructs the proper API URL for GetPosts
        let url_constructor = ApiUrlConstructor {
            endpoint: api_endpoints::GetEndpoint::GET_MODLOG.to_string(),
            id: None,
            params: None,
        };

        // This assembles the GetPosts request form
        let get_form = GetModlog {
            auth: None,
            community_id: None,
            // 3 for testing, 100 for release
            limit: Some(3),
            mod_person_id: None,
            other_person_id: None,
            page: Some(page),
            type_: None,
        };

        // This is where the API is called for GetPosts and the GetPostsResponse is returned
        get_mod_log(cx, &api_url_builder(cx, url_constructor, get_form))
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
                modlog
                    .read(cx)
                    .map(|res| match res {
                        None => {
                            view! { cx, <div>{format!("{err_msg}")}</div> }
                        }
                        Some(res) => {
                            view! { cx,
                                <div>
                                    <ModLogList items=res.into() />
                                </div>
                            }
                        }
                    })
            }}
            </Transition>

            // Pagination for the mod log
            <nav aria-label="Feed Page Navigation">
                <ul class="pagination justify-content-center">
                    {move || if page() > 1 {
                        view! { cx, <li class="page-item"><A class="page-link" href=move || format!("?page={}", page() - 1) >Previous</A></li>}
                    } else {
                        view! { cx, <li class="page-item disabled"><A class="page-link" href="" >Previous</A></li>}
                    }}
                    <li class="page-item"><A class="page-link" href=move || format!("/")>Page: {page}</A></li>
                    <li class="page-item"><A class="page-link" href=move || format!("?page={}", page() + 1) >Next</A></li>
                </ul>
            </nav>
        </div>
    }
}

// The modlog list containing the modlog items
#[component]
fn ModLogList(cx: Scope, items: MaybeSignal<GetModlogResponse>) -> impl IntoView {
    let sorted_modlog = mod_log_sorter(items.get());

    view! { cx,
      <ul>
        <ModLogItem items=sorted_modlog />
      </ul>
    }
}

// The WIP component for displaying modlog actions as individual items
#[component]
fn ModLogItem(cx: Scope, items: Vec<String>) -> impl IntoView {
    view! { cx,

      {items.into_iter()
        .map(|item| view! { cx, <li>{format!("{:?}", item)}</li>})
        .collect_view(cx)}

    }
}

// The function for (trying) to sort the modlog actions into a unified, chronological list
// This feels like a really scuffed way to sort a struct of Vectors...
// If someone knows a better way to do this, please feel free to improve it. It needs to be sorted by the `when_` field for each vector in the struct
fn mod_log_sorter(items: GetModlogResponse) -> Vec<String> {
    let mut sorted_modlog = Vec::new();

    for item in items.added {
        sorted_modlog.push(format!("{:?}", item));
    }

    for item in items.added_to_community {
        sorted_modlog.push(format!("{:?}", item));
    }

    for item in items.admin_purged_comments {
        sorted_modlog.push(format!("{:?}", item));
    }

    for item in items.admin_purged_communities {
        sorted_modlog.push(format!("{:?}", item));
    }

    for item in items.admin_purged_persons {
        sorted_modlog.push(format!("{:?}", item));
    }

    for item in items.admin_purged_posts {
        sorted_modlog.push(format!("{:?}", item));
    }

    for item in items.banned {
        sorted_modlog.push(format!("{:?}", item));
    }

    for item in items.banned_from_community {
        sorted_modlog.push(format!("{:?}", item));
    }

    for item in items.featured_posts {
        sorted_modlog.push(format!("{:?}", item));
    }

    for item in items.hidden_communities {
        sorted_modlog.push(format!("{:?}", item));
    }

    for item in items.locked_posts {
        sorted_modlog.push(format!("{:?}", item));
    }

    for item in items.removed_comments {
        sorted_modlog.push(format!("{:?}", item));
    }

    for item in items.removed_communities {
        sorted_modlog.push(format!("{:?}", item));
    }

    for item in items.removed_posts {
        sorted_modlog.push(format!("{:?}", item));
    }

    for item in items.transferred_to_community {
        sorted_modlog.push(format!("{:?}", item));
    }

    // Not sorted...
    sorted_modlog
}

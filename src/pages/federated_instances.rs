use leptos::*;
use leptos_router::*;

use crate::api::federation::get_federated_instances;
use crate::api::structs::*;
use crate::api::*;

// TODO - federated_instances.rs:
// Improve styling to deal with the small overflow that happens with the tables between shifting from 2 columns to 1 column
// Potentially move logic from the page to an actual component to maintain seperation of page from components (Optional)

/// The component box for federated instances page.
#[component]
pub fn FederatedInstancesList(cx: Scope) -> impl IntoView {
    // Technically not necessary for this page, but not sure how to use `create_resource` without it?
    let query = use_query_map(cx);
    let page = move || {
        query
            .with(|q| q.get("page").and_then(|page| page.parse::<i32>().ok()))
            .unwrap_or(1)
    };

    // Variable that holds the returned GetFederatedInstancesResponse from the API
    let instances = create_resource(cx, page, move |_page| async move {
        // This constructs the proper API URL for GetFederatedInstances
        let url_constructor = ApiUrlConstructor {
            endpoint: api_endpoints::GetEndpoint::GET_FEDERATED_INSTANCES.to_string(),
            id: None,
            params: None,
        };

        // This assembles the GetFederatedInstances request form
        let get_form = GetFederatedInstances { auth: None };

        // This is where the API is called for GetFederatedInstances and the GetFederatedInstancesResponse is returned
        get_federated_instances(cx, &api_url_builder(cx, url_constructor, get_form))
            .await
            .ok()
    });

    let err_msg = "Error loading this page: ";

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
                    instances
                        .read(cx)
                        .map(|res| match res {
                            None => {
                                view! { cx, <div>{format!("{err_msg}")}</div> }
                            }
                            Some(res) => {

                                view! { cx,
                                    <div class="row">
                                        <FederatedInstancesTables instances=res
                                            .federated_instances
                                            .unwrap()
                                            .into()/>
                                    </div>
                                }
                            }
                        })
                }}

            </Transition>
        </div>
    }
}

// The component for crafting the two core tables for the federated instances items
#[component]
fn FederatedInstancesTables(
    cx: Scope,
    instances: MaybeSignal<FederatedInstances>,
) -> impl IntoView {
    view! { cx,
        <div class="col-lg-6 col-md-12">
            <div class="container">
                <table class="table table-dark table-striped">
                    <thead>
                        <tr>
                            <th colspan="3">
                                <h3>"Federated Instances"</h3>
                            </th>
                        </tr>
                        <tr>
                            <th scope="col">"Domain Name"</th>
                            <th scope="col">"Software"</th>
                            <th scope="col">"Version"</th>
                        </tr>
                    </thead>
                    // Check to see if the instance is using an allowed list or just has linked instances, as these should be mutually exclusive
                    {if instances.get().allowed.is_empty() {
                        view! { cx, <Federated federated_instances=instances.get().linked/> }
                    } else {
                        view! { cx, <Federated federated_instances=instances.get().allowed/> }
                    }}

                </table>
            </div>
        </div>
        <div class="col-lg-6 col-md-12">
            <div class="container">
                <table class="table table-dark table-striped">
                    <thead>
                        <tr>
                            <th colspan="3">"Blocked Instances"</th>
                        </tr>
                        <tr>
                            <th scope="col">"Domain Name"</th>
                            <th scope="col">"Software"</th>
                            <th scope="col">"Version"</th>
                        </tr>
                    </thead>
                    <Blocked blocked_instances=instances.get().blocked/>
                </table>
            </div>
        </div>
    }
}

// The component for building the federated instances table for allows and linked items
#[component]
fn Federated(cx: Scope, federated_instances: Vec<Instance>) -> impl IntoView {
    let mut sorted_instances = federated_instances;

    sorted_instances.sort_by(|a, b| a.domain.cmp(&b.domain));

    view! { cx,
        <tbody>
            {sorted_instances
                .into_iter()
                .map(|item| {
                    view! { cx,
                        <tr>
                            <td>
                                <a
                                    rel="external"
                                    href=format!("https://{}", item.domain)
                                    target="_blank"
                                >
                                    {format!("{}", item.domain)}
                                </a>
                            </td>
                            <td>{format!("{}", item.software.unwrap_or_default())}</td>
                            <td>{format!("{}", item.version.unwrap_or_default())}</td>
                        </tr>
                    }
                })
                .collect_view(cx)}
        </tbody>
    }
}

// The component for building the federated instances table for blocked items
#[component]
fn Blocked(cx: Scope, blocked_instances: Vec<Instance>) -> impl IntoView {
    let mut sorted_instances = blocked_instances;

    sorted_instances.sort_by(|a, b| a.domain.cmp(&b.domain));

    view! { cx,
        <tbody>
            {sorted_instances
                .into_iter()
                .map(|item| {
                    view! { cx,
                        <tr>
                            <td>
                                <a
                                    rel="external"
                                    href=format!("https://{}", item.domain)
                                    target="_blank"
                                >
                                    {format!("{}", item.domain)}
                                </a>
                            </td>
                            <td>{format!("{}", item.software.unwrap_or_default())}</td>
                            <td>{format!("{}", item.version.unwrap_or_default())}</td>
                        </tr>
                    }
                })
                .collect_view(cx)}
        </tbody>
    }
}

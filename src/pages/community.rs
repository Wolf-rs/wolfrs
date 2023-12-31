use leptos::*;
use leptos_router::*;

use crate::api::structs::router_endpoints;
use crate::components::{feed::Feed, sidecards::community::Sidecard};

// TODO - community.rs:
// Sidecard component still needs to be built
// Real styling with Bootstrap
// Ensure mobile layout works as expected

// The main page for the community, its feed, and its sidebar.

#[component]
pub fn Community(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="container overflow-hidden">
            <div class="row gx-4">
                // Feed Column
                <div class="col-md-9">
                    <br/>
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
                        <Feed endpoint=router_endpoints::RouterEndpoint::COMMUNITY/>
                    </Transition>
                </div>

                // Sidecard Column
                <div class="col-12 col-md-3">
                    <br/>
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
                        <Sidecard/>
                    </Transition>
                </div>
            </div>
        </div>
    }
}

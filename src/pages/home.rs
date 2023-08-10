use leptos::*;

use crate::api::structs::router_endpoints;
use crate::components::feed::Feed;
use crate::components::sidecard::{Sidebar, TrendingCommunities};

/// Renders the home page of the Lemmy instance, which is the feed and sidebar screen.
#[component]
pub fn Home(cx: Scope) -> impl IntoView {
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
                        <Feed endpoint=router_endpoints::RouterEndpoint::HOME/>
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
                        <Sidebar endpoint=router_endpoints::RouterEndpoint::HOME/>
                    </Transition>

                </div>
            </div>
        </div>
    }
}

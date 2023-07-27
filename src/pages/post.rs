use leptos::*;
use leptos_router::*;

use crate::components::{comments::Comments, post_view::PostView};
#[component]
pub fn Post(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="container overflow-hidden">
                <div class="row gx-4">
                    // Feed Column
                    <div  class="col-md-9">
                    <Transition fallback=move || {
                        // Handles the loading screen while waiting for a reply from the API
                        view! { cx,
                            <div class="d-flex align-items-center">
                                <h1>Loading...</h1>
                                <div class="spinner-grow ms-auto" role="status" aria-hidden="true"></div>
                            </div>
                         }
                    }>
                        <PostView />
                        // Comments are currently not working. The comments data is loaded from the API, but Anyhow interprets them as having error, so wraps them in an Err() and that breaks things.
                        //<Comments />
                    </Transition>
                    </div>

                    // Sidecard Column
                    <div  class="col-12 col-md-3">
                        <div style="background-color: #1ea;" class="row gy-4">
                            //<Trending />
                        </div>
                        <div style="background-color: #b47;" class="row">
                            //<Sidecard />
                        </div>
                    </div>
                </div>
            </div>
    }
}

use leptos::*;
use leptos_router::*;

use crate::components::{comments::Comments, post_view::PostView};

// TODO - post.rs:
// Sidecard component still needs to be built
// Real styling with Bootstrap
// Ensure mobile layout works as expected

// The main page for viewing a post and its comments.

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
                        <Comments />
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

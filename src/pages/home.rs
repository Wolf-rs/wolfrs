use leptos::*;

use crate::components::feed::Feed;

/// Renders the home page of the Lemmy instance, which is the feed and sidebar screen.
#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    view! { cx,

            <div class="container overflow-hidden">
                <div class="row gx-4">
                    // Feed Column
                    <div  class="col-md-9">
                        <Feed />
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

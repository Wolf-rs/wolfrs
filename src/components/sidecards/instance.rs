use leptos::*;
use leptos_router::*;
use markdown::*;

use crate::api::site::get_site;
use crate::api::structs::*;
use crate::api::*;
use crate::components::instance::*;
use crate::components::sidecards::trending::TrendingCommunities;

#[component]
pub fn Sidecard(cx: Scope) -> impl IntoView {
    let query = use_query_map(cx);
    let page = move || {
        query
            .with(|q| q.get("page").and_then(|page| page.parse::<i32>().ok()))
            .unwrap_or(1)
    };

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
                        <div class="spinner-grow ms-auto" role="status" aria-hidden="true"></div>
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
}

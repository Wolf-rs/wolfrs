use leptos::*;
use leptos_router::*;
use markdown::*;

use crate::api::community::get_community;
use crate::api::structs::*;
use crate::api::*;
use crate::components::instance::*;

#[component]
pub fn Sidecard(cx: Scope) -> impl IntoView {
    let query = use_query_map(cx);
    let page = move || {
        query
            .with(|q| q.get("page").and_then(|page| page.parse::<i32>().ok()))
            .unwrap_or(1)
    };

    let params = use_params_map(cx);
    let community_name = move || {
        params
            .with(|params| params.get("community_name").cloned())
            .unwrap_or_default()
            .parse::<String>()
            .unwrap()
    };

    let sidebar = create_resource(cx, page, move |page| async move {
        // This constructs the proper API URL for GetPosts or GetPersonDetails
        let url_constructor = ApiUrlConstructor {
            endpoint: api_endpoints::GetEndpoint::GET_COMMUNITY.to_string(),
            id: None,
            params: None,
        };

        let get_form = GetCommunity {
            auth: None,
            id: None,
            name: Some(community_name()),
        };

        get_community(cx, &api_url_builder(cx, url_constructor, get_form))
            .await
            .ok()
    });

    let err_msg = "Error loading this post: ";

    view! { cx,
        <div class="text-left">
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
                                let sidebar = match res.community_view.community.description.clone()
                                {
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
                                        // Community sidecard
                                        <div class="card text-left">
                                            <div class="card-header">
                                                <img
                                                    src=res.community_view.community.banner
                                                    class="card-img-top"
                                                    alt="Banner for the Lemmy instance"
                                                />
                                            </div>
                                            <div class="card-body">
                                                <h4 class="card-title text-center">
                                                    {res.community_view.community.title}
                                                </h4>
                                                <hr/>
                                                <div inner_html=sidebar></div>
                                                <hr/>
                                                <h6>"Moderators"</h6>
                                                <ul class="list-group list-group-flush">
                                                    {res
                                                        .moderators
                                                        .clone()
                                                        .into_iter()
                                                        .map(|moderator| {
                                                            view! { cx,
                                                                // Checks to see if a user has an avatar set, if not it assigns a default one

                                                                {
                                                                    let admin_avatar = match moderator.moderator.avatar {
                                                                        Some(_) => moderator.moderator.avatar,
                                                                        _ => {
                                                                            Option::Some(
                                                                                "/static/default_assets/default-profile.png".to_string(),
                                                                            )
                                                                        }
                                                                    };
                                                                    let moderator_link = if moderator.moderator.local {
                                                                        format!("/user/{}", moderator.moderator.name)
                                                                    } else {
                                                                        let url_regex = regex::Regex::new(r"https://(.*)/u/(.*)")
                                                                            .unwrap();
                                                                        let external_creator_link = match url_regex
                                                                            .captures(&moderator.moderator.actor_id)
                                                                        {
                                                                            Some(external_creator_link) => external_creator_link,
                                                                            None => {
                                                                                println!(
                                                                                    "url regex error: {:?}", & moderator.moderator.actor_id
                                                                                );
                                                                                url_regex
                                                                                    .captures(&moderator.moderator.actor_id)
                                                                                    .unwrap_or_else(|| {
                                                                                        url_regex.captures("https://error.com/u/error").unwrap()
                                                                                    })
                                                                            }
                                                                        };
                                                                        format!(
                                                                            "/user/{}@{}", moderator.moderator.name, &
                                                                            external_creator_link[1]
                                                                        )
                                                                    };

                                                                    view! { cx,
                                                                        <li class="list-group-item">
                                                                            <a href=format!("{}", moderator_link)>
                                                                                <img
                                                                                    src=admin_avatar
                                                                                    alt="mdo"
                                                                                    width="32"
                                                                                    height="32"
                                                                                    class="rounded"
                                                                                />
                                                                                "  "
                                                                                {moderator.moderator.name}
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
                                        // Community Statistics Card
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
                                                            <td>
                                                                {format!("{} Users", res.community_view.counts.subscribers)}
                                                            </td>
                                                            <td>
                                                                <div class="vr"></div>
                                                            </td>
                                                            <td>
                                                                {format!("{} Posts", res.community_view.counts.posts)}
                                                            </td>
                                                        </tr>
                                                        <tr>
                                                            <td>
                                                                {format!(
                                                                    "{} / Day", res.community_view.counts.users_active_day
                                                                )}

                                                            </td>
                                                            <td>
                                                                <div class="vr"></div>
                                                            </td>
                                                            <td>
                                                                {format!("{} Comments", res.community_view.counts.comments)}
                                                            </td>
                                                        </tr>
                                                        <tr>
                                                            <td>
                                                                {format!(
                                                                    "{} / Month", res.community_view.counts.users_active_month
                                                                )}

                                                            </td>
                                                            <td>
                                                                <div class="vr"></div>
                                                            </td>
                                                            <td>
                                                                // This should eventually become the rank of the community on the Instance
                                                                // This will be for another time...
                                                                {format!("{:?} Moderators", res.moderators.clone().len())}

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

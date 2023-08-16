use leptos::*;
use markdown::*;

use crate::api::structs::*;

#[component]
pub fn Sidecard(cx: Scope, sidebar: Resource<i32, Option<GetPostResponse>>) -> impl IntoView {
    let err_msg = "Error loading sidecard: ";

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
                                let banner_image = match res.community_view.community.banner {
                                    Some(_) => res.community_view.community.banner,
                                    _ => {
                                        Option::Some(
                                            "/static/default_assets/default-community.png".to_string(),
                                        )
                                    }
                                };

                                view! { cx,
                                    <div>
                                        // Community sidecard
                                        <div class="card text-left">
                                            <div class="card-header">
                                                <img
                                                    src=banner_image
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

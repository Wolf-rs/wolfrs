use leptos::*;
use markdown::*;

use crate::api::structs::*;
use crate::components::instance::*;

#[component]
pub fn Sidecard(
    cx: Scope,
    sidebar: Resource<i32, Option<GetPersonDetailsResponse>>,
) -> impl IntoView {
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
                                let sidebar = match res.person_view.person.bio.clone() {
                                    Some(text) => {
                                        markdown::to_html_with_options(
                                                text.as_str(),
                                                &Options::gfm(),
                                            )
                                            .unwrap()
                                    }
                                    None => "".to_string(),
                                };

                                let icon_image = match res.person_view.person.avatar {
                                    Some(_) => res.person_view.person.avatar,
                                    _ => {
                                        Option::Some(
                                            "/static/default_assets/default-community.png".to_string(),
                                        )
                                    }
                                };

                                let banner_image = match res.person_view.person.banner {
                                    Some(_) => res.person_view.person.banner,
                                    _ => {
                                        Option::Some(
                                            "/static/default_assets/banners/wolfrs-default-user-banner.png".to_string(),
                                        )
                                    }
                                };

                                view! { cx,
                                    <div>
                                        // User sidecard
                                        <div class="card text-left">
                                        <div class="card-header">
                                        <img
                                            src=banner_image
                                            class="card-img-top"
                                            alt="Community banner"
                                        />
                                    </div>
                                    <div class="card-body">
                                        <img
                                            src=icon_image
                                            class="img-fluid rounded"
                                            alt="Icon for the community"
                                        />
                                        <br/>
                                        <br/>

                                                {if !res.person_view.person.display_name.is_none() {
                                                    view! { cx,
                                                        <h4 class="card-title text-center">
                                                            {res.person_view.person.display_name}
                                                        </h4>
                                                    }
                                                } else {

                                                    view! { cx,
                                                        <h4 class="card-title text-center text-nowrap">
                                                            {res.person_view.person.name}
                                                        </h4>
                                                    }
                                                }}
                                                <hr/> <div class="markdown" inner_html=sidebar></div> <hr/>
                                                {if !res.moderates.is_empty() {
                                                    view! { cx,
                                                        <div>
                                                            <h6>"Moderates"</h6>
                                                            <ul class="list-group list-group-flush">
                                                                {res
                                                                    .moderates
                                                                    .clone()
                                                                    .into_iter()
                                                                    .map(|community| {
                                                                        view! { cx,
                                                                            // Checks to see if a user has an avatar set, if not it assigns a default one

                                                                            {
                                                                                let community_avatar = match community.community.icon {
                                                                                    Some(_) => community.community.icon,
                                                                                    _ => {
                                                                                        Option::Some(
                                                                                            "/static/default_assets/default-profile.png".to_string(),
                                                                                        )
                                                                                    }
                                                                                };
                                                                                let moderator_link = if community.community.local {
                                                                                    format!("/community/{}", community.community.name)
                                                                                } else {
                                                                                    let url_regex = regex::Regex::new(r"https://(.*)/c/(.*)")
                                                                                        .unwrap();
                                                                                    let external_creator_link = match url_regex
                                                                                        .captures(&community.community.actor_id)
                                                                                    {
                                                                                        Some(external_creator_link) => external_creator_link,
                                                                                        None => {
                                                                                            println!(
                                                                                                "url regex error: {:?}", & community.community.actor_id
                                                                                            );
                                                                                            url_regex
                                                                                                .captures(&community.community.actor_id)
                                                                                                .unwrap_or_else(|| {
                                                                                                    url_regex.captures("https://error.com/u/error").unwrap()
                                                                                                })
                                                                                        }
                                                                                    };
                                                                                    format!(
                                                                                        "/community/{}@{}", community.community.name, &
                                                                                        external_creator_link[1]
                                                                                    )
                                                                                };

                                                                                view! { cx,
                                                                                    <li class="list-group-item">
                                                                                        <a href=format!("{}", moderator_link)>
                                                                                            <img
                                                                                                src=community_avatar
                                                                                                alt="mdo"
                                                                                                width="32"
                                                                                                height="32"
                                                                                                class="rounded"
                                                                                            />
                                                                                            "  "
                                                                                            {community.community.title}
                                                                                        </a>
                                                                                    </li>
                                                                                }
                                                                            }
                                                                        }
                                                                    })
                                                                    .collect::<Vec<_>>()}
                                                            </ul>
                                                        </div>
                                                    }
                                                } else {
                                                    // Checks to see if a user has an avatar set, if not it assigns a default one

                                                    // Checks to see if a user has an avatar set, if not it assigns a default one

                                                    // Checks to see if a user has an avatar set, if not it assigns a default one

                                                    // Checks to see if a user has an avatar set, if not it assigns a default one

                                                    view! { cx, <div></div> }
                                                }}

                                            </div>
                                        </div>
                                        <br/>
                                        // User Statistics
                                        <div class="card text-left">
                                            <div class="card-header">
                                                <h5 class="card-title text-center">"Statistics"</h5>
                                            </div>
                                            <div class="card-body">
                                                <table class="table table-dark">
                                                    <thead>
                                                        <tr>
                                                            <th scope="col">"Activity"</th>
                                                            <th scope="col">
                                                                <div class="vr"></div>
                                                            </th>
                                                            <th scope="col">"Karma"</th>
                                                        </tr>
                                                    </thead>
                                                    <tbody>
                                                        <tr>
                                                            <td>
                                                                {format!("{} Posts", res.person_view.counts.post_count)}
                                                            </td>
                                                            <td>
                                                                <div class="vr"></div>
                                                            </td>
                                                            <td>
                                                                {format!("{} Karma", res.person_view.counts.post_score)}
                                                            </td>
                                                        </tr>
                                                        <tr>
                                                            <td>
                                                                {format!(
                                                                    "{} Comments", res.person_view.counts.comment_count
                                                                )}

                                                            </td>
                                                            <td>
                                                                <div class="vr"></div>
                                                            </td>
                                                            <td>
                                                                {format!("{} Karma", res.person_view.counts.comment_score)}
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

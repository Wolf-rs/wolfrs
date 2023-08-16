use leptos::*;

use crate::components::instance::*;

// The Footer for the site
#[component]
pub fn Footer(cx: Scope) -> impl IntoView {
    let logo_source = match get_instance_details().unwrap().no_logo_text {
        true => get_instance_details().unwrap().logo_name,
        false => get_instance_details().unwrap().logo_text_name,
    };
    let width = get_instance_details().unwrap().logo_width;
    let height = get_instance_details().unwrap().logo_height;
    let name = get_instance_details().unwrap().name;
    let slogan = get_instance_details().unwrap().slogan;
    let documentation_url = get_instance_details().unwrap().documentation_url;
    let source_code_url = get_instance_details().unwrap().source_code_url;

    view! { cx,
        <div class="container">
            <footer class="d-flex flex-wrap justify-content-between align-items-center py-3 my-4 border-top">
                <p class="col-md-4 mb-0 text-body-secondary">{format!("{}: {}", name, slogan)}</p>

                <a class="navbar-brand" href="#">
                    <img
                        src=format!("/assets/{}", logo_source)
                        alt="Logo"
                        width=format!("{}", width)
                        height=format!("{}", height)
                        class="d-inline-block align-text-top"
                    />
                </a>

                <ul class="nav col-md-5 justify-content-end">
                    <li class="nav-item">
                        <a href="/modlog" class="nav-link px-2 text-body-secondary">
                            {"Modlog"}
                        </a>
                    </li>
                    <li class="nav-item">
                        <a href="/federation" class="nav-link px-2 text-body-secondary">
                            {"Federation"}
                        </a>
                    </li>
                    <li class="nav-item">
                        <a
                            href=documentation_url.to_string()
                            class="nav-link px-2 text-body-secondary"
                        >
                            {"Documentation"}
                        </a>
                    </li>
                    <li class="nav-item">
                        <a
                            href=source_code_url.to_string()
                            class="nav-link px-2 text-body-secondary"
                        >
                            {"Source Code"}
                        </a>
                    </li>
                    <li class="nav-item">
                        <a href="https://join-lemmy.org/" class="nav-link px-2 text-body-secondary">
                            {"Lemmy Info"}
                        </a>
                    </li>
                </ul>
            </footer>
        </div>
    }
}

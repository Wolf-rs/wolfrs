use leptos::*;

use crate::components::instance::*;

// The Navbar Header for the UI
#[component]
pub fn Header(cx: Scope) -> impl IntoView {
    let logo_source = match get_instance_details().unwrap().no_logo_text {
        true => get_instance_details().unwrap().logo_name,
        false => get_instance_details().unwrap().logo_text_name,
    };
    let width = get_instance_details().unwrap().logo_width;
    let height = get_instance_details().unwrap().logo_height;
    let name = match get_instance_details().unwrap().no_logo_text {
        true => get_instance_details().unwrap().name,
        false => "".to_string(),
    };
    //TODO: Change how default image is handled
    let temp_img_src = "/assets/favicon.ico";

    view! { cx,
        <nav class="navbar navbar-expand-lg bg-body-tertiary">
            <div class="container-fluid">
                <a class="navbar-brand" href="/">
                    <img src={format!("/assets/{}", logo_source)} alt="Logo" width={format!("{}", width)} height={format!("{}", height)} class="d-inline-block align-text-top" />
                    {name}
                </a>
                <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarSupportedContent" aria-controls="navbarSupportedContent" aria-expanded="false" aria-label="Toggle navigation">
                <span class="navbar-toggler-icon"></span>
                </button>
                <div class="collapse navbar-collapse" id="navbarSupportedContent">
                    <ul class="navbar-nav me-auto mb-2 mb-lg-0">
                        <li class="nav-item dropdown">
                        <a class="nav-link dropdown-toggle" href="#" role="button" data-bs-toggle="dropdown" aria-expanded="false">
                            {"Menu"}
                        </a>
                        <ul class="dropdown-menu">
                            <li><a class="dropdown-item" href="/communities">{"Explore Communities"}</a></li>
                            <li><hr class="dropdown-divider" /></li>
                            <li><a class="dropdown-item" href="/create_post">{"Create Post"}</a></li>
                            <li><a class="dropdown-item" href="/create_community">{"Create Communitiy"}</a></li>
                        </ul>
                        </li>
                    </ul>
                    <div class="row">
                        <div class="col">
                            <a class="btn btn-primary" href="/search" role="button">
                                <div class="row">
                                    <div class="col col-sm-12">
                                        <i class="bi bi-search"></i>
                                    </div>
                                    <div class="col col-sm-0"></div>
                                </div>
                            </a>
                        </div>
                        <div class="col">
                            <a class="btn btn-primary" href="/notifications" role="button">
                                <div class="row">
                                    <div class="col col-sm-12">
                                        <i class="bi bi-bell"></i>
                                    </div>
                                    <div class="col col-sm-0">
                                        {""}
                                    </div>
                                </div>
                            </a>
                        </div>
                        <div class="col">

                            //TODO: Improve button design and handling, possibly to own component
                            <div class="dropdown text-end">
                                <button class="btn btn-secondary btn-sm dropdown-toggle" type="button" data-bs-toggle="dropdown" aria-expanded="false">
                                    <img src={temp_img_src.to_string()} alt="mdo" width="32" height="32" class="rounded-circle" />
                                    {"Sign In/Create Account"}
                                </button>
                                <ul class="dropdown-menu text-small">
                                    <li><a class="dropdown-item" href="/user/login">{"Profile"}</a></li>
                                    <li><a class="dropdown-item" href="/settings">{"Settings"}</a></li>
                                    <li><hr class="dropdown-divider" /></li>
                                    <li><a class="dropdown-item" href="#">{"Sign out"}</a></li>
                                </ul>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </nav>
    }
}

use crate::error_template::{AppError, ErrorTemplate};
use leptos::{html::Header, *};
use leptos_meta::*;
use leptos_router::*;

use crate::components::{footer::Footer, header::Header, instance::*};
use crate::pages::home::Home;

// Boilerplate for when the pages are created
// use crate::pages::{
//     communities::Communities, community::Community, create_community::CreateCommunity,
//     create_post::CreatePost, home::Home, instances::Instances, login::Login, mod_log::ModLog,
//     notifications::Notifications, post::Post, reports::Reports, search::Search, settings::Settings,
//     user::User,
// };

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    // Get the instance details from the Instance.toml file and assign them to variables for use in the template
    let favicon = get_instance_details().unwrap().favicon_name;
    let title = get_instance_details().unwrap().name;
    let slogan = get_instance_details().unwrap().slogan;

    view! {
        cx,

        // Sets the Bootstrap dark-theme attribute
        <Html attributes=AdditionalAttributes::from(vec![("data-bs-theme", "dark")]) />

        // Sets the favicon, which seems to work best as an .ico file
        <Link rel="icon" type_="image/x-icon" href={format!("/assets/{}", favicon)} />

        // Inject the Bootstrap stylesheet into the <head> of the page
        // The id=leptos means cargo-leptos will hot-reload this stylesheet, which may or may not be required due to being loaded from a CDN...
        <Stylesheet href="https://cdn.jsdelivr.net/npm/bootstrap-icons@1.10.2/font/bootstrap-icons.css"/>
        <Stylesheet id="leptos" href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.0/dist/css/bootstrap.min.css"/>
        <Style>"body {display: flex; min-height: 100vh; flex-direction: column;} main {flex: 1 0 auto;}"</Style>

        // Dynamically sets the title of the page based on the name and slogan defined in the Instance.toml file
        <Title text={format!("{}: {}", title, slogan)}/>

        // Primary Router for the site
        <Router fallback=|cx| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { cx,
                <ErrorTemplate outside_errors/>
            }
            .into_view(cx)
        }>

            <Header />
            <main>
                <Routes>
                    <Route path="/" view=|cx| view! { cx, <Home/> }/>
                    // <Route path="/communities" view=|cx| view! { cx, <Communities/> }/>
                    // <Route path="/community" view=|cx| view! { cx, <Community/> }/>
                    // <Route path="/create_community" view=|cx| view! { cx, <CreateCommunity/> }/>
                    // <Route path="/create_post" view=|cx| view! { cx, <CreatePost/> }/>
                    // <Route path="/instances" view=|cx| view! { cx, <Instances/> }/>
                    // <Route path="/login" view=|cx| view! { cx, <Login/> }/>
                    // <Route path="/mod_log" view=|cx| view! { cx, <ModLog/> }/>
                    // <Route path="/notifications" view=|cx| view! { cx, <Notifications/> }/>
                    // <Route path="/post/:id" view=|cx| view! { cx, <Post/> }/>
                    // <Route path="/reports" view=|cx| view! { cx, <Reports/> }/>
                    // <Route path="/search" view=|cx| view! { cx, <Search/> }/>
                    // <Route path="/settings" view=|cx| view! { cx, <Settings/> }/>
                    // <Route path="/user/:username" view=|cx| view! { cx, <User/> }/>
                </Routes>
            </main>
            <Footer />
        </Router>

        // Inject the Bootstrap JavaScript into the bottom of the <body> of each page
        <script src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.0/dist/js/bootstrap.bundle.min.js" integrity="sha384-geWF76RCwLtnZ8qwWowPQNguL3RmwHVBC9FhGdlKrxdiJJigb/j/68SIy3Te4Bkz" crossorigin="anonymous"></script>
    }
}

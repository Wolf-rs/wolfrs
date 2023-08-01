use leptos::*;
use leptos_router::*;

// TODO: Improve handling of clicked buttons
// Improve potential styling

// Pagination component for pages that require it
#[component]
pub fn Pagination(cx: Scope) -> impl IntoView {
    let query = use_query_map(cx);
    let page = move || {
        query
            .with(|q| q.get("page").and_then(|page| page.parse::<i32>().ok()))
            .unwrap_or(1)
    };

    view! { cx,
        <nav aria-label="Feed Page Navigation">
            <ul class="pagination justify-content-center">
                {move || if page() > 1 {
                    view! { cx, <li class="page-item"><A class="page-link" href=move || format!("?page={}", page() - 1) >Previous</A></li>}
                } else {
                    view! { cx, <li class="page-item disabled"><A class="page-link" href="" >Previous</A></li>}
                }}
                <li class="page-item"><A class="page-link" href=move || format!("")>Page: {page}</A></li>
                <li class="page-item"><A class="page-link" href=move || format!("?page={}", page() + 1) >Next</A></li>
            </ul>
        </nav>
    }
}

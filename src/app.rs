use leptos::*;
use leptos_meta::*;
use leptos_router::*;
#[component]
pub fn App() -> impl IntoView {
  provide_meta_context();

  view! {
      <Title text="Welcome to Leptos"/>
      <Stylesheet id="leptos" href="./pkg/leptos_ssr_test_1.css"/>
      <Router>
          <main>
              <Routes>
                  <Route path="" view=Home/>
                  <Route path="/*any" view=NotFound/>
              </Routes>
          </main>
      </Router>
  }
}

#[component]
fn Home() -> impl IntoView {
  view! { <h1>"Welcome to Leptos!"</h1> }
}

#[component]
fn NotFound() -> impl IntoView {
  // set an HTTP status code 404
  // this is feature gated because it can only be done during
  // initial server-side rendering
  // if you navigate to the 404 page subsequently, the status
  // code will not be set because there is not a new HTTP request
  // to the server
  #[cfg(feature = "ssr")]
  {
    // this can be done inline because it's synchronous
    // if it were async, we'd use a server function
    let resp = expect_context::<leptos_actix::ResponseOptions>();
    resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
  }

  view! { <h1>"Not Found"</h1> }
}


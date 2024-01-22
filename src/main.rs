mod app;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  use actix_web::web::Data;
  use app::App;
  use leptos_actix::LeptosRoutes;

  let conf = leptos::get_configuration(Some("Cargo.toml")).await.unwrap();
  let addr = conf.leptos_options.site_addr;
  let routes = leptos_actix::generate_route_list(App);
  actix_web::HttpServer::new(move || {
    let leptos_options = &conf.leptos_options;
    actix_web::App::new()
      .service(favicon)
      .leptos_routes(leptos_options.to_owned(), routes.to_owned(), App)
      .app_data(Data::new(conf.leptos_options.to_owned()))
  })
  .bind(&addr)?
  .run()
  .await
}

#[actix_web::get("/favicon.ico")]
async fn favicon(leptos_options: actix_web::web::Data<leptos::LeptosOptions>) -> impl actix_web::Responder {
  let leptos_options = leptos_options.into_inner();
  let site_root = &leptos_options.site_root;
  actix_files::NamedFile::open_async(format!("{site_root}/favicon.ico")).await
}


use rocket::routes;

#[derive(askama::Template)]
#[template(source = "<h1>Hello, World!</h1>", ext="html")]
struct Template {

}

#[rocket::get("/index.html")]
fn index() -> Template {
    Template {} 
}

#[rocket::launch]
fn launch() -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .mount("/", routes![index])
}

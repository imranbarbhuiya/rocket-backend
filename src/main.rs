use rocket::{
    http::{Cookie, CookieJar},
    response::{content::RawHtml, Flash, Redirect},
};

#[macro_use]
extern crate rocket;

#[get("/")]
fn user_id(cookies: &CookieJar<'_>) -> RawHtml<String> {
    RawHtml(
        cookies
            .get_private("user_id")
            .map(|crumb| format!("User ID: {}. <a href=\"/logout\">logout</a>", crumb.value()))
            .unwrap_or("Not logged in. Log in <a href=\"/login/123\">here</a>.".into()),
    )
}

#[get("/login/<user_id>")]
fn login(cookies: &CookieJar<'_>, user_id: String) -> RawHtml<String> {
    let html = RawHtml(format!(
        "Successfully logged in. User ID: {}. Go <a href=\"/\">home page</a>.",
        &user_id
    ));

    cookies.add_private(Cookie::new("user_id", user_id));

    html
}

#[get("/logout")]
fn logout(cookies: &CookieJar<'_>) -> Flash<Redirect> {
    cookies.remove_private(Cookie::named("user_id"));
    Flash::success(Redirect::to("/"), "Successfully logged out.")
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![user_id, login, logout])
}

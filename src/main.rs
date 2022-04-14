#[macro_use]
extern crate rocket;
use rocket::http::{Cookie, CookieJar};
use rocket::request::{FromRequest, Outcome, Request};

struct User {
    user_type: String,
}
struct Admin {
    user_type: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<User, ()> {
        let user_type = request
            .cookies()
            .get("loggedInUser")
            .map(|cookie| cookie.value());
        match user_type {
            None => Outcome::Forward(()),
            Some(user_type) => {
                if user_type == "user" {
                    Outcome::Success(User {
                        user_type: String::from("user"),
                    })
                } else {
                    Outcome::Forward(())
                }
            }
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Admin {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Admin, ()> {
        let user_type = request
            .cookies()
            .get("loggedInUser")
            .map(|cookie| cookie.value());
            match user_type {
                None => Outcome::Forward(()),
                Some(user_type) => {
                    if user_type == "admin" {
                        Outcome::Success(Admin {
                            user_type: String::from("admin"),
                        })
                    } else {
                        Outcome::Forward(())
                    }
                }
            }
        
    }
}

#[get("/")]
fn index() -> &'static str {
    "Is color orange?"
}
#[get("/orange/<r>/<g>/<b>")]
fn is_orange(r: &str, g: &str, b: &str) -> &'static str {
    print!("{}, {}, {}", r, g, b);
    "is Orange"
}
#[get("/login")]
fn login(jar: &CookieJar<'_>) {
    jar.add(Cookie::new("loggedInUser", "user"))
}
#[get("/loginAdmin")]
fn login_admin(jar: &CookieJar<'_>) {
    jar.add(Cookie::new("loggedInUser", "admin"))
}
#[get("/logout")]
fn logout(jar: &CookieJar<'_>) {
    jar.remove(Cookie::named("loggedInUser"))
}
#[get("/dashboard")]
fn admin_dashboard(admin: Admin) -> String { format!("{} dashboard", admin.user_type) }

#[get("/dashboard", rank = 2)]
fn user_dashboard(user: User) -> String { format!("{} dashboard", user.user_type) }

#[get("/dashboard", rank = 3)]
fn public_dashboard() -> &'static str{ "public dashboard" }

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, is_orange, login,logout, login_admin, admin_dashboard, user_dashboard, public_dashboard])
}

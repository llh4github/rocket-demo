#[macro_use]
extern crate rocket;

mod apis;
mod config;
mod entity;
mod test;
#[launch]
fn rocket() -> _ {
    // config::init_server()
    launch_openapi()
}

async fn launch_openapi() {
    rocket::build()
        .mount(
            "/",
            openapi_get_routes![get_all_users, get_user, get_user_by_name, create_user,],
        )
        .launch()
        .await
}
#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct User {
    /// A unique user identifier.
    user_id: u64,
    /// The current username of the user.
    username: String,
    #[schemars(example = "example_email")]
    email: Option<String>,
}

fn example_email() -> &'static str {
    "test@example.com"
}

/// # Get all users
///
/// Returns all users in the system.
#[openapi(tag = "Users")]
#[get("/user")]
fn get_all_users() -> Json<Vec<User>> {
    Json(vec![User {
        user_id: 42,
        username: "bob".to_owned(),
        email: None,
    }])
}

/// # Get user
///
/// Returns a single user by ID.
#[openapi(tag = "Users")]
#[get("/user/<id>")]
fn get_user(id: u64) -> Option<Json<User>> {
    Some(Json(User {
        user_id: id,
        username: "bob".to_owned(),
        email: None,
    }))
}

/// # Get user by name
///
/// Returns a single user by username.
#[openapi(tag = "Users")]
#[get("/user_example?<user_id>&<name>&<email>")]
fn get_user_by_name(user_id: u64, name: String, email: Option<String>) -> Option<Json<User>> {
    Some(Json(User {
        user_id,
        username: name,
        email,
    }))
}

/// # Create user
#[openapi(tag = "Users")]
#[post("/user", data = "<user>")]
fn create_user(user: Json<User>) -> Json<User> {
    user
}

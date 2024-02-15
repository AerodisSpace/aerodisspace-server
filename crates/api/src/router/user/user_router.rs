use rocket::{routes, Route};

use crate::router::user::free::{test_token, user_route_login, user_route_search, user_route_signup};

pub fn user_router() -> Vec<Route> {
    let mut routes = Vec::new();
    let free_routes = routes![user_route_signup, user_route_login, user_route_search, test_token];
    let auth_routes = routes![];
    routes.extend(free_routes);
    routes.extend(auth_routes);

    routes
}

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::uri::Origin;
use rocket::http::Method;
pub struct JwtFairing;

#[rocket::async_trait]
impl Fairing for JwtFairing {
    fn info(&self) -> Info {
        Info {
            name: "JWT fairing",
            kind: Kind::Request,
        }
    }

    async fn on_request(&self, req: &mut rocket::Request<'_>, _data: &mut rocket::Data<'_>) {
        let open_routes = ["/login", "/"];
        let request_path = req.uri().path();

        if open_routes.contains(&request_path.as_str()) {
            return;
        }

        let token_valid = req
            .headers()
            .get_one("Authorization")
            .and_then(|header| header.strip_prefix("Bearer "))
            .map(|token| verificar_token(token))
            .unwrap_or(false);

        if !token_valid {
            if let Ok(uri) = Origin::parse("/unauthorized") {
                req.set_uri(uri);
                req.set_method(Method::Get)
            }
        }
    }
}

fn verificar_token(token: &str) -> bool {
    token != "1231"
}

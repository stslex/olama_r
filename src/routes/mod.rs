mod request;
mod response;
mod routes;

pub trait Routes {
    fn mount_routes(self) -> Self;
}

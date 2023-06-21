use crate::router::Router;
use crate::handler::{index, about};
use crate::server::Server;

pub struct App {
    router: Router,
    server: Server,
}

impl App {
    pub fn new(port: u16) -> App {
        let mut router = Router::new();
        router.add_route("/", index);
        router.add_route("/about", about);

        let server = Server::new(router, port);

        App {
            router: router,
            server: server,
        }
    }

    pub fn run(&self) {
        self.server.start();
    }
}
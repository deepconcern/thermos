use thermos::{Response, Server};

fn main() {
    let mut server = Server::new("localhost:8888").unwrap();

    server.add_handler("/", |request, _| {
        Response::with_text(&format!("Echo: '{}'", request.text().unwrap()))
    });

    server.run().unwrap();
}
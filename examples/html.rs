use thermos::{Response, Server, Status};

fn main() {
    let mut server = Server::new("localhost:8888").unwrap();

    server.add_handler("/", |_, _| {
        match Response::with_file("examples/assets/hello.html") {
            Ok(r) => r,
            Err(e) => {
                eprintln!("{}", e);

                Response::with_status(Status::InternalServerError)
            }
        }
    });

    server.run().unwrap();
}
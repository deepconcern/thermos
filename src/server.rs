use std::{
    io::{self, BufReader, BufWriter, Write},
    net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener, TcpStream, ToSocketAddrs},
};

use crate::{
    RequestReadError, Status,
    context::Context,
    handler::Handler,
    http::{Request, response::Response},
    route::RouteNode,
};

fn not_found(_: Request, _: Context) -> Response {
    Response::with_status(Status::NotFound)
}

#[derive(Debug)]
pub enum ServerError {
    BindError(io::Error),
    RequestReadError(RequestReadError),
    ResponseWriteError(io::Error),
}

pub struct Server {
    addr: SocketAddr,
    root: RouteNode,
}

impl Server {
    pub fn new<A>(addr: A) -> Result<Self, io::Error>
    where
        A: ToSocketAddrs,
    {
        Ok(Self {
            addr: addr.to_socket_addrs()?.next().unwrap(),
            ..Default::default()
        })
    }

    fn handle_incoming(
        &self,
        reader_stream: BufReader<TcpStream>,
    ) -> Result<Response, ServerError> {
        let request =
            Request::from_stream(reader_stream).map_err(|e| ServerError::RequestReadError(e))?;

        println!("NOW {} {} {}", request.method, request.version, request.uri);

        let context = Context::default();

        let response = match self.root.search(&request.uri.path) {
            Some(handler) => (handler)(request, context),
            None => not_found(request, context),
        };

        Ok(response)
    }

    pub fn add_handler(&mut self, path: &str, handler: Handler) {
        self.root.add_handler(path, handler);
    }

    pub fn run(&self) -> Result<(), ServerError> {
        let listener = TcpListener::bind(self.addr).map_err(|e| ServerError::BindError(e))?;

        for incoming in listener.incoming() {
            let tcp_stream = match incoming {
                Ok(tcp_stream) => tcp_stream,
                Err(error) => {
                    eprintln!("{}", error);
                    continue;
                }
            };

            let reader_stream = BufReader::new(match tcp_stream.try_clone() {
                Ok(t) => t,
                Err(error) => {
                    eprintln!("{}", error);
                    continue;
                }
            });

            let response = self
                .handle_incoming(reader_stream)
                .unwrap_or_else(|server_error| {
                    eprintln!("{:?}", server_error);

                    Response::with_status(Status::InternalServerError)
                });

            let response_bytes = response
                .bytes()
                .map_err(|e| ServerError::ResponseWriteError(e))?;

            BufWriter::new(tcp_stream)
                .write_all(&response_bytes)
                .unwrap_or_else(|error| {
                    eprintln!("{:?}", error);
                });
        }

        Ok(())
    }
}

impl Default for Server {
    fn default() -> Self {
        Self {
            addr: SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 8888)),
            root: RouteNode::default(),
        }
    }
}

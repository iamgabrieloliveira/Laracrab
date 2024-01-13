use std::{
    fmt::Display,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    str::FromStr,
};

#[derive(Debug)]
enum Status {
    Ok = 200,
}

#[derive(Debug)]
struct Request<'a> {
    url: &'a str,
    method: Method,
}

#[derive(Debug)]
struct Response {
    status: Status,
}

type Handler = fn(request: Request) -> Response;

#[derive(Debug, PartialEq, Eq)]
enum Method {
    GET,
    POST,
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for Method {
    type Err = ();

    fn from_str(input: &str) -> Result<Method, Self::Err> {
        match input {
            "GET" => Ok(Method::GET),
            "POST" => Ok(Method::POST),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Route<'a> {
    method: Method,
    url: &'a str,
    handler: Handler,
}

#[derive(Debug)]
struct Router<'a> {
    routes: Vec<Route<'a>>,
}

impl<'a> Router<'a> {
    fn new() -> Router<'a> {
        return Router { routes: Vec::new() };
    }

    fn get(&mut self, url: &'a str, handler: Handler) -> () {
        self.add_route(Method::GET, url, handler);
    }

    fn post(&mut self, url: &'a str, handler: Handler) -> () {
        self.add_route(Method::POST, url, handler);
    }

    fn add_route(&mut self, method: Method, url: &'a str, handler: Handler) -> () {
        let route = Route {
            method,
            url,
            handler,
        };
        self.routes.push(route);
    }

    fn listen(&mut self, address: &str) {
        let listener = TcpListener::bind(address).unwrap();

        for stream in listener.incoming() {
            let stream = stream.unwrap();

            self.handle_connection(stream);
        }
    }

    fn handle_connection(&mut self, mut stream: TcpStream) -> () {
        let reader = BufReader::new(&mut stream);
        let mut request_lines = reader.lines();
        let raw_url = request_lines.next().unwrap().unwrap();

        let mut request_uri = raw_url.split_whitespace().into_iter();

        let method = request_uri.next().unwrap();
        let url = request_uri.next().unwrap();
        let http_version = request_uri.next().unwrap();

        let request = Request {
            method: Method::from_str(method).unwrap(),
            url,
        };

        println!(
            "Request: method={:?} url={} http-version={}",
            method, url, http_version
        );

        let route = self
            .routes
            .iter()
            .find(|route| route.url == request.url && route.method == request.method);

        match route {
            Some(route) => {
                let _response = (route.handler)(request);
                todo!();
            }
            None => {
                println!("Route not found");
            }
        }
    }
}

fn main() {
    let mut router = Router::new();

    router.get("/", |_request| Response { status: Status::Ok });
    router.post("/", |_request| Response { status: Status::Ok });

    router.listen("127.0.0.1:6969");
}

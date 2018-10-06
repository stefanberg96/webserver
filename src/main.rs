
extern crate clap;
extern crate hyper;

use clap::{App,Arg};
use hyper::{Body, Request, Response, Server};
use hyper::rt::Future;
use hyper::service::service_fn_ok;
use std::str::FromStr;



fn main() {


    let matches = App::new("My default_web_app")
        .version("0.1")
        .author("Stefan")
        .about("Listens on ip  and responds with hello world.")
        .arg(Arg::with_name("ip")
            .long("ip")
            .required(true)
            .help("Sets the port to listen on")
            .takes_value(true)).get_matches();

    let addr = std::net::SocketAddr::from_str(matches.value_of("ip").unwrap()).unwrap();

    // A `Service` is needed for every connection, so this
    // creates on of our `hello_world` function.
    let new_svc = || {
        // service_fn_ok converts our function into a `Service`
        service_fn_ok(handle_connection)
    };

    let server = Server::bind(&addr)
        .serve(new_svc)
        .map_err(|e| eprintln!("server error: {}", e));

    // Run this server for... forever!
    hyper::rt::run(server);
}




fn handle_connection(req: Request<Body>) -> Response<Body>  {
    println!("Receveid a request {:?}", req);
    let resp = Response::new(Body::from("Hello world"));
    println!("Send the information back to the request");
    return resp;
}
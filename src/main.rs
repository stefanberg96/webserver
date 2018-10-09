extern crate clap;
extern crate hyper;
extern crate lettre;
extern crate lettre_email;
extern crate native_tls;

use clap::{App, Arg};
use hyper::{Body, Request, Response, Server};
use hyper::rt::Future;
use hyper::service::service_fn_ok;
use std::str::FromStr;
use lettre::EmailTransport;
use lettre::smtp;
use lettre_email::EmailBuilder;
use lettre::ClientTlsParameters;
use native_tls::TlsConnector;


fn main() {
    let matches = App::new("My default_web_app")
        .version("0.1")
        .author("Stefan")
        .about("Listens on ip  and responds with hello world.")
        .arg(Arg::with_name("ip")
            .long("ip")
            .required(true)
            .help("Sets the port to listen on")
            .takes_value(true))
        .arg(Arg::with_name("username")
            .short("u")
            .help("username for the gmail account")
            .takes_value(true)
            .required(true)
        )
        .arg(Arg::with_name("password")
            .short("p")
            .help("password of the gmail account")
            .takes_value(true)
            .required(true)
        ).get_matches();

    let username = String::from(matches.value_of("username").unwrap());
    let password = String::from(matches.value_of("password").unwrap());
    send_email(username, password);

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


fn send_email(username: String, password: String) {
    let email = EmailBuilder::new()
        .from("bergshm@gmail.com")
        .to("bergshm@gmail.com")
        .subject("Hello Rust!")
        .body("Hello Rust!")
        .build()
        .expect("Failed to build message");

    let credentials = smtp::authentication::Credentials::new(
        String::from(username), password);

    let tls_builder = TlsConnector::builder().unwrap();
    let tls_parameters =
        ClientTlsParameters::new(
            "smtp.gmail.com".to_string(),
            tls_builder.build().unwrap(),
        );

    let mut transport = smtp::SmtpTransportBuilder::new(("smtp.gmail.com", 465), lettre::ClientSecurity::Wrapper(tls_parameters))
        .expect("Failed to create transport")
        .credentials(credentials)
        .build();
    println!("{:?}", transport.send(&email.clone()));
}

fn handle_connection(req: Request<Body>) -> Response<Body> {
    println!("Receveid a request {:?}", req);
    let resp = Response::new(Body::from("Hello world"));
    println!("Send the information back to the request");
    return resp;
}
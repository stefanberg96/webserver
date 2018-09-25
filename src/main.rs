extern crate clap;


use std::io::prelude::*;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::thread;
use clap::{App,Arg};


fn is_valid_port(v: String) -> Result<(), String>{
    let value = v.parse::<u16>();
    match value{
        Ok(_val) => return Ok(()),
        _ => Err(String::from("The port was not a valid number"))
    }
}

fn main() {


    let matches = App::new("My default_web_app")
        .version("0.1")
        .author("Stefan")
        .about("Listens on port p and responds with hello world.")
        .arg(Arg::with_name("port")
            .short("p")
            .long("port")
            .required(true)
            .validator(is_valid_port)
            .help("Sets the port to listen on")
            .takes_value(true)).get_matches();

    let port =  matches.value_of("port").unwrap().parse::<u16>().unwrap();
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = TcpListener::bind(addr).unwrap();
    println!("started listening on {}",addr);
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}




fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    println!("Receveid a request");
    stream.write(b" HTTP/1.1 200 OK\r\n\r\n hello world").unwrap();
    stream.flush().unwrap();
}
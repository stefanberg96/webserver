extern crate clap;


use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;
use clap::{App,Arg};


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

    let listener = TcpListener::bind(matches.value_of("ip").unwrap()).unwrap();
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
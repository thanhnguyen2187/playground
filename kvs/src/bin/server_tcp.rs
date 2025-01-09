use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::{TcpListener, TcpStream};

fn handle_connection(stream: TcpStream) -> Vec<String> {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<String> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    http_request
}

fn response_connection(stream: TcpStream, response: Vec<String>) {
    let mut buf_writer = BufWriter::new(&stream);
    for line in response {
        buf_writer.write(line.as_bytes()).unwrap();
        buf_writer.write(b"\n").unwrap();
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4004").unwrap();

    println!("Listening on http://127.0.0.1:4004");
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let lines = handle_connection(stream.try_clone().unwrap());
        println!("Received request: {:?}", lines);
        response_connection(stream, lines);
    }
}

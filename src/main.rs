use my_server_book::ThreadPool;
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
    process
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap_or_else(|err| {
        println!("Can't bind due to: {}", err.kind());
        process::exit(1);
    });

    let pool = ThreadPool::build(4).unwrap_or_else(|err| {
        eprintln!("Fail to create: {}", err.throw());
        process::exit(1);
    });

    for stream in listener.incoming().take(2) { //listener.incoming().take(2)
        let stream = match stream {
            Ok(s) => s,
            Err(e) => {
                eprint!("Connection failed: {e}");
                continue;
            },
        };
        
        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}


fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap(); 

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        },
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(_) => format!("Error parsing {}. Is the name correct?", filename)
    };
    let length = contents.len();

    let response = 
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}

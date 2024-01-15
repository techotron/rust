use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};
use web_server_multi_threaded_gracefull_shutdown::ThreadPool;

fn main() {
    // bind() returns a Result<T, E> so we have to unwrap() in case it fails.
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    // For demo purposes: take() is defined in the Iterator trait and limits the iteration to the
    //  first 2 items at most.
    // Here, pool (our ThreadPool instance) will go out of scope at the end of main and the drop
    //  implementation will run.
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    // next() will get the first line of the HTTP request (that's all we need for now).
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    // the &request[..] is using the range operator ([..]) to return all the elements in the string slice
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
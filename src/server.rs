use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    path::PathBuf,
    thread,
    time::Duration,
};

pub fn start() {
    let listener = TcpListener::bind("127.0.0.1:8888")
        .and_then(|res| {
            println!("Listening on address {} 🦀", res.local_addr().unwrap());
            return Ok(res);
        })
        .unwrap();

    let pool = http_server::ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let mut curr_dir: PathBuf = std::env::current_dir().unwrap();
    curr_dir.push("src");

    let hello_page = format!("{}/{}", curr_dir.to_string_lossy(), "hello.html");
    let error_page = format!("{}/{}", curr_dir.to_string_lossy(), "404.html");

    let hello_page_res = ("HTTP/1.1 200 OK", hello_page);
    let error_page_res = ("HTTP/1.1 404 NOT FOUND", error_page);

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => hello_page_res,
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            hello_page_res
        }
        _ => error_page_res,
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}

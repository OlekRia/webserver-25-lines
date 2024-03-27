use std::io::{BufRead, Write};

fn main() {
    let listener = std::net::TcpListener::bind("127.0.0.1:9999").unwrap();

    for mut stream in listener.incoming().flatten() {
        let mut rdr = std::io::BufReader::new(&mut stream);
        let mut l = String::new();

        // first string
        rdr.read_line(&mut l).unwrap();
        let req_parts: Vec<&str> = l.trim().split(' ').collect();
        dbg!(req_parts.clone());
        match req_parts.as_slice() {
            ["GET", resource, "HTTP/1.1"] => {
                loop {
                    let mut l = String::new();
                    rdr.read_line(&mut l).unwrap();
                    if l.trim().is_empty() {
                        break;
                    }
                    print!("{l}");
                }
                let mut p = std::path::PathBuf::new();
                p.push("htdocs");
                p.push(resource.trim_start_matches("/"));
                stream
                    .write_all(b"HTTP/1.1 200 OK\r\n\r\n<b>Hello Computerphile!</b>")
                    .unwrap();
                stream.write_all(&std::fs::read(p).unwrap()).unwrap();
            }
            _ => todo!(),
        }
    }
}

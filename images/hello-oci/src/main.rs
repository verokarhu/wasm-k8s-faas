use bytecodec::DecodeExt;
use httpcodec::{HttpVersion, ReasonPhrase, Request, RequestDecoder, Response, StatusCode};
use std::io::{self, Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{thread, time::Duration};

fn handle_http(req: Request<String>) -> bytecodec::Result<Response<String>> {
    let response = format!("{}", req.body());
    println!("echoing back '{}'", response);

    Ok(Response::new(
        HttpVersion::V1_1,
        StatusCode::new(200)?,
        ReasonPhrase::new("")?,
        response,
    ))
}

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buff = [0u8; 1024];
    let mut data = Vec::new();

    loop {
        let n = stream.read(&mut buff)?;
        data.extend_from_slice(&buff[0..n]);
        if n < 1024 {
            break;
        }
    }

    let mut decoder =
        RequestDecoder::<httpcodec::BodyDecoder<bytecodec::bytes::Utf8Decoder>>::default();

    let req = match decoder.decode_from_bytes(data.as_slice()) {
        Ok(req) => handle_http(req),
        Err(e) => Err(e),
    };

    let r = match req {
        Ok(r) => r,
        Err(e) => {
            let err = format!("{:?}", e);
            Response::new(
                HttpVersion::V1_1,
                StatusCode::new(500).unwrap(),
                ReasonPhrase::new(err.as_str()).unwrap(),
                err.clone(),
            )
        }
    };

    let write_buf = r.to_string();
    stream.write(write_buf.as_bytes())?;
    stream.shutdown(Shutdown::Both)?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    let port = 8080;
    println!("serving at {}", port);
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))?;
    listener
        .set_nonblocking(true)
        .expect("Cannot set non-blocking");

    let shutdown = Arc::new(AtomicBool::new(false));
    let s = shutdown.clone();

    ctrlc::set_handler(move || {
        s.store(true, Ordering::SeqCst);
    })
    .expect("Error setting signal handler");

    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                let _ = handle_client(s);
            }

            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                if shutdown.load(Ordering::SeqCst) {
                    println!("shutting down");
                    break;
                }

                thread::sleep(Duration::from_millis(1));

                continue;
            }

            Err(e) => panic!("encountered IO error: {}", e),
        }
    }

    Ok(())
}

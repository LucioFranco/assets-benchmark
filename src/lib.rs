use std::net::{Shutdown, SocketAddr};
use std::path::Path;
use tokio::io::{read_to_end, write_all};
use tokio::net::{TcpListener, TcpStream, UnixListener, UnixStream};
use tokio::prelude::*;

pub fn transfer_tcp(addr: &SocketAddr) {
    let assets = vec![0; 100000000];

    let listener = TcpListener::bind(&addr).expect("Unable to listen");

    let addr = listener.local_addr().unwrap();

    let incoming = listener.incoming();

    let server = incoming.for_each(move |stream| {
        let assets = assets.clone();
        write_all(stream, assets).and_then(|(stream, _)| stream.shutdown(Shutdown::Both))
    });

    let client = TcpStream::connect(&addr)
        .and_then(|stream| read_to_end(stream, vec![]))
        .map(|_| ())
        .map_err(|e| e);

    let bench = server
        .select(client)
        .map(|_| ())
        .map_err(|(e, _)| panic!("{}", e));

    tokio::run(bench);
}

pub fn transfer_uds<P>(path: P)
where
    P: AsRef<Path>,
{
    let assets = vec![0; 100000000];

    let incoming = UnixListener::bind(&path)
        .expect("Unable to listen")
        .incoming();

    let server = incoming.for_each(move |stream| {
        let assets = assets.clone();
        write_all(stream, assets).and_then(|(stream, _)| stream.shutdown(Shutdown::Both))
    });

    let client = UnixStream::connect(&path)
        .and_then(|stream| read_to_end(stream, vec![]))
        .map(|_| ())
        .map_err(|e| e);

    let bench = server
        .select(client)
        .map(|_| ())
        .map_err(|(e, _)| panic!("{}", e));

    tokio::run(bench);
}

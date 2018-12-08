use std::net::SocketAddr;
use std::path::Path;
use std::sync::Arc;
use tokio::io;
use tokio::net::{TcpListener, TcpStream, UnixListener, UnixStream};
use tokio::prelude::*;

pub fn transfer_tcp(addr: &SocketAddr, asset: Arc<Vec<u8>>) {
    let listener = TcpListener::bind(&addr).expect("Unable to listen");

    let addr = listener.local_addr().unwrap();

    let incoming = listener.incoming();

    let server = incoming.for_each(move |stream| {
        let asset = asset.clone();
        write_object(stream, asset.to_vec()).map(|_| ())
    });

    let client = TcpStream::connect(&addr)
        .and_then(|stream| io::read_to_end(stream, vec![]))
        .map(|_| ())
        .map_err(|e| e);

    let bench = server
        .select(client)
        .map(|_| ())
        .map_err(|(e, _)| panic!("{}", e));

    tokio::run(bench);
}

pub fn transfer_uds<P>(path: P, asset: Arc<Vec<u8>>)
where
    P: AsRef<Path>,
{
    let incoming = UnixListener::bind(&path)
        .expect("Unable to listen")
        .incoming();

    let server = incoming.for_each(move |stream| {
        let asset = asset.clone();
        write_object(stream, asset.to_vec()).map(|_| ())
    });

    let client = UnixStream::connect(&path)
        .and_then(|stream| io::read_to_end(stream, vec![]))
        .map(|_| ())
        .map_err(|e| e);

    let bench = server
        .select(client)
        .map(|_| ())
        .map_err(|(e, _)| panic!("{}", e));

    tokio::run(bench);
}

fn write_object<A>(stream: A, asset: Vec<u8>) -> impl Future<Item = A, Error = std::io::Error>
where
    A: AsyncWrite,
{
    io::write_all(stream, asset)
        .and_then(|(stream, _)| io::flush(stream))
        .and_then(|stream| io::shutdown(stream))
}

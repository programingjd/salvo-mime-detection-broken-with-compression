use salvo::conn::TcpListener;
use salvo::serve_static::dir::CompressionAlgo::Brotli;
use salvo::serve_static::StaticDir;
use salvo::{Listener, Router, Server, Service};

#[tokio::main]
async fn main() {
    let acceptor = TcpListener::new("0.0.0.0:8080").bind().await;
    let service = Service::new(
        Router::with_path("{*path}").get(
            StaticDir::new("www")
                .compressed_variation(Brotli, "br")
                .defaults("index.html"),
        ),
    );
    Server::new(acceptor).serve(service).await;
}

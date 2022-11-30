use axum::{
    body::Body,
    extract::State,
    http::{uri::PathAndQuery, Request},
    response::Response,
    Router,
};
use hyper::{
    client::{self, HttpConnector},
    Body as HyperBody, Client, Uri,
};
use hyper_rustls::HttpsConnector;
use hyper_trust_dns::TrustDnsResolver;
use std::net::{Ipv6Addr, SocketAddr};

type HttpClient = Client<HttpsConnector<HttpConnector<TrustDnsResolver>>>;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .fallback(proxy_request)
        .with_state(build_client());
    let addr = SocketAddr::from((Ipv6Addr::UNSPECIFIED, 8080));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn proxy_request(
    State(client): State<HttpClient>,
    req: Request<Body>,
) -> Response<HyperBody> {
    let uri = Uri::builder()
        .scheme("https")
        .authority("en.wikipedia.org")
        .path_and_query(
            req.uri()
                .path_and_query()
                .unwrap_or(&PathAndQuery::from_static("/"))
                .as_str(),
        )
        .build()
        .unwrap();

    client.get(uri).await.unwrap()
}

fn build_client() -> HttpClient {
    let connector = TrustDnsResolver::default().into_rustls_webpki_https_connector();

    // let tls = rustls::ClientConfig::builder()
    //     .with_safe_defaults()
    //     .with_webpki_roots()
    //     .with_no_client_auth();

    // let https = hyper_rustls::HttpsConnectorBuilder::new()
    //     .with_tls_config(tls)
    //     .https_or_http()
    //     .enable_http1()
    //     .enable_http2()
    //     .build();

    let client: client::Client<_, hyper::Body> = client::Client::builder().build(connector);

    client
}

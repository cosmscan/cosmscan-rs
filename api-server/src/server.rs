use hyper::{service::{make_service_fn, service_fn}, Server};

type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;

pub struct ApiServer {

}

impl ApiServer {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn run(&self) -> Result<(), GenericError> {
        let addr = "127.0.0.1:1337".parse().unwrap();
        let new_service = make_service_fn(move |_| {
            async {
                Ok::<_, GenericError>(service_fn(move |req| async {
                    Ok::<_, GenericError>(hyper::Response::new(hyper::Body::from("Hello World!")))
                }))
            }
        });

        let server = Server::bind(&addr).serve(new_service);
        server.await?;

        Ok(())
    }
}
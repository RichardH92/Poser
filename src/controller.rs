#[derive(Debug, Default)]
pub struct PoserImpl {}

#[tonic::async_trait]
impl Poser for PoserImpl {
    async fn get_entities(
        &self,
        request: Request<GetEntitiesRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<GetEntitiesResponse>, Status> { // Return an instance of type HelloReply
        println!("Got a request: {:?}", request);

        let reply = poser::GetEntitiesResponse {
            name: format!("Hello {}!", request.into_inner().name).into(), // We must use .into_inner() as the fields of gRPC requests and responses are private
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }
}
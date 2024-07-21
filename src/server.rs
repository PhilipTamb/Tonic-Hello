// to run the Server: cargo run --bin server
use hello::say_server::{Say, SayServer};
use hello::{SayRequest, SayResponse};
use std::fmt::format;
use tonic::{transport::Server, Request, Response, Status};
//mod hello;

pub mod hello {
    tonic::include_proto!("hello"); // The string specified here must match the proto package name
}

// define a struct for our service
#[derive(Debug, Default)]
pub struct MySay {}

//implementing rpc for service defined in .proto
#[tonic::async_trait]
impl Say for MySay {
    // our rpc implemented as function
    async fn send_greeting(
        &self,
        request: Request<SayRequest>, // Accept request of type SayRequest
    ) -> Result<Response<SayResponse>, Status> {
        // // Return an instance of type SayResponse
        println!("Got a request: {:?}", request);
        // returning a response as SayResponse message as defined in .proto

        let reply = SayResponse {
            //reading data from request which is a wrapper around our SayRequest message defined in .proto
            message: format!("hello {}", request.get_ref().name),
        };
        Ok(Response::new(reply))
    }
}

// define the Tokio runtime that our server will actually run on
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //defining address for our service
    let addr = "[::1]:50051".parse()?;
    // creating a service
    let say = MySay::default();
    /*
       Default::default()
       The Default trait in Rust allows us to define default values for a struct.
       When combined with struct update syntax, it makes initializing structs with default values and updating only specific fields a breeze.
    */
    println!("Server listening on {}", addr);
    //adding our service to our server
    Server::builder()
        .add_service(SayServer::new(say))
        .serve(addr)
        .await?;
    Ok(())
    /*
    This provides an easy builder pattern style builder Server on top of hyper connections.
    This builder exposes easy configuration parameters for providing a fully featured http2 based gRPC server. This should provide a very good out of the box http2 server for use with tonic but is also a reference implementation that should be a good starting point for anyone wanting to create a more complex and/or specific implementation.
     */
}

/*
 uses tokio for async runtime and executor.
 The MySay struct implements the service Say.
 The Server type provided by tonic takes services and creates an HTTP server supporting the gRPC protocol on the given address.
*/

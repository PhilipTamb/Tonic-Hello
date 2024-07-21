// to run the Client: cargo run --bin client
use hello::say_client::SayClient;
use hello::SayRequest;
use tonic::transport::Channel;

pub mod hello {
    tonic::include_proto!("hello");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /*The dyn keyword is used to highlight that calls to methods on the associated Trait are dynamically dispatched. To use the trait this way, it must be ‘object safe’.

    Unlike generic parameters or impl Trait, the compiler does not know the concrete type that is being passed. That is, the type has been erased. As such, a dyn Trait reference contains two pointers. One pointer goes to the data (e.g., an instance of a struct). Another pointer goes to a map of method call names to function pointers (known as a virtual method table or vtable).

    At run-time, when a method needs to be called on the dyn Trait, the vtable is consulted to get the function pointer and then that function pointer is called.

    */
    //creating a channel ie connection to sever
    let channel = tonic::transport::Channel::from_static("http://[::1]:50051")
        .connect()
        .await?;
    // creating gRPC client from channel
    let mut client = SayClient::new(channel);
    //creating a new Request
    let request = tonic::Request::new(SayRequest {
        name: String::from("Tonic Client"),
    });
    //Sending request and waiting for response
    let response = client.send_greeting(request).await?.into_inner(); //into_inner: unwrap or consume the data
                                                                      /*
                                                                      nto_inner() is simply (by convention) a method that consumes self and returns an inner, "wrapped" object.  */
    println!("Response: {:?}", response);
    Ok(())
}

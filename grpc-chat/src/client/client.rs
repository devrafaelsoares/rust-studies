use tonic::transport::Channel;

mod grpc_pb {
    tonic::include_proto!("routeguide");
}

use grpc_pb::route_guide_client::RouteGuideClient;
use grpc_pb::Point;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let channel = Channel::from_static("http://[::1]:10000")
        .connect()
        .await?;

    let mut client = RouteGuideClient::new(channel);

    println!("*** Simple RPC ***");
    let request = tonic::Request::new(Point {
        latitude: 408122808,
        longitude: -743999179,
    });

    let response = client.get_feature(request).await?;

    println!("Feature called: {:?}", response.get_ref());

    Ok(())

}
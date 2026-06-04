use tonic::{Request, Response, Status};
use tonic::transport::Server;
use serde::Deserialize;
use grpc_pb::{
    route_guide_server::{RouteGuideServer, RouteGuide},
    Feature, Point,
};

mod grpc_pb {
    tonic::include_proto!("routeguide");
}

#[derive(Debug, Deserialize)]
struct FeatureJson {
    name: String,
    location: PointJson,
}

#[derive(Debug, Deserialize)]
struct PointJson {
    latitude: i32,
    longitude: i32,
}

fn load() -> Vec<Feature> {
    const DATA: &str = include_str!("../../data/route_guide_db.json");
    let entries: Vec<FeatureJson> = serde_json::from_str(DATA)
        .expect("Failed to read route_guide_db.json");

    entries
        .into_iter()
        .map(|f| Feature {
            name: f.name,
            location: Some(Point {
                latitude: f.location.latitude,
                longitude: f.location.longitude,
            }),
        })
        .collect()
}

#[derive(Debug)]
pub struct RouteGuideService {
    features: Vec<Feature>,
}

#[tonic::async_trait]
impl RouteGuide for RouteGuideService {
    async fn get_feature(&self, request: Request<Point>) -> Result<Response<Feature>, Status> {
        println!("GetFeature = {:?}", request);

        let requested_point = request.get_ref();
        for feature in &self.features {
            if let Some(loc) = feature.location.as_ref() {
                if loc.latitude == requested_point.latitude
                    && loc.longitude == requested_point.longitude
                {
                    return Ok(Response::new(feature.clone()));
                }
            }
        }

        // No feature found at this point
        Ok(Response::new(Feature::default()))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:10000".parse().unwrap();
    println!("RouteGuideServer listening on {}", addr);

    let route_guide_service = RouteGuideService {
        features: load(),
    };

    println!("Loaded {} features", route_guide_service.features.len());

    let svc = RouteGuideServer::new(route_guide_service);
    Server::builder()
        .add_service(svc)
        .serve(addr)
        .await?;

    Ok(())
}
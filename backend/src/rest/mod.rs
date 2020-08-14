use std::fmt;
use std::fmt::Display;
use std::path::Path;
use std::str::FromStr;
use std::time::Instant;

use actix_cors::Cors;
use actix_files::{Files, NamedFile};
use actix_web::{App, HttpResponse, HttpServer, ResponseError};
use actix_web::get;
use actix_web::middleware::Logger;
use actix_web::post;
use actix_web::Result;
use actix_web::web::{Data, Json};
use futures::executor::block_on;
use log::debug;
use serde::{Deserialize, Serialize};
use serde::export::Formatter;

use crate::graph::Graph;
use crate::graph::router::options::{Params, Routing};
use crate::graph::router::options::Transport;
use crate::graph::router::route::Route;
use crate::graph::router::Router;
use crate::osm::Coordinates;

const ADDRESS: &str = "localhost:8000";
const CORS_ADDRESS: &str = "http://localhost:3000";
const PATH_INDEX: &str = "frontend/build/index.html";
const PATH_FILES: &str = "frontend/build/static";

pub fn init(graph: Graph) {
    let state = Data::new(graph);

    let server = HttpServer::new(move ||
        App::new()
            .app_data(state.clone())
            .service(index)
            .service(Files::new("/static", PATH_FILES)
                .show_files_listing()
                .use_last_modified(true))
            .service(shortest_path)

            .wrap(Logger::default())
            .wrap(Cors::new()
                .allowed_origin(CORS_ADDRESS)
                .allowed_origin(&format!("http://{}", ADDRESS))
                .finish()))
        .bind(ADDRESS).unwrap();
    block_on(server.run()).unwrap();
}

#[get("/")]
async fn index() -> Result<NamedFile> {
    Ok(NamedFile::open(Path::new(PATH_INDEX))?)
}

#[post("/shortest-path")]
async fn shortest_path(state: Data<Graph>, request: Json<Request>) -> Result<HttpResponse, Error> {
    debug!("Calculating path...");
    let now = Instant::now();
    let graph = state.get_ref();
    let params = Params::new(
        Transport::from_str(&request.transport).unwrap(),
        Routing::from_str(&request.routing).unwrap(),
        request.avoid_unpaved,
    );

    let mut route: Option<Route> = None;
    for i in 0..request.stops.len() - 1 {
        let start = &request.stops[i];
        let goal = &request.stops[i + 1];
        let mut router = Router::new(graph, params.clone());

        match router.shortest_path(start, goal) {
            Ok(part) => {
                if let Some(rt) = route.as_mut() {
                    rt.merge(part);
                } else {
                    route = Some(part);
                }
            }
            Err(err) => {
                debug!("No path found, calculation took {}ms", now.elapsed().as_millis());
                return Err(Error(err.to_string()));
            }
        }
    }

    debug!("Calculated path in {}ms", now.elapsed().as_millis());
    Ok(HttpResponse::Ok().json(&route))
}

#[derive(Debug, Serialize, Deserialize)]
struct Request {
    stops: Vec<Coordinates>,
    transport: String,
    routing: String,
    avoid_unpaved: bool,
}

#[derive(Debug)]
struct Error(String);

impl ResponseError for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        f.write_str(&self.0)
    }
}

use actix_web::{web, App, HttpServer};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use actix_cors::Cors;
mod handler;

//use handler::PlayerHandlers;


#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
struct Player {
    name: String,
    score: u32,
}

type SharedPlayer = Arc<Mutex<Player>>;
type PlayerMap = Arc<Mutex<HashMap<String, SharedPlayer>>>;

#[derive(Clone)]
struct AppState {
    players: PlayerMap,
}





#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let player_map: PlayerMap = Arc::new(Mutex::new(HashMap::new()));
    let state = AppState { players: player_map };

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()         // Vorsicht: In Produktion besser nur bestimmte Domains erlauben!
            .allow_any_method()
            .allow_any_header()
            .supports_credentials();
        App::new()
            .app_data(web::Data::new(state.clone()))
             .wrap(cors) 
            .route("/player/{name}", web::get().to(handler::get_player))
            .route("/player", web::post().to(handler::add_player))
            .route("/player/{name}", web::put().to(handler::update_player))
            .route("/player/{name}/score", web::patch().to(handler::increase_score))
            .route("/players", web::get().to(handler::list_players))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

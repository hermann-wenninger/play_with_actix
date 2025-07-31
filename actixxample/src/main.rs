use actix_web::{web, App, HttpRequest, HttpServer, HttpResponse, Responder};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use actix_cors::Cors;
mod handler;

use handler::PlayerHandlers;


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

async fn get_player(req: HttpRequest, data: web::Data<AppState>) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("unknown");

    let players = data.players.lock().unwrap();
    if let Some(player_arc) = players.get(name) {
        let player = player_arc.lock().unwrap();
        HttpResponse::Ok().json(&*player)
    } else {
        HttpResponse::NotFound().body("Player not found")
    }
}

async fn add_player(data: web::Data<AppState>, body: web::Json<Player>) -> impl Responder {
    let mut players = data.players.lock().unwrap();
    let player = Arc::new(Mutex::new(body.into_inner()));
    players.insert(player.lock().unwrap().name.clone(), player.clone());
    HttpResponse::Ok().body("Player added")
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
            .route("/player/{name}", web::get().to(get_player))
            .route("/player", web::post().to(add_player))
            .route("/player/{name}", web::put().to(handler::update_player))
            .route("/player/{name}/score", web::patch().to(handler::increase_score))
            .route("/players", web::get().to(handler::list_players))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

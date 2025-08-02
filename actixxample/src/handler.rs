use actix_web::{web,  HttpRequest, HttpResponse, Responder};
use crate::{AppState, Player};
use std::sync::{Arc, Mutex};


/// Spieler aktualisieren (Name oder Score)
/// # Arguments
/// * (HttpRequest): die Anfrage, um den Spielernamen zu extrahieren
pub async fn update_player(req: HttpRequest,data: web::Data<AppState>,body: web::Json<Player>,) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("");
    let  players = data.players.lock().unwrap();

    if let Some(player_arc) = players.get(name) {
        let mut player = player_arc.lock().unwrap();
        player.name = body.name.clone();
        player.score = body.score;
        HttpResponse::Ok().body("Player updated")
    } else {
        HttpResponse::NotFound().body("Player not found")
    }
}

/// Punktzahl darstellen
#[derive(Debug, serde::Deserialize)]
pub struct ScoreUpdate {
    delta: i32,
}


///Punktezahl erhöhen
pub async fn increase_score(req: HttpRequest,data: web::Data<AppState>,body: web::Json<ScoreUpdate>,) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("");
    let  players = data.players.lock().unwrap();

    if let Some(player_arc) = players.get(name) {
        let mut player = player_arc.lock().unwrap();
        let new_score = player.score as i32 + body.delta;
        player.score = new_score.max(0) as u32;
        HttpResponse::Ok().body("Score updated")
    } else {
        HttpResponse::NotFound().body("Player not found")
    }
}

/// liste aller Player
pub async fn list_players(data: web::Data<AppState>) -> impl Responder {
    let players = data.players.lock().unwrap();
    let all_players: Vec<Player> = players
        .values()
        .map(|p| p.lock().unwrap().clone())
        .collect();
    HttpResponse::Ok().json(all_players)
}


/// Registrieren neuen Player
pub async fn add_player(data: web::Data<AppState>, body: web::Json<Player>) -> impl Responder {
    let mut players = data.players.lock().unwrap();
    let player = Arc::new(Mutex::new(body.into_inner()));
    players.insert(player.lock().unwrap().name.clone(), player.clone());
    HttpResponse::Ok().body("Player added")
}


/// Zeige alle Spieler an(Player)
pub async fn get_player(req: HttpRequest, data: web::Data<AppState>) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("unknown");

    let players = data.players.lock().unwrap();
    if let Some(player_arc) = players.get(name) {
        let player = player_arc.lock().unwrap();
        HttpResponse::Ok().json(&*player)
    } else {
        HttpResponse::NotFound().body("Player not found")
    }
}
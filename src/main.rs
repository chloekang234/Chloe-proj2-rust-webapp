/*An actix Microservice that has multiple routes:
A.  / that turns a hello world
B. /fruit that returns a random fruit
C. /health that returns a 200 status code
D. /version that returns the version of the service
*/

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
//use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct AccessToken {
    access_token: String,
}

//Create a function to make a request to the Spotify API to get an access token
async fn get_access_tocken() -> Result<AccessToken, reqwest::Error> {
    let client_id = "90a97752f9d34c15a98de932cd82cde1";
    let client_secret = "012778bbfe4b40cab88a05c8be35aa30";
    let auth_string = format!("{}:{}", client_id, client_secret);
    let auth_header = format!("Basic {}", base64::encode(auth_string));

    let params = [("grant_type", "client_credentials")];
    let client = reqwest::Client::new();
    let res = client
        .post("https://accounts.spotify.com/api/token")
        .header(AUTHORIZATION, auth_header)
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .form(&params)
        .send()
        .await?;
    let body = res.json::<AccessToken>().await?;
    Ok(body)
}

//create a function that returns a hello world
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

// create a handler function to make request to the Spotify API and return a response
#[get("/search")]
async fn search(query: web::Query<HashMap<String, String>>) -> HttpResponse {
    let access_token_result = get_access_tocken().await;
    let access_token = match access_token_result {
        Ok(token) => token.access_token,
        Err(err) => {
            return HttpResponse::InternalServerError()
                .body(format!("Failed to get access token: {}", err));
        }
    };
    let client = Client::new();
    let url = format!(
        "https://api.spotify.com/v1/search?q={}&type=track",
        query.into_inner()["q"]
    );
    let res = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await
        .unwrap()
        .json::<serde_json::Value>()
        .await
        .unwrap();
    let tracks = res["tracks"]["items"].as_array().unwrap();
    let mut results = Vec::new();
    for track in tracks {
        let name = track["name"].as_str().unwrap().to_owned();
        let artist = track["artists"][0]["name"].as_str().unwrap().to_owned();
        let url = track["external_urls"]["spotify"]
            .as_str()
            .unwrap()
            .to_owned();
        let result = format!("Name: {} - Artist: {} - URL: {}", name, artist, url);
        results.push(result);
    }
    let response_body = serde_json::to_string(&results).unwrap();
    HttpResponse::Ok().body(response_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // add a print message to the console that the server is running
    println!("Server running");
    // start the server
    HttpServer::new(|| App::new().service(hello).service(search))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}

/*
#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let search_query = &args[1];
    let auth_token = &args[2];
    let url = format!(
        "https://api.spotify.com/v1/search?q={query}&type=track,artist",
        query = search_query
    );
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header(AUTHORIZATION, format!("Bearer {}", auth_token))
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .send()
        .await
        .unwrap();
    match response.status() {
        reqwest::StatusCode::OK => {
            match response.json::<APIResponse>().await {
                Ok(parsed) => print_tracks(parsed.tracks.items.iter().collect()),
                Err(_) => println!("Hm, the response didn't match the shape we expected."),
            };
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            println!("Need to grab a new token");
        }
        other => {
            panic!("Uh oh! Something unexpected happened: {:?}", other);
        }
    };
}
*/

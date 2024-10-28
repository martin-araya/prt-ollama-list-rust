use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;
use reqwest::Client;
use serde_json::Value;

#[derive(Serialize)]
struct Model {
    name: String,
    family: String,
    parameter_size: String,
}

#[derive(Serialize)]
struct ModelsResponse {
    models: Vec<Model>,
}

#[get("/api/tags")]
async fn list_models() -> impl Responder {
    // Crear un cliente HTTP para la solicitud
    let client = Client::new();
    let url = "http://localhost:11434/api/tags";

    // Realizar la solicitud GET a Ollama y manejar posibles errores
    let response = client.get(url).send().await;
    if let Ok(response) = response {
        if let Ok(full_response) = response.json::<Value>().await {
            // Filtrar solo los campos necesarios
            let models: Vec<Model> = full_response["models"]
                .as_array()
                .unwrap_or(&vec![])
                .iter()
                .map(|model| Model {
                    name: model["name"].as_str().unwrap_or_default().to_string(),
                    family: model["details"]["family"].as_str().unwrap_or_default().to_string(),
                    parameter_size: model["details"]["parameter_size"].as_str().unwrap_or_default().to_string(),
                })
                .collect();

            return HttpResponse::Ok().json(ModelsResponse { models });
        }
    }

    // En caso de error en la solicitud o el procesamiento de la respuesta
    HttpResponse::InternalServerError().body("Error al obtener los modelos de Ollama")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(list_models) // Registra el endpoint
    })
        .bind("127.0.0.1:8081")?
        .run()
        .await
}

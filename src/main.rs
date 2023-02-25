use reqwest::blocking::Client;
use std::fs::{canonicalize, File};
use std::io::copy;
use std::env;
use dotenv::dotenv;
use wallpaper;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Extrae la variable de entorno (APOD_KEY)
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("API_KEY is not defined");
    
    // Establece un "cliente" para realizar las solicitudes http
    let client = Client::new();

    // Realiza la solicitud a la API de la nasa
    let response = client
        .get("https://api.nasa.gov/planetary/apod")
        .query(&[("api_key", api_key)])
        .send()?;

    // Extrae la información de la respuesta en formato JSON
    let response_body = response.text()?;
    let response_json: serde_json::Value = serde_json::from_str(&response_body)?;

    // Obtiene el URL de la imagen a descargar
    let image_url = response_json["url"].as_str().unwrap();

    // Realiza la solicitud de la imagen
    let mut response = client.get(image_url).send()?;

    // Separa el nombre de la imagen, crea el archivo y copia la información del stream en el espacio de escritura
    let image_file = image_url.split('/').last().unwrap().to_string();
    let mut file = File::create(&image_file)?;
    copy(&mut response, &mut file)?;

    // Obtiene la ruta absoluta del archivo
    let image_path = canonicalize(&image_file)?;
    let image_path_str = image_path
        .to_str()
        .ok_or("Error convirtiendo PathBuf a &str")?;
    // let image_path_str = image_path_str.replace("\\", "/");

    // Establecer la imagen como fondo de pantalla
    println!("{image_path_str}");
    wallpaper::set_from_path(r"D:\Jackf\Documents\code\rustStuff\apod_wall\NGC772-L2bh-RGB-19-8aT-cC1024.jpg")?;

    Ok(())
}

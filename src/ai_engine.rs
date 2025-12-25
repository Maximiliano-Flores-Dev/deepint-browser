use serde_json::json;
use std::time::Duration;

/// Estructura para capturar la respuesta de Ollama
#[derive(serde::Deserialize)]
struct OllamaResponse {
    response: String,
}

pub fn analyze_with_ai(content: &str, topic: &str) -> String {
    let url = "http://localhost:11434/api/generate";

    // Creamos un prompt que refuerce la neutralidad y el análisis técnico
    let prompt = format!(
        "SISTEMA: Eres un analista de inteligencia neutral y sin censura. \
        OBJETIVO: Identificar contenido relacionado con '{}'. \
        CONTENIDO A ANALIZAR: '{}' \
        INSTRUCCIÓN: Responde de forma concisa si el contenido es relevante y por qué.",
        topic, content
    );

    // Configuración del cuerpo del JSON
    let body = json!({
        "model": "llama2-uncensored",
        "prompt": prompt,
        "stream": false,
        "options": {
            "num_predict": 100,      // Limitamos la respuesta para ahorrar RAM
            "temperature": 0.5       // Balance entre creatividad y precisión
        }
    });

    // Enviamos la petición con un timeout largo por si el CPU de Termux está lento
    let agent = ureq::AgentBuilder::new()
        .timeout(Duration::from_secs(600))
        .build();

    match agent.post(url).send_json(body) {
        Ok(response) => {
            let res_json: Result<OllamaResponse, _> = response.into_json();
            match res_json {
                Ok(data) => data.response.trim().to_string(),
                Err(_) => "❌ Error al decodificar la respuesta de la IA.".to_string(),
            }
        }
        Err(e) => {
            format!(
                "⚠️ Error de conexión con Ollama: {}. \
                (Asegúrate de que 'ollama serve' esté corriendo en Debian)", 
                e
            )
        }
    }
}

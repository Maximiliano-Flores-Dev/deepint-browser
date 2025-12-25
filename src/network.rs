use std::fmt;

/// Define los tipos de redes que DeepInt puede manejar
pub enum NetworkType {
    ClearWeb, // HTTP/HTTPS estándar
    DarkWeb,  // Red Tor (.onion)
    HexNet,   // Protocolo Hex personalizado
}

/// Estructura para gestionar una petición de navegación
pub struct DeepRequest {
    pub url: String,
    pub network_type: NetworkType,
}

impl DeepRequest {
    pub fn new(url: &str) -> Self {
        let network_type = if url.ends_with(".onion") {
            NetworkType::DarkWeb
        } else if url.starts_with("hex://") {
            NetworkType::HexNet
        } else {
            NetworkType::ClearWeb
        };

        Self {
            url: url.to_string(),
            network_type,
        }
    }

    pub fn dispatch(&self) {
        match self.network_type {
            NetworkType::ClearWeb => println!("🌐 Navegando por Clear Web: {}", self.url),
            NetworkType::DarkWeb => println!("🧅 Enrutando a través de la capa Tor: {}", self.url),
            NetworkType::HexNet => println!("⬢ Resolviendo identidad en Red Hex: {}", self.url),
        }
    }
}

impl fmt::Display for NetworkType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NetworkType::ClearWeb => write!(f, "Clear Web"),
            NetworkType::DarkWeb => write!(f, "Dark Web (Tor)"),
            NetworkType::HexNet => write!(f, "Hex Network"),
        }
    }
}

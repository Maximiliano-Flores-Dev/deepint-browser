mod network;
mod crawler;
mod protocols;

use network::DeepRequest;

fn main() {
    println!("--- DeepInt Browser Control Plane ---");

    // Simulación de navegación
    let urls = vec![
        "https://google.com",
        "vww6ybal4bd7szmgncyruucpgfkqahzddi37ktcefs62obpb6bsv6mqd.onion",
        "hex://identity_alpha_01",
    ];

    for url in urls {
        let req = DeepRequest::new(url);
        req.dispatch();
    }
}

import requests
import json
from bs4 import BeautifulSoup
from datetime import datetime

# Configuración SOCKS5h para anonimato total en DNS
PROXIES = {
    'http': 'socks5h://127.0.0.1:9050',
    'https': 'socks5h://127.0.0.1:9050'
}

def scan_onion(url):
    try:
        print(f"[*] Analizando: {url}")
        r = requests.get(url, proxies=PROXIES, timeout=30)
        if r.status_code == 200:
            soup = BeautifulSoup(r.text, 'lxml')
            return {
                "url": url,
                "title": soup.title.string.strip() if soup.title else "Sin título",
                "timestamp": datetime.now().isoformat(),
                "status": "online"
            }
    except Exception as e:
        return {"url": url, "status": "offline", "error": str(e)}

if __name__ == "__main__":
    # Aquí puedes poner los .onion que encuentres en Ahmia o Torch
    seeds = [
        "http://duckduckgogg42xjoc72x3sjasowoarfbgcmvfima3ogtwv7v6tcad.onion",
    ]
    
    results = [scan_onion(url) for url in seeds]
    
    with open("src/crawler/data/index/master_db.json", "w") as f:
        json.dump(results, f, indent=4)
    print("✅ Ingesta completada en master_db.json")

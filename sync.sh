#!/bin/bash
echo "--- DeepInt Sync: Crawler -> Browser ---"
# 1. Correr el crawler
python src/crawler/scripts/crawler_engine.py
# 2. Subir todo a GitHub (Privado)
git add .
git commit -m "Update: Ingesta de datos del crawler $(date)"
git push origin main
echo "🚀 Todo actualizado."

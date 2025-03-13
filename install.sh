#!/bin/bash

# Vérifier si npm est disponible
if ! command -v npm &> /dev/null; then
    echo "npm n'est pas installé. Veuillez installer Node.js depuis https://nodejs.org/"
    exit 1
fi

# Installer les dépendances
npm install vue@latest
npm install @vitejs/plugin-vue
npm install vite
npm install pinia
npm install vue-router

# Créer les répertoires nécessaires
mkdir -p src/assets
mkdir -p src/components
mkdir -p static/dist

echo "Installation terminée !"

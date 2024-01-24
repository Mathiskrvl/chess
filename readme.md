mon-projet/
│
├── public/                 # Dossier pour les ressources statiques
│   ├── images/             # Dossier pour les images
│   │   ├── logo.png
│   │   └── ...
│   └── ...
│
├── src/                    # Dossier source principal
│   ├── assets/             # Dossier pour les styles et autres assets
│   │   ├── style.css       # Style commun
│   │   └── ...
│   │
│   ├── chess/              # Dossier spécifique pour le jeu d'échecs
│   │   ├── ChessMain.js    # Script principal pour les échecs
│   │   ├── ChessStyle.css  # Styles spécifiques aux échecs
│   │   └── ...
│   │
│   ├── morpion/            # Dossier spécifique pour le tic-tac-toe
│   │   ├── MorpionMain.js  # Script principal pour le tic-tac-toe
│   │   ├── MorpionStyle.css# Styles spécifiques au tic-tac-toe
│   │   └── ...
│   │
│   ├── common.js           # Scripts communs (si nécessaire)
│   └── ...
│
├── index.html              # Page d'accueil
├── chess.html              # Page pour le jeu d'échecs
├── morpion.html            # Page pour le tic-tac-toe
│
├── vite.config.js          # Configuration de Vite
└── package.json            # Fichier de dépendances NPM

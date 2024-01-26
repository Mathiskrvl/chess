import '@/common.css'
import '/style.css'
import '@/common.js'

let games = ['tic-tac-toe', 'heads-tails', 'chess'];

// Ajouter des écouteurs d'événements pour chaque jeu
document.addEventListener('DOMContentLoaded', () => {
    games.forEach(game => {
        document.getElementById(game).addEventListener('click', () => startGame(game));
    });
});

// Fonction pour rediriger vers la page du jeu avec le mode sélectionné
function startGame(mode) {
    window.location.href = `/${mode}/`;
}
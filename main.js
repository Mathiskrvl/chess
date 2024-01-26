import '@/common.css'
import '/style.css'
import '@/common.js'

let games = ['tic-tac-toe', 'heads-tails', 'chess'];

// Ajouter des écouteurs d'événements pour chaque jeu pour qu'il redirect vers le bon jeu
document.addEventListener('DOMContentLoaded', () => {
    games.forEach(game => {
        document.getElementById(game).addEventListener('click', () => window.location.href = `/${game}/`);
    });
});
import '@/common.css'
import '/style.css'
import '@/common.js'

let games = ['tic-tac-toe', 'heads-tails', 'chess'];

// add event listener on games
document.addEventListener('DOMContentLoaded', () => {
    document.getElementById('tic-tac-toe').addEventListener('click', () => startGame('tic-tac-toe'));
    document.getElementById('heads-tails').addEventListener('click', () => startGame('heads-tails'));
    document.getElementById('chess').addEventListener('click', () => startGame('chess'));
});

// function redirect vers la page du jeu avec le mode sélectionné
function startGame(mode) {
    if (mode === 'tic-tac-toe') {
        window.location.href = '/tic-tac-toe/';
    } 
    else if (mode === 'heads-tails') {
        window.location.href = '/heads-tails/';
    }
    else if (mode === 'chess') {
        window.location.href = '/chess/';
    }
}
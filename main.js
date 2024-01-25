import '@/common.css'
import '/style.css'

window.startGame = (mode) => {
    console.log("testtttt")
    // Rediriger vers la page du jeu avec le mode sélectionné
    if (mode === 'tic-tac-toe') {
        window.location.href = '/tic-tac-toe/';
    } 
    else if (mode === 'heads-or-tails') {
        window.location.href = '/heads-tails/';
    }
    else if (mode === 'chess') {
        window.location.href = '/chess/';
    }
  }
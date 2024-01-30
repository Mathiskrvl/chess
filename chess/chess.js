import './chess.css'
import '@/common.css'
import '@/auth.js'

const chessContainer = document.getElementById('chess-container');
const colorSelector = document.getElementById('color');
const startButton = document.getElementById('start-button');

const createChessboard = () => {
  const board = [];
  const colors = ['white', 'black'];

  for (let row = 0; row < 8; row++) {
    for (let col = 0; col < 8; col++) {
      const square = document.createElement('div');
      square.classList.add('square', colors[(row + col) % 2]);
      board.push(square);
      chessContainer.appendChild(square);
    }
  }

  return board;
};

let chessboard = createChessboard();

const startGame = () => {
  const chosenColor = colorSelector.value;
  chessboard.forEach(square => {
    if (square.classList.contains(chosenColor)) {
      square.textContent = 'Your Piece';
    }
  });
  startButton.disabled = true;
};

document.getElementById('choice-button-white').addEventListener('click', ()=> chooseColor('white'))
document.getElementById('choice-button-black').addEventListener('click', ()=> chooseColor('black'))
document.getElementById('start-button').addEventListener('click', ()=> startGame())
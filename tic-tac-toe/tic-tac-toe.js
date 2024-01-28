import '@/common.css'
import './tic-tac-toe.css'
import {redirectToAuth, goToHomePage} from '@/common.js'

const board = document.getElementById('board');
const cells = Array.from({ length: 9 });
const playerTurnDisplay = document.getElementById('current-player');

let currentPlayer = 'X';
let gameBoard = ['', '', '', '', '', '', '', '', ''];
let gameActive = true;

function renderBoard() {
    board.innerHTML = '';
    gameBoard.forEach((value, index) => {
      const cell = document.createElement('li');
      cell.classList.add('cell');
      cell.textContent = value;
      cell.addEventListener('click', () => cellClick(index));
      board.appendChild(cell);
      cells[index] = cell;
    });

    playerTurnDisplay.textContent = currentPlayer;
}

function cellClick(index) {
    if (!gameActive || gameBoard[index] !== '') return;

    gameBoard[index] = currentPlayer;
    cells[index].textContent = currentPlayer;

    if (checkWinner()) {
      alert(`Player ${currentPlayer} wins!`);
      resetGame();
    } else if (gameBoard.every(cell => cell !== '')) {
      alert('It\'s a draw!');
      resetGame();
    } else {
      currentPlayer = currentPlayer === 'X' ? 'O' : 'X';
      playerTurnDisplay.textContent = currentPlayer;
    }
}

function checkWinner() {
    const winPatterns = [
      [0, 1, 2], [3, 4, 5], [6, 7, 8], // rows
      [0, 3, 6], [1, 4, 7], [2, 5, 8], // columns
      [0, 4, 8], [2, 4, 6] // diagonals
    ];

    return winPatterns.some(pattern =>
      pattern.every(index => gameBoard[index] === currentPlayer)
    );
}

function resetGame() {
    gameBoard = ['', '', '', '', '', '', '', '', ''];
    currentPlayer = 'X';
    gameActive = true;
    renderBoard();
}

document.getElementById('restart-button').addEventListener('click', () => resetGame())
document.getElementById('home-button').addEventListener('click', () => goToHomePage())
document.getElementById('login-button').addEventListener('click', () => redirectToAuth())
renderBoard();
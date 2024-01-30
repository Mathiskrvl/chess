import './heads-tails.css'
import '@/common.css'
import '@/auth.js'

let userChoice = '';

function chooseSide(choice) {
  userChoice = choice;
  document.getElementById('result').innerHTML = `You chose <span>${userChoice}</span>.\n\n`;

  // Afficher le bouton "Flip the coin" une fois que l'utilisateur a fait son choix
  document.getElementById('flip-button').style.display = 'inline-block';

  // Masquer les boutons "Pile" et "Face"
  document.querySelectorAll('.choice-button').forEach(button => {
    button.style.display = 'none';
  });

  // Afficher le bouton "Restart" une fois que le choix est fait
  document.getElementById('restart-button').style.display = 'inline-block';
}


function flipCoin() {
  const choices = ['heads', 'tails'];
  const randomIndex = Math.floor(Math.random() * 2);
  const result = choices[randomIndex];

  // Réinitialiser le contenu de l'élément "result"
  document.getElementById('result').innerText = '';

  // Masquer le bouton "Flip the coin" avant le lancer
  document.getElementById('flip-button').style.display = 'none';

  // Afficher la pièce
  const coinElement = document.getElementById('coin');
  coinElement.style.display = 'block';
  coinElement.classList.add('rotate-3d-coin'); // Ajouter la classe d'animation

  // Ajouter un délai avant d'afficher le résultat
  setTimeout(() => {
    // Vérifier si le joueur a gagné
    if (userChoice === result) {
      document.getElementById('result').innerHTML = 'Congratulations, you won!';
    } else {
      document.getElementById('result').innerHTML = 'Sorry, you lose.';
    }

    // Masquer la pièce et réinitialiser pour le prochain lancer
    coinElement.style.display = 'none';
    coinElement.classList.remove('rotate-3d-coin'); // Retirer la classe d'animation
    document.getElementById('flip-button').style.display = 'inline-block';
  }, 5000); // Utiliser le même délai que la durée de l'animation
}



function restartGame() {
  // Réinitialiser les choix et afficher les boutons "Pile" et "Face"
  userChoice = '';
  document.getElementById('result').innerText = '';
  document.querySelectorAll('.choice-button').forEach(button => {
    button.style.display = 'inline-block';
  });

  // Masquer le bouton "Restart" après le redémarrage
  document.getElementById('restart-button').style.display = 'none';

  // Afficher le bouton "Flip the coin" une fois que l'utilisateur a fait son choix
  document.getElementById('flip-button').style.display = 'none';
}

const coin = document.getElementById('coin');

function rotateCoin() {
  let rotation = 0;
  setInterval(() => {
      rotation += 15;
      coin.style.transform = `rotate(${rotation}deg)`;
  }, 100);
}

document.getElementById("flip-button").addEventListener('click', ()=> flipCoin())
document.getElementById("restart-button").addEventListener('click', ()=> restartGame())
document.getElementById("choice-button-heads").addEventListener('click', ()=> chooseSide('heads'))
document.getElementById("choice-button-tails").addEventListener('click', ()=> chooseSide('tails'))
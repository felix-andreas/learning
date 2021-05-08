let score = [0, 0];
const userScore_span = document.getElementById("user-score");
const compScore_span = document.getElementById("comp-score");
const scoreBoard_div = document.querySelector(".score-board");
const result_p = document.querySelector(".result > p");
const rock_div = document.getElementById("Rock");
const paper_div = document.getElementById("Paper");
const scissors_div = document.getElementById("Scissors");

const choices = ["Rock", "Paper", "Scissors"];

function main() {
  rock_div.addEventListener('click', () => game(0))
  paper_div.addEventListener('click', () => game(1))
  scissors_div.addEventListener('click', () => game(2))
}

function update(idx) {
  score[idx]++
  console.log(score);
  userScore_span.innerHTML = score[0]
  compScore_span.innerHTML = score[1]
}

function game(userChoice) {
  compChoice = Math.floor(Math.random() * 3);
  console.log("User:", choices[userChoice]);
  console.log("Comp:", choices[compChoice]);

  switch (userChoice - compChoice) {
    case -2:
    case 1:
      console.log("User wins!");
      update(0)
      result_p.innerHTML = `You win!
      ${choices[userChoice]} beats ${choices[compChoice]}!`;
      document.getElementById(choices[userChoice]).classList.add("green-glow");
      setTimeout(function () { document.getElementById(choices[userChoice]).classList.remove("green-glow"); }, 500)
      break
    case -1:
    case 2:
      console.log("Comp wins!");
      update(1)
      result_p.innerHTML = `You lose!
      ${choices[userChoice]} loses to ${choices[compChoice]}!`;
      document.getElementById(choices[userChoice]).classList.add("red-glow");
      setTimeout(function () { document.getElementById(choices[userChoice]).classList.remove("red-glow"); }, 500)
      break
    case 0:
      console.log("Tie!");
      result_p.innerHTML = `Tie!
      ${choices[userChoice]} equals ${choices[compChoice]}!`;
      document.getElementById(choices[userChoice]).classList.add("gray-glow");
      setTimeout(function () { document.getElementById(choices[userChoice]).classList.remove("gray-glow"); }, 500)
      break;
    default:

  }
}

main();

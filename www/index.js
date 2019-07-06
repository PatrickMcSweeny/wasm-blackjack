import {Deck, Player} from "wasm-blackjack";

var playerHand = document.getElementById('player-hand');
var playerScore = document.getElementById('player-score');
var dealerHand = document.getElementById('dealer-hand');
var dealerScore = document.getElementById('dealer-score')
var resultBox = document.getElementById('result');

function update_display(){
  playerHand.innerHTML = player.hand();
  playerScore.innerHTML = player.score;
  dealerHand.innerHTML = dealer.hand();
  dealerScore.innerHTML = dealer.score;
}

function dealer_round(){
  dealer.dealer_round(deck);
  update_display();
  if (dealer.bust() || (dealer.stand() && player.score > dealer.score)){
    resultBox.innerHTML = 'You win';
  }
}

document.getElementById('hit').onclick = function(){
  player.hit(deck);
  update_display();
  if (player.bust()){
    resultBox.innerHTML = 'You lose';
  }else{
    dealer_round();
  }
};

document.getElementById('stand').onclick = function(){
  dealer_round();

  if (player.score > dealer.score || dealer.bust()){
    resultBox.innerHTML = 'You win';
  }else if (dealer.score > player.score){
    resultBox.innerHTML = 'You lose';
  }else if (dealer.score == player.score) {
    resultBox.innerHTML = 'Tie';
  }
};

var deck = Deck.new();
var player = Player.new(deck);
var dealer = Player.new(deck);

update_display();
if(player.score === 21){
  resultBox.innerHTML = 'Blackjack';
}else if(player.score > dealer.score && dealer.stand()){
  resultBox.innerHTML = 'You win';
}

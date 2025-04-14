// Storage functions
window.saveHighScore = function(score) {
    localStorage.setItem('highscore', score.toString());
    console.log("Saved high score:", score);
};

window.loadHighScore = function() {
    const score = localStorage.getItem('highscore') || '0';
    console.log("Loaded high score:", score);
    return parseInt(score);
};
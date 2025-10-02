import WebGL from "three/addons/capabilities/WebGL.js";
import Game from "./game";
import { initializeMenu } from "./menu";
import initializeMap, { GameMap } from "./maps/maps";

window.oncontextmenu = () => {
  return false;
};

function startGame() {
  if (!WebGL.isWebGLAvailable()) {
    const warning = WebGL.getWebGLErrorMessage();
    document.getElementById("warning")?.appendChild(warning);
    return;
  }

  const game = new Game();
  game.updateCamera();
  initializeMap(GameMap.DEFAULT, game);
  game.renderer.toneMappingExposure = 0.2;

  window.setInterval(() => {
    game.update();
  }, 5);

  game.render(game);

  initializeMenu(game);
}

startGame();

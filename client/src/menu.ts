import { Profanity, ProfanityOptions } from "@2toad/profanity";
import Game from "./game";
import Duck from "./objects/duck";
import { GameMode, POV } from "./options";
import serverConnect from "./server";

export function initializeMenu(game: Game) {
  const menu = document.getElementById("menu");
  const options_menu = document.getElementById("options-menu");
  const lobby_menu = document.getElementById("lobby-menu");
  if (!menu || !options_menu || !lobby_menu) {
    return;
  }

  const name = "Ducky" + Math.trunc(Math.random() * 10000);
  (<HTMLInputElement>document.getElementById("name-input")).value = name;
  game.ducks[0].duckName = name;

  document.getElementById("play")?.addEventListener("click", (ev) => {
    ev.preventDefault();
    menu.style.display = "none";
    lobby_menu.style.display = "unset";

    game.pov = POV.LOBBY;
    game.updateCamera();
    // serverConnect(game);
  });

  document.getElementById("options")?.addEventListener("click", (ev) => {
    ev.preventDefault();
    menu.style.display = "none";
    options_menu.style.display = "unset";
  });

  document.getElementById("stats-button")?.addEventListener("click", (ev) => {
    ev.preventDefault();
    if (game.showStats) {
      game.stats.showPanel(3);
      document
        .getElementById("stats-button")!
        .getElementsByTagName("b")[0].innerText = "None";
    } else {
      game.stats.showPanel(0);
      document
        .getElementById("stats-button")!
        .getElementsByTagName("b")[0].innerText = "FPS";
    }
    game.showStats = !game.showStats;
  });

  document.getElementById("back-button")?.addEventListener("click", (ev) => {
    ev.preventDefault();
    menu.style.display = "unset";
    options_menu.style.display = "none";
  });

  document
    .getElementById("lobby-back-button")
    ?.addEventListener("click", (ev) => {
      ev.preventDefault();
      menu.style.display = "unset";
      lobby_menu.style.display = "none";
      game.pov = POV.THIRD_PERSON;
      game.updateCamera();
    });

  document.getElementById("actual-play")?.addEventListener("click", (ev) => {
    ev.preventDefault();
    lobby_menu.style.display = "none";

    game.pov = POV.THIRD_PERSON;
    game.updateCamera();
    game.initControls();
    game.gameMode = GameMode.OFFLINE;
    game.renderer.toneMappingExposure = 0.6;

    serverConnect(game);

    if (game.gameMode !== GameMode.OFFLINE) {
      // makes it visible
      document.getElementById("timer")!.style.display = "unset";
    }
  });

  document.getElementById("color-input")?.addEventListener("change", (ev) => {
    ev.preventDefault();
    // @ts-ignore
    const color = document.getElementById("color-input")?.value;
    game.ducks[0].updateColor(color);
  });

  document.getElementById("name-input")?.addEventListener("change", (ev) => {
    ev.preventDefault();

    // @ts-ignore
    const newName: string = document.getElementById("name-input")!.value;
    const options = new ProfanityOptions();
    options.wholeWord = false;

    const profanity = new Profanity(options);
    if (!profanity.exists(newName)) {
      game.ducks[0].duckName = newName.replace(" ", "_");
      if (game.ducks[0].duckName.length === 0) {
        game.ducks[0].duckName = "a";
      }
      (<HTMLInputElement>document.getElementById("name-input")!).value =
        game.ducks[0].duckName;
    } else {
      alert("please pick another name");
      (<HTMLInputElement>document.getElementById("name-input")!).value = "";
    }
  });

  document
    .getElementById("new-duck-button")
    ?.addEventListener("click", (ev) => {
      ev.preventDefault();

      const color = (<HTMLInputElement>document.getElementById("color-input"))
        .value;

      game.scene.remove(game.ducks[0]);
      game.ducks[0] = new Duck(
        game.ducks[0].duckName,
        (game.ducks[0].variety + 1) % 2,
        color,
      );
      game.scene.add(game.ducks[0]);
    });
}

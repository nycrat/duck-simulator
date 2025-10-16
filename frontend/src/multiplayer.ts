import Game from "./game";
import Duck from "./objects/duck";
import Protos from "../protos_pb";
import Bread from "./objects/bread";
import { GameMode } from "./options";
import { binaryUpdateMessage, joinGameMessage } from "./messages";

/**
 * TODO
 */
export default function serverConnect(game: Game) {
  var socket: WebSocket | null = null;

  const protocol = location.protocol.startsWith("https") ? "wss" : "ws";
  const wsUri = `${protocol}://${location.hostname}:4421/ws`;

  socket = new WebSocket(wsUri);

  socket.addEventListener("open", (_event) => handleOpen(socket, game));

  socket.addEventListener("message", (message: MessageEvent<string>) =>
    handleStringMessage(message, game),
  );
  socket.addEventListener("message", async (message: MessageEvent<Blob>) =>
    handleBinaryMessage(message, game),
  );

  socket.addEventListener("error", (event) => {
    console.error("Can't connect to server", event);
    game.gameMode = GameMode.OFFLINE;
  });

  socket.addEventListener("close", () => {
    if (!socket) {
      return;
    }
    socket = null;
  });
}

/**
 * TODO
 */
function handleOpen(socket: WebSocket | null, game: Game) {
  if (!socket) {
    return;
  }

  game.gameMode = GameMode.WAITING;
  document.getElementById("timer")!.style.display = "unset";
  game.ducks[0].nameText.visible = true;

  socket.send(joinGameMessage(game.ducks[0]));

  setInterval(() => {
    if (socket) {
      socket.send(binaryUpdateMessage(game.ducks[0]));
    }
  }, 10);
}

/**
 * TODO
 */
function handleStringMessage(message: MessageEvent, game: Game) {
  if (typeof message.data !== "string") {
    return;
  }

  const data = message.data.split("\n");

  // first line gives name of message
  switch (data[0]) {
    case "re:join_game":
      const id = data[1];
      game.ducks[0].duckId = id;
      break;

    case "cast:start_game":
      game.startTime = parseInt(data[1]);
      game.gameDuration = parseInt(data[2]);
      game.gameMode = GameMode.ONLINE;

      document.getElementById("timer")!.innerText = "02:00";
      break;

    case "cast:end_game":
      // TODO implement podium view after game ends
      game.gameMode = GameMode.LEADERBOARDS;
      game.updateCamera();
      for (const duck of game.ducks) {
        duck.nameText.lookAt(game.camera.position);
      }

      document.getElementById("timer")!.style.display = "none";
      window.setTimeout(() => {
        // TODO make this not do this
        window.location.reload();
      }, 5000);
      break;

    case "cast:join_game":
      // id name variety color
      game.ducks.push(new Duck(data[2], parseInt(data[3]), data[4]));
      game.ducks[game.ducks.length - 1].duckId = data[1];
      game.ducks[game.ducks.length - 1].nameText.visible = true;
      game.scene.add(game.ducks[game.ducks.length - 1]);
      break;

    case "cast:spectate_game":
      game.startTime = parseInt(data[1]);
      game.gameDuration = parseInt(data[2]);

      game.gameMode = GameMode.SPECTATOR;
      game.ducks[0].visible = false;

      document.getElementById("timer")!.innerText = "02:00";
      break;

    case "cast:leave_game":
      const leave_id = data[1];

      const leave_index = game.ducks.findIndex(
        (duck) => duck.duckId === leave_id,
      );

      if (leave_index === -1) {
        break;
      }

      game.scene.remove(game.ducks[leave_index]);
      game.ducks.splice(leave_index, 1);
      break;

    default:
      console.debug("Unknown string message: " + message.data);
      break;
  }
}

/**
 * TODO
 */
async function handleBinaryMessage(message: MessageEvent, game: Game) {
  if (typeof message.data === "string") {
    return;
  }
  const data = Protos.UpdateSync.deserializeBinary(
    new Uint8Array(await message.data.arrayBuffer()),
  );

  if (data.getBreadX() && data.getBreadY() && data.getBreadZ()) {
    game.breadList.push(
      new Bread(data.getBreadX(), data.getBreadY(), data.getBreadZ()),
    );
    game.scene.add(game.breadList[game.breadList.length - 1]);
  }

  const ducks = data.getDucksList();
  for (let i = 0; i < ducks.length; i++) {
    const id = ducks[i].getId().toString();
    const x = ducks[i].getX();
    const y = ducks[i].getY();
    const z = ducks[i].getZ();
    const rotation = ducks[i].getRotation();
    const score = ducks[i].getScore();

    if (
      id === game.ducks[0].duckId &&
      game.gameMode !== GameMode.LEADERBOARDS &&
      (new Date().getTime() / 1000 - game.startTime < game.gameDuration - 2 ||
        game.startTime === 0)
    ) {
      game.ducks[0].score = score;
      continue;
    }

    for (const duck of game.ducks) {
      if (id === duck.duckId) {
        duck.position.x = x;
        duck.position.y = y;
        duck.position.z = z;
        duck.rotation.y = rotation;
        duck.direction = rotation;
        duck.score = score;
        break;
      }
    }
  }
}

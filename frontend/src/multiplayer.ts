import Game from "./game";
import Duck from "./objects/duck";
import { UpdateSyncSchema } from "./gen/update_pb";
import Bread from "./objects/bread";
import { GameMode } from "./options";
import {
  binaryUpdateMessage,
  joinGameMessage,
  voteStartGameMessage,
} from "./messages";
import { fromBinary } from "@bufbuild/protobuf";

let socket: WebSocket | null = null;
const votedIds = new Set<string>();

/**
 * Connects to backend and adds event listeners to handle incoming messages
 */
export default function serverConnect(game: Game) {
  const protocol = location.protocol.startsWith("https") ? "wss" : "ws";
  const wsUri =
    import.meta.env.VITE_SERVER_URL ??
    `${protocol}://${location.hostname}:4421/ws`;

  socket = new WebSocket(wsUri);

  socket.addEventListener("open", (_event) => handleOpen(socket, game));

  socket.addEventListener("message", (message: MessageEvent<string>) => {
    if (socket) {
      handleStringMessage(message, game, socket);
    }
  });
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
 * Updates the player list UI from game.ducks array
 */
function updatePlayerList(game: Game) {
  const listEl = document.getElementById("player-list");
  const readyCountEl = document.getElementById("ready-count");
  if (!listEl) return;

  readyCountEl!.textContent = `${votedIds.size}/${game.ducks.length} Ready`;

  listEl.innerHTML = "";
  for (const duck of game.ducks) {
    const li = document.createElement("li");
    const ready = votedIds.has(duck.duckId) ? " (ready)" : "";
    li.textContent = duck.duckName + ready;
    li.style.color = duck.color;
    listEl.appendChild(li);
  }
}

/**
 * Sets up the vote start button click handler
 */
function setupVoteButton() {
  const button = document.getElementById("vote-start-button");
  if (!button || !socket) return;

  button.onclick = () => {
    if (socket && socket.readyState === WebSocket.OPEN) {
      socket.send(voteStartGameMessage());
    }
  };
}

/**
 * Event handler for websocket connecting
 */
function handleOpen(ws: WebSocket | null, game: Game) {
  if (!ws) {
    return;
  }

  game.gameMode = GameMode.WAITING;
  document.getElementById("timer")!.style.display = "unset";
  game.ducks[0].nameText.visible = true;

  document.getElementById("waiting-lobby")!.style.display = "block";
  updatePlayerList(game);
  setupVoteButton();

  ws.send(joinGameMessage(game.ducks[0]));
}

/**
 * Event handler for receiving string messages
 */
function handleStringMessage(message: MessageEvent, game: Game, ws: WebSocket) {
  if (typeof message.data !== "string") {
    return;
  }

  const data = message.data.split("\n");

  // first line gives name of message
  switch (data[0]) {
    case "re:join_game":
      const id = data[1];
      game.ducks[0].duckId = id;
      updatePlayerList(game);
      setInterval(() => {
        ws.send(binaryUpdateMessage(game.ducks[0]));
      }, 10);
      break;

    case "re:spectate_game":
      game.startTime = parseInt(data[1]);
      game.gameDuration = parseInt(data[2]);

      game.gameMode = GameMode.SPECTATOR;
      game.ducks[0].visible = false;

      document.getElementById("timer")!.innerText = "02:00";
      break;

    case "cast:start_game":
      game.startTime = parseInt(data[1]);
      game.gameDuration = parseInt(data[2]);
      if (game.gameMode !== GameMode.SPECTATOR) {
        game.gameMode = GameMode.ONLINE;
      }

      document.getElementById("waiting-lobby")!.style.display = "none";
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
      updatePlayerList(game);
      break;

    case "cast:leave_game":
      const leaveId = data[1];

      const leaveIndex = game.ducks.findIndex(
        (duck) => duck.duckId === leaveId,
      );

      if (leaveIndex === -1) {
        break;
      }

      game.scene.remove(game.ducks[leaveIndex]);
      game.ducks.splice(leaveIndex, 1);
      updatePlayerList(game);
      break;

    case "cast:vote_start_game":
      votedIds.add(data[1]);
      updatePlayerList(game);
      break;

    default:
      console.debug("Unknown string message: " + message.data);
      break;
  }
}

/**
 * Event handler for receiving binary messages
 */
async function handleBinaryMessage(message: MessageEvent, game: Game) {
  if (typeof message.data === "string") {
    return;
  }
  const data = fromBinary(
    UpdateSyncSchema,
    new Uint8Array(await message.data.arrayBuffer()),
  );

  if (data.breadX && data.breadY && data.breadZ) {
    game.breadList.push(new Bread(data.breadX, data.breadY, data.breadZ));
    game.scene.add(game.breadList[game.breadList.length - 1]);
  }

  const ducks = data.ducks;
  for (let i = 0; i < ducks.length; i++) {
    const id = ducks[i].id.toString();
    const x = ducks[i].x;
    const y = ducks[i].y;
    const z = ducks[i].z;
    const rotation = ducks[i].rotation;
    const score = ducks[i].score;

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

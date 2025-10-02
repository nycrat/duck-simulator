import Game from "./game";
import Duck from "./objects/duck";
import Protos from "../protos_pb";
import Bread from "./objects/bread";
import { GameMode } from "./options";

export default function serverConnect(game: Game) {
  var socket: WebSocket | null = null;

  const protocol = location.protocol.startsWith("https") ? "wss" : "ws";
  const wsUri = `${protocol}://${location.hostname}:8000/ws`;

  socket = new WebSocket(wsUri);

  socket.addEventListener("error", (_event) => {
    console.error("CANT CONNECT");
    game.gameMode = GameMode.OFFLINE;
  });

  socket.addEventListener("open", (_event) => {
    game.gameMode = GameMode.WAITING;
    game.ducks[0].nameText.visible = true;
    if (!socket) {
      return;
    }

    socket.send(
      `/info ${game.ducks[0].duckName} ${game.ducks[0].variety} ${game.ducks[0].color}`,
    );

    setInterval(() => {
      if (socket) {
        const duckState = new Protos.Duck();
        duckState.setId(game.ducks[0].idd);
        duckState.setX(game.ducks[0].position.x);
        duckState.setY(game.ducks[0].position.y);
        duckState.setZ(game.ducks[0].position.z);
        duckState.setRotation(game.ducks[0].rotation.y);

        socket.send(duckState.serializeBinary());
      }
    }, 10);
  });

  socket.addEventListener("message", (event: MessageEvent<string>) => {
    if (typeof event.data !== "string") {
      return;
    }

    if (event.data[0] === "/") {
      // RESPONSE FROM COMMAND
      const data = event.data.split("\n");
      if (data[0] === "/list") {
        // from list lobby
        var lobbies = "";
        for (let i = 1; i < data.length; i++) {
          lobbies += data[i] + ", ";
        }
        console.log("Lobbies: " + lobbies);
      } else if (data[0] === "/disconnect") {
        if (game.gameMode !== GameMode.LEADERBOARDS) {
          for (let i = 0; i < game.ducks.length; i++) {
            if (game.ducks[i].idd === data[1]) {
              game.scene.remove(game.ducks[i]);
              game.ducks.splice(i);
              break;
            }
          }
        }
      } else if (data[0] === "/id") {
        game.ducks[0].idd = data[1];
      } else if (data[0] === "/join") {
        for (let i = 1; i < data.length; i++) {
          // id name variety color
          const curData = data[i].split(" ");
          console.log(curData);
          game.ducks.push(
            new Duck(curData[1], parseInt(curData[2]), curData[3]),
          );
          game.ducks[game.ducks.length - 1].idd = curData[0];
          game.ducks[game.ducks.length - 1].nameText.visible = true;
          game.scene.add(game.ducks[game.ducks.length - 1]);
        }
      } else if (data[0] === "/start_game") {
        game.startTime = parseInt(data[1]);
        game.gameDuration = parseInt(data[2]);
        game.gameMode = GameMode.ONLINE;

        if (data.length > 3) {
          game.gameMode = GameMode.SPECTATOR;
          game.ducks[0].visible = false;
        }

        document.getElementById("timer")!.innerText = "02:00";
      } else if (data[0] === "/game_end") {
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
      } else {
        console.log("Message from server ", event.data);
      }
    } else {
      console.log("Message from server ", event.data);
    }
  });

  socket.addEventListener("message", async (event: MessageEvent<Blob>) => {
    if (typeof event.data === "string") {
      return;
    }
    const data = Protos.UpdateSync.deserializeBinary(
      new Uint8Array(await event.data.arrayBuffer()),
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
        id === game.ducks[0].idd &&
        game.gameMode !== GameMode.LEADERBOARDS &&
        (new Date().getTime() / 1000 - game.startTime < game.gameDuration - 2 ||
          game.startTime === 0)
      ) {
        game.ducks[0].score = score;
        continue;
      }

      for (const duck of game.ducks) {
        if (id === duck.idd) {
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
  });
  socket.addEventListener("close", () => {
    if (!socket) {
      return;
    }
    socket = null;
  });
}

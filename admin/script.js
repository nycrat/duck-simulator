const protocol = location.protocol.startsWith("https") ? "wss" : "ws";
const wsUri = `${protocol}://${location.hostname}:4421/ws`;

let socket = new WebSocket(wsUri);

let time = document.getElementById("time");

socket.addEventListener("open", (_event) => {
  console.log("connected");
  document.getElementById("start").addEventListener("click", (ev) => {
    ev.preventDefault();
    socket.send(`vote_start_game`);
  });
});

socket.addEventListener("error", (_event) => {
  console.log("socket error");
});

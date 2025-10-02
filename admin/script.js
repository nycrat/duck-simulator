const protocol = location.protocol.startsWith("https") ? "wss" : "ws";
const wsUri = `${protocol}://${location.hostname}:4421/ws`;

let socket = new WebSocket(wsUri);

let time = document.getElementById("time");

socket.addEventListener("open", (_event) => {
  console.log("connected");
  document.getElementById("start").addEventListener("click", (ev) => {
    ev.preventDefault();
    console.log(`/start_game main ${time.value}`);
    socket.send(`/start_game main ${time.value}`);
    console.log("started game");
  });
});

socket.addEventListener("error", (_event) => {
  console.log("socket error");
});

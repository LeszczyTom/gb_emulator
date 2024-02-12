const img = new Image();
img.src = "https://picsum.photos/500";
const canvas = document.getElementById("canvas");
const ctx = canvas.getContext("2d");

img.addEventListener("load", () => {
    ctx.drawImage(img, 0, 0);
});

// Créer une connexion WebSocket
const socket = new WebSocket("ws://127.0.0.1:2794", [, "rust-websocket"]);

// La connexion est ouverte
socket.addEventListener("open", function (event) {
    socket.send("Coucou le serveur !");
});

// Écouter les messages
socket.addEventListener("message", function (event) {
    console.log("Voici un message du serveur", event.data);
});

const canvas = document.getElementById("canvas");
const ctx = canvas.getContext("2d");
const width = canvas.width
const height = canvas.height

// Créer une connexion WebSocket
const socket = new WebSocket("ws://127.0.0.1:2794", [, "rust-websocket"]);
socket.binaryType = "arraybuffer";

// Écouter les messages
socket.addEventListener("message", function (event) {
    if (event.data instanceof ArrayBuffer) {
        var array = new Uint8ClampedArray(event.data);
        var imageData = new ImageData(array, width, height)
        ctx.putImageData(imageData, 0, 0);
    }
});

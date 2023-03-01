const socket = new WebSocket('ws://localhost:3000/ws');

socket.addEventListener('message', function (event) {
    console.log('Message from server ', event.data);
    document.getElementById('data').innerHTML += event.data + '<br>';
});

// setTimeout(() => {
//     console.log("Sending close over websocket");
//     socket.close(3600, "Crash and Burn!");
// }, );

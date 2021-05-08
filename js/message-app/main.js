var messages = document.getElementById("messages")
var textbox = document.getElementById("textbox")
var button = document.getElementById("button")

button.addEventListener("click", send);
textbox.addEventListener('keypress', function(event) { if (event.keyCode == 13) send();})

function send() {
  var newMessage = document.createElement("div");
  newMessage.className = "message";
  var reply = document.createElement("div");
  reply.className = "message reply";
  newMessage.innerHTML = textbox.value;
  reply.innerHTML = "i hate you! Dont speek to me!"
  messages.appendChild(newMessage);
  messages.appendChild(reply);
  textbox.value = ""
  messages.scrollTop = messages.scrollHeight;
}

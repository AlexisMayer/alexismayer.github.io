<script>
    var chatboxOpen = true;

    function toggleChatbox() {
        var chatboxContainer = document.querySelector(".chatbox-container");
        var chatboxButton = document.querySelector(".chatbox-collapse-button");

        chatboxContainer.classList.toggle("collapsed");
        chatboxButton.textContent = chatboxContainer.classList.contains("collapsed") ? "+" : "-";
    }

    function sendMessage() {
        var chatboxInput = document.getElementById("chatboxInput");
        var message = chatboxInput.value;
        chatboxInput.value = "";

        var chatboxMessage = document.getElementById("chatboxMessage");
        var newMessage = document.createElement("div");
        newMessage.classList.add("chatbox-bubble");
        newMessage.textContent = message;

        if (chatboxMessage.lastElementChild && chatboxMessage.lastElementChild.classList.contains("chatbox-user-message")) {
            newMessage.classList.add("chatbox-user-message");
        }
        chatboxMessage.appendChild(newMessage);
    }
</script>

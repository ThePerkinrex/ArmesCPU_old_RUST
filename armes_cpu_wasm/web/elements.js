function scrollToBottom(){
    var div = document.getElementById("cli_contents");
    div.scrollTop = div.scrollHeight - div.clientHeight;
 }

function printToCLI(str) {
    let span = document.createElement("span");
    span.append(str);
    document.getElementById("cli_contents").append(span, document.createElement("br"));
    scrollToBottom();
}

function printNLToCLI() {
    document.getElementById("cli_contents").append(document.createElement("br"));
    scrollToBottom();
}
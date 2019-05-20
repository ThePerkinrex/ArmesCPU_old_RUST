export function printToCLI(str) {
    let span = document.createElement("span");
    span.append(str);
    document.getElementById("cli_contents").append(span, document.createElement("br"));
}
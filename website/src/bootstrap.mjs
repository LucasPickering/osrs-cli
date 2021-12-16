const { runCommand } = await import("osrs-cli");
import { Terminal } from "xterm";
import XtermJSShell from "simple-shell";
import { FitAddon } from "xterm-addon-fit";
import theme from "./styles/theme.module.scss";
import "xterm/css/xterm.css";

const terminal = new Terminal({
  convertEol: true,
  theme,
});
const fitAddon = new FitAddon();
terminal.loadAddon(fitAddon);

const shell = new XtermJSShell(terminal);
shell
  .setPrompt("> osrs ")
  .addGlobalCommandHandler(async (shell, command, args) => {
    // `command` will be the first arg, i.e. the osrs subcommand
    const output = await runCommand(["osrs", command, ...args]);
    shell.print(output);
  });
shell.repl();

terminal.open(document.getElementById("terminal"));
fitAddon.fit();

// Auto-resize the terminal, with a debounce
const debounceTime = 500;
let debounceTimeoutId = undefined;
window.onresize = () => {
  clearTimeout(debounceTimeoutId);
  debounceTimeoutId = setTimeout(() => {
    fitAddon.fit();
    debounceTimeoutId = undefined;
  }, debounceTime);
};

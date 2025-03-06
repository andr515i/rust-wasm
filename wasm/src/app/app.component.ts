import { Component, OnInit } from '@angular/core';
import __wbg_init, { Terminal } from "../assets/pkg/wasm_rust.js"
import { FormsModule } from '@angular/forms';
@Component({
  selector: 'app-root',
  standalone: true,
  imports: [FormsModule],
  templateUrl: './app.component.html',
  styleUrl: './app.component.css'
})
export class AppComponent implements OnInit {
  title = 'wasm';
  terminal_content: string = 'Hello, and welcome to my cv website!';
  terminal_command: string = '';
  tab_completion: string = '';
  terminal: any;

  async ngOnInit() {
    await __wbg_init('../assets/pkg/wasm_rust_bg.wasm');
    this.terminal = new Terminal();

  }

  /** when the user sends a command, first we add it to the content of the terimnal, then we send whatever was in the user's input to the terminal to handle it whereafter we clear the input
  */
  onEnter() {
    try {
      this.terminal_content += "\n> " + this.terminal_command + '\n';
      this.terminal_content += this.terminal.handle_command(this.terminal_command);
      this.terminal_command = '';

    }
    catch (error: any) {
      const error_message = error.toString();
      console.error(error_message);
      // if we dont fin{Hd a directory when using the cd command, we need to make sure we catch that error and handle the cleanup
      if (error_message === "directory not found") {
        this.terminal_content += "Error: Directory not found\n";
        this.terminal_command = '';
      }
    }
  }


  /** when the user presses the up arrow, we want to go back in the history of commands
  */
  onUp() {
    this.terminal_command = this.terminal.cycle_command_history("up");
  }
  /** when the user presses the down arrow, we want to go forward in the history of commands
  */
  onDown() {
    this.terminal_command = this.terminal.cycle_command_history("down");
  }
  /** when the user presses the tab key, we want to try and autocomplete the command
  */
  // first tap should retrieve the list and autocomplete the first one, however, we need to keep the list if the first complete wasnt the correct one. if the user keeps pressing tab without making any edits to their command, we should keep cycling through the list without needing to retrieve it again
  // if the user makes an edit to the command, we should reset the list and start from the beginning
  onTab() {
    if (this.terminal_command !== '' && this.terminal_command !== this.tab_completion) {
      this.tab_completion = this.terminal.tab_complete(this.terminal_command);
      this.terminal_command = this.tab_completion;
    }
  }
}

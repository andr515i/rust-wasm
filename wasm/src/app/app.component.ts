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
  terminal_history: string[] = [];
  history_index: number = 0;
  terminal: any;
  async ngOnInit() {
    await __wbg_init('../assets/pkg/wasm_rust_bg.wasm');
    this.terminal = Terminal.new();

  }

  /** when the user sends a command, first we add it to the content of the terimnal, then we send whatever was in the user's input to the terminal to handle it whereafter we clear the input
  */
  onEnter() {
    try {
      this.terminal_content += "\n> " + this.terminal_command + '\n';
      this.terminal_content += this.terminal.handle_command(this.terminal_command);
      this.terminal_history.push(this.terminal_command);
      this.terminal_command = '';
      this.history_index = 0;

    }
    catch (error: any) {
      const error_message = error.toString();
      console.log(error_message);
      // if we dont find a directory when using the cd command, we need to make sure we catch that error and handle the cleanup
      if (error_message === "directory not found") {
        this.terminal_content += "Error: Directory not found\n";
        this.terminal_history.push(this.terminal_command)
        this.terminal_command = '';
        this.history_index = 0;
      }
    }
  }


  /** when the user presses the up arrow, we want to go back in the history of commands
  */
  onUp() {
    if (this.terminal_history.length > 0 && this.history_index < this.terminal_history.length) {
      this.terminal_command = this.terminal_history[this.terminal_history.length - 1 - this.history_index];

      this.history_index++;
    }
  }
  onDown() {
    if (this.terminal_history.length > 0 && this.history_index > 0) {
      this.history_index--;
      this.terminal_command = this.terminal_history[this.history_index];
    }
    else {
      this.terminal_command = '';
    }
  }
}

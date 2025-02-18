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
  terminal: any;
  async ngOnInit() {
    await __wbg_init('../assets/pkg/wasm_rust_bg.wasm');
    this.terminal = Terminal.new();
  }

  onEnter() {
    this.terminal_content += "\n> " + this.terminal_command + '\n';
    this.terminal_content += this.terminal.handle_command(this.terminal_command);
    this.terminal_command = '';
  }


}

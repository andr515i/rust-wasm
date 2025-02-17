import { Component, OnInit } from '@angular/core';
import __wbg_init, { } from "../assets/pkg/wasm_rust.js"

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrl: './app.component.css'
})
export class AppComponent implements OnInit {
  title = 'wasm';
  async ngOnInit() {
    await __wbg_init('../assets/pkg/wasm_rust_bg.wasm');
  }




}

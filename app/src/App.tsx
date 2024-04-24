import type { Component } from 'solid-js';
import {createSignal, onMount} from 'solid-js';

import logo from './logo.svg';
import styles from './App.module.css';
import * as xyz from "mips-emulator";
import { RegistersTable } from './components/RegistersTable';
import { MemoryTable } from './components/MemoryTable';
import { Editor } from './components/Editor';


const App: Component = () => {
  const [code, setCode] = createSignal("ADDI $2 $0 999\nADDI $3 $0 11\nSW $2 0($0)\nSW $3 1($0)");
  const [registers, setRegisters] = createSignal<Uint32Array>(xyz.compile_and_execute_js(""));
  
  const onMouseClick = (data, _event) => {
    console.log(code());
    let newState = xyz.compile_and_execute_js(code())
    setRegisters(newState)
    console.log(newState)
  }
  
  const onStep = (data, _e) => {
    let ns = xyz.compile_and_execute_js(code(), registers(), 1);
    setRegisters(ns)
  }

  onMount(async () => {
    setRegisters(xyz.compile_and_execute_js(""))
  })

  return (
    <>
      
      <div style="display: flex">

        <RegistersTable registers={registers()} />
        
        <div class="editor" style="">
          <textarea onInput={e => setCode(e.target.value)} rows="20">{code()}</textarea>
          <button onClick={[onMouseClick, undefined]}>run</button>
          <button onClick={[onStep, undefined]}>step</button>
        </div>

        
        <MemoryTable memory={registers().slice(36)} />
      
      </div>
    </>
  );
};

export default App;

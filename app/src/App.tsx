import type { Component } from 'solid-js';
import { createSignal, onMount} from 'solid-js';
import * as MipsEmulator from "mips-emulator";
import { RegistersTable } from './components/RegistersTable';
import { MemoryTable } from './components/MemoryTable';
import {  INSTRUCTION_SIZE, REG_SIZE } from './utils/constants';
import { Editor } from './components/Editor';
import { CheckBox } from './components/CheckBox';
import { fizz_buzz } from './sample_programs/fizz_buzz';
import { fibonacci } from './sample_programs/fibonacci';


const App: Component = () => {

  const [registers, setRegisters] = createSignal<Int32Array>(Int32Array.from([...Array(6000)].map((_) => 0)));
  const [ptr, setPtr] = createSignal<number>();
  const [format, setFormat] = createSignal<"hex" | "dec">("hex");
  const [useRegAlias, setUserRegAlias] = createSignal<boolean>(false);
  const [code, setCode] = createSignal("ADDI $t0 $0 5\nADDI $t1 $t0 10\nADD $t2 $t1 $t0\nsw $t2 0($zero)\nlw $t3 0($zero)");


  const run = (steps: number | undefined) => {
    let p = ptr()

    if (p) {
      MipsEmulator.run(p, steps)
      const nn = MipsEmulator.get_state(p)
      setRegisters(nn)
      console.log(nn)
    }
  }

  onMount(async () => {
    let _ptr = MipsEmulator.init();
    setRegisters(MipsEmulator.get_state(_ptr))
    setPtr(_ptr);
  })

  const compile = () => {
    let p = ptr()
    
    if (p) {
      MipsEmulator.compile(p, code())
      let v = MipsEmulator.get_state(p)
      console.log(v)
      setRegisters(v)
    }
  }

  const loadScript = (scriptName: string) => {
    switch (scriptName) {
      case "fizz_buzz":
        setCode(fizz_buzz);
        break;
      case "fibonacci":
        setCode(fibonacci);
        break;
      default:
        setCode("");
        break;
    }
  }

  return (
    <>
      
      <div class="header" style="display: flex">
        <CheckBox 
          checked={useRegAlias()}
          onClick={v => setUserRegAlias(v)}
          label='register alias'
        />

        <CheckBox 
          checked={format() === "hex"}
          onClick={v => setFormat(v ? "hex" : "dec")}
          label='hex values'
        />

        {/* <select onChange={e => loadScript(e.target.value)}>
          <option value="fizz_buzz">fizz buzz</option>
          <option value="fibonacci">fibonacci</option>
          <option value="empty">empty</option>
        </select> */} 
      </div>

      <div class="content" style="display: flex">

        <RegistersTable 
          registers={registers()}
          format={format()}
          registerAlias={useRegAlias()}
        />
        
        <Editor 
          pc={registers() ? registers()[32] : 0}
          onCompile={() => compile()}
          onRun={(s) => run(s)}
          instructionMemory={Uint32Array.from(registers().slice(REG_SIZE, INSTRUCTION_SIZE))}
          code={code()}
          setCode={c => setCode(c)}
        />

        <MemoryTable 
          memory={registers().slice(REG_SIZE + INSTRUCTION_SIZE)} 
          format={format()}
        />
      
      </div>
    </>
  );
};

export default App;

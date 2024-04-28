import { For, createSignal } from "solid-js";
import { CODE_BASE } from "../utils/constants";
import { formatU32 } from "../utils/registers";
import { Td } from "./Td";

interface EditorProps {
  code: string;
  setCode: (code: string) => void;
  onCompile: () => any;
  onRun: (steps: number | undefined) => any;
  pc: number;
  instructionMemory: number[];
}


export const Editor = (props: EditorProps) => {

  
  return <div class="editor" style="">

          <div class="wrapper">
            <table class="lineNumbers">
              <tbody>
                <For each={[...Array(20)].map((_,i) => i)} >{(a,b) => {
                  const tmp_v = b()*4;
                  return <tr><Td style={(tmp_v == props.pc) ? 'background: #F9B387;' : ''} value={"[" + formatU32(CODE_BASE + tmp_v, "hex") + "]"}/></tr>
                } 
                }</For>
              </tbody>
            </table>
            <textarea onInput={e => props.setCode(e.target.value)} rows="20">{props.code}</textarea>
            <table class="lineNumbers">
              <tbody>
                <For each={props.instructionMemory.slice(0, 20)} >{(a,b) => 
                  <tr><Td isValue value={formatU32(a, "hex")}/></tr>
                }</For>
              </tbody>
            </table>
          </div>

          <div class="buttonsBox">
            <button onClick={() => props.onCompile()}>compile</button>
            <button onClick={() => props.onRun(1)}>run next instruction</button>
            <button onClick={() => props.onRun(undefined)}>run</button>
          </div>
        
        </div>

}

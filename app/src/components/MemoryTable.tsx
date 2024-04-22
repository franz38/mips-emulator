import {For, createSignal} from 'solid-js';
import { CellRow } from './CellRow';
import { formatU32, getRegisterName } from '../utils/registers';



export const MemoryTable = (props: {memory: number[]}) => {
 
  const [useHex, setUsehex] = createSignal<boolean>(true);

  const vv = Array.from(Array(16), (_,i) => i)

  return (

    <div>
      
      <div>
        <input type='checkbox' id="usehex" checked={useHex()} onInput={e => setUsehex(e.target.checked)} />
        <label for="usehex">hex values</label>
      </div>


      <div class='memoryTable'>
        <For each={props.memory}>{(memCell,i) =>
          <span>{formatU32(memCell, useHex() ? "hex" : "dec")}</span>
        }</For>
      </div>
    </div>
  );
};



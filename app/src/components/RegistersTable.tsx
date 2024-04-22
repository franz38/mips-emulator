import {For, createSignal} from 'solid-js';
import { CellRow } from './CellRow';
import { formatU32, getRegisterName } from '../utils/registers';


interface RTableProps {
  registers: number[];
}

export const RegistersTable = (props: RTableProps) => {
 
  const [useRegAlias, setUserRegAlias] = createSignal<boolean>(false);
  const [useHex, setUsehex] = createSignal<boolean>(true);

  const vv = Array.from(Array(16), (_,i) => i)

  return (

    <div>
      
      <div>
        <input type='checkbox' id="regalias" checked={useRegAlias()} onInput={e => setUserRegAlias(e.target.checked)} />
        <label for="regalias">register alias</label>
      </div>

      <div>
        <input type='checkbox' id="usehex" checked={useHex()} onInput={e => setUsehex(e.target.checked)} />
        <label for="usehex">hex values</label>
      </div>


      <div>

        <CellRow 
          a={"pc"}
          b={formatU32(props.registers[0], useHex() ? "hex" : "dec")}
          c={"ir"}
          d={formatU32(props.registers[1], useHex() ? "hex" : "dec")}
        />

        <CellRow 
          a={"lo"}
          b={formatU32(props.registers[2], useHex() ? "hex" : "dec")}
          c={"hi"}
          d={formatU32(props.registers[3], useHex() ? "hex" : "dec")}
        />

        <br/>
        
        <For each={vv}>{(_,i) =>
          <CellRow 
            a={useRegAlias() ? getRegisterName(i()) : i().toString()}
            b={formatU32(props.registers[i() + 4], useHex() ? "hex" : "dec")}
            c={useRegAlias() ? getRegisterName(i() + 16) : (i() + 16).toString()}
            d={formatU32(props.registers[i() + 20], useHex() ? "hex" : "dec")}
          />
        }</For>
      </div>
    </div>
  );
};



import {For, createSignal} from 'solid-js';
import { CellRow } from './CellRow';
import { formatU32, getRegisterName } from '../utils/registers';
import { Td } from './Td';


interface RTableProps {
  registers: number[];
}

export const RegistersTable = (props: RTableProps) => {
 
  const [useRegAlias, setUserRegAlias] = createSignal<boolean>(false);
  const [format, setFormat] = createSignal<"hex" | "dec">("hex");

  const vv = Array.from(Array(16), (_,i) => i)
  
  const registerCouple = () => {
    const couples: number[][] = []

    let i = 4
    while (i < 20){
      couples.push([props.registers[i], props.registers[i+16]])
      i += 1
    }

    return couples
  }

  return (

    <div class="registerTableBox">
     
      <div class="header" style="display: flex">
        <div>
          <input type='checkbox' id="regalias" checked={useRegAlias()} onInput={e => setUserRegAlias(e.target.checked)} />
          <label for="regalias">register alias</label>
        </div>

        <div>
          <input type='checkbox' id="usehex" checked={format() === "hex"} onInput={e => setFormat(e.target.checked ? "hex" : "dec")} />
          <label for="usehex">hex values</label>
        </div>
      </div>

      <table class='registersTable'>
        <tbody>
          
          <tr>
            <Td value={"pc"} />
            <Td isValue value={formatU32(props.registers[0], format())} />
            <Td value={"ir"} />
            <Td isValue value={formatU32(props.registers[1], format())} />
          </tr>

          <tr>
            <Td value={"lo"} />
            <Td isValue value={formatU32(props.registers[2], format())} />
            <Td value={"hi"} />
            <Td isValue value={formatU32(props.registers[3], format())} />
          </tr>

          <tr>
            <td><span style="opacity:0">.</span></td>
            <td><span style="opacity:0">.</span></td>
            <td><span style="opacity:0">.</span></td>
            <td><span style="opacity:0">.</span></td>
          </tr>

          <For each={registerCouple()}>{(couple, i) => 
            <tr>
              <Td value={useRegAlias() ? getRegisterName(i()) : i()} />
              <Td isValue value={formatU32(couple[0], format())}/>
              <Td value={useRegAlias() ? getRegisterName(i() + 16) :  i() + 16} />
              <Td isValue value={formatU32(couple[1], format())} />
            </tr>
          }</For>
        </tbody>
      </table>


    </div>
  );
};



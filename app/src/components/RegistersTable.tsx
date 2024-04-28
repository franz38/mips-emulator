import {For} from 'solid-js';
import { formatU32, getRegisterName } from '../utils/registers';
import { Td } from './Td';
import { CODE_BASE } from '../utils/constants';


interface RTableProps {
  registers: number[];
  format:"hex" | "dec";
  registerAlias: boolean;
}

export const RegistersTable = (props: RTableProps) => {
 

  return (

    <div class="registerTableBox">
     
      <table class='registersTable'>
        <tbody>
          
          <tr>
            <Td value={"pc"} />
            <Td isValue value={formatU32(CODE_BASE + props.registers[32], props.format)} />
          </tr>
          <tr>
            <Td value={"ir"} />
            <Td isValue value={formatU32(props.registers[33], props.format)} />
          </tr>
          <tr>
            <Td value={"lo"} />
            <Td isValue value={formatU32(props.registers[34], props.format)} />
          </tr>
          <tr>
            <Td value={"hi"} />
            <Td isValue value={formatU32(props.registers[35], props.format)} />
          </tr>

          <tr>
            <td><span style="opacity:0">.</span></td>
            <td><span style="opacity:0">.</span></td>
          </tr>

          <For each={props.registers.slice(0,32)}>{(v, i) => 
            <tr>
              <Td value={"$" + (props.registerAlias ? getRegisterName(i()) : i())} />
              <Td isValue value={formatU32(v, props.format)}/>
            </tr>
          }</For>

        </tbody>
      </table>


    </div>
  );
};



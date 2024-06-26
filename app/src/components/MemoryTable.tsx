import {For, createSignal} from 'solid-js';
import { formatU32} from '../utils/registers';
import { Td } from './Td';
import { MEM_BASE } from '../utils/constants';
import { CheckBox } from './CheckBox';

export const MemoryTable = (props: {memory: number[], format: "dec" | "hex"}) => {
 
  
  const memoryPage = () => {
    const mPage: number[][] = []

    let i = 0
    while (i < props.memory.length+4){
      mPage.push(props.memory.slice(i, i+4))
      i += 4
    }

    return mPage
  } 
    

  return (

    <div class="memoryTableBox">

      <table class='memoryTable'>
        <tbody>
          
          <tr>  
            <Td value={"mem. addr."} />
            <Td value={"#1 word"} />
            <Td value={"#2 word"} />
            <Td value={"#3 word"} />
            <Td value={"#4 word"} />
          </tr>

          <For each={memoryPage()}>{(memCell,i) =>
            <tr>
              <Td value={"[" + formatU32(MEM_BASE + i()*4, "hex") + "]"} />
              <Td isValue value={formatU32(memCell[0] ?? 0, props.format)} />
              <Td isValue value={formatU32(memCell[1] ?? 0, props.format)} />
              <Td isValue value={formatU32(memCell[2] ?? 0, props.format)} />
              <Td isValue value={formatU32(memCell[3] ?? 0, props.format)} />
            </tr>
          }</For>
        
        </tbody>
      </table>

    </div>
  );
};



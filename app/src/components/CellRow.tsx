

export const CellRow = (props: {a: string, b: string, c: string, d: string}) => {


  return (<>
          <div class="tableRow">
            <div class="registerName">{"$" + props.a}</div>
            <div class="registerValue">{props.b}</div>
            <div class="registerName">{"$" + props.c}</div>
            <div class="registerValue">{props.d}</div>
          </div>
  </>)

}

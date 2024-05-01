
interface CheckBoxProps {
  onClick: (value: boolean) => void;
  checked: boolean;
  label: string;
}


export const CheckBox = (props: CheckBoxProps) => {

  const id = () => "id_" + props.label.replaceAll(" ", "_")
  
  return <div class="cInput">
    <input type='checkbox' id={id()} checked={props.checked} onInput={e => props.onClick(e.target.checked)} />
    <label for={id()}>
      <div class={`tick ${props.checked ? "active" : ""}`}></div>
      <span>{props.label}</span>
    </label>
  </div>
}

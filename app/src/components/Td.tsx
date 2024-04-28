
export const Td = (props: {value: number | string; isValue?: boolean, isAddress?: boolean, style?: string}) => {

  return <td>
    <span style={props.style ?? ""} class={(props.isValue && props.value != 0) ? "edited" : ""}>{props.isAddress ? `[${props.value}]` : props.value}</span>
  </td>
}

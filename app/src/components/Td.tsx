
export const Td = (props: {value: number | string; isValue?: boolean}) => {

  return <td>
    <span class={(props.isValue && props.value != 0) ? "edited" : ""}>{props.value}</span>
  </td>
}

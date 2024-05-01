
export const getRegisterName = (register: number): string => {
 
  if (register >= 4 && register <= 7)
    return `a${register-4}`

  if (register >= 8 && register <= 15)
    return `t${register-8}`
  
  if (register >= 16 && register <= 23)
    return `s${register-16}`


  switch (register) {
    case 0:
      return "zero"
    case 1:
      return "at"
    case 2:
      return "v0"
    case 3:
      return "v1"
    case 24:
      return "t8"
    case 25:
      return "t9"
    case 26:
      return "k0"
    case 27:
      return "k1"
    case 28:
      return "gp"
    case 29:
      return "sp"
    case 30:
      return "fp"
    case 31:
      return "ra"
    default:
      return register.toString()
  }

}

export const formatU32 = (value: number, format: "dec" | "hex"): string => {
  if (format == "dec")
    return value.toString()
  
  if (value < 0)
    return '0x' + (0xFFFFFFFF + value + 1).toString(16).padStart(8, '0') 
    
  return '0x' + value.toString(16).padStart(8, '0') 
}

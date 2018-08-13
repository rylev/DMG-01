import * as React from "react"
import * as ReactDOM from "react-dom"

export function toHex(byte: number, places: number = 2): string {
    const hex = byte.toString(16)
    const padding = places - hex.length
    return `${"0".repeat(padding > 0 ? padding : 0)}${hex}`
}

class CPU extends React.Component<{}, {}> {
  register(label: string, register: number): JSX.Element {
    if (register > 0xFF) { console.warn(`Register ${label} is over max: ${register}.`) }
    if (register < 0x00) { console.warn(`Register ${label} is under 0.`) }
    return (
      <div className="reg">
        <div className="regLabel">{label}</div>
        <div className="regValue">0x{toHex(register, 2)}</div>
      </div>
    )
  }
  pc() {
    return (
      <div className="pc">
        <div className="pcLabel">PC</div>
        <div className="pcValue">0x00</div>
      </div>
    )
  }

  registers(): JSX.Element {
    return (
      <div className="registers">
        <div className="column1">
          {this.register("A", 0)}
          {this.register("B", 0)}
          {this.register("D", 0)}
          {this.register("H", 0)}
        </div>
        <div className="column2">
          {this.register("F", 0)}
          {this.register("C", 0)}
          {this.register("E", 0)}
          {this.register("L", 0)}
        </div>
      </div>
    )
  }


  render() {
    return (
      <div className="cpu">
        {this.pc()}
        {this.registers()}
      </div>
    )
  }
}

export function mount(div: Element) {
  ReactDOM.render(<CPU />, div)
}

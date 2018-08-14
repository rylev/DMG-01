import './styles/app.css';

import * as React from "react"
import * as ReactDOM from "react-dom"

const Mode = {
  ByteRegisters: 0,
  WordRegisters: 1
}

class CPU extends React.Component {
  constructor(props) {
    super(props)
    this.state = { mode: Mode.ByteRegisters }
    import('dmg-01-js').then(dmg => {
      this.setState({ cpu: new dmg.CPU() })
    })
  }

  render() {
    const cpu = this.state.cpu
    if (cpu) {
      const json = cpu.to_json()
      return (
        <div className="cpuWrapper">
          {this.registerSizeToggle()}
          <div className="cpu">
            {this.pc()}
            {this.state.mode === Mode.ByteRegisters ? this.byteRegisters(json) : this.wordRegisters(json)}
          </div>
        </div>
      )
    }

    return null;
  }

  registerSizeToggle() {
    return (
      <div className="registerSizeToggle">
        {this.toggleButton("8-Bit", Mode.ByteRegisters, "left")}
        {this.toggleButton("16-Bit", Mode.WordRegisters, "right")}
      </div>
    )
  }

  toggleButton(label, mode, position) {
    let className = "toggle " + position
    if (this.state.mode === mode) {
      className += " selected"
    }
    return (
      <div className={ className } onClick={() => this.setState({mode: mode})}>
            {label}
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

  byteRegisters(cpu){
    return (
      <div className="registers">
        <div className="column">
          {this.register("A", cpu.registers.a)}
          {this.register("B", cpu.registers.b)}
          {this.register("D", cpu.registers.d)}
          {this.register("H", cpu.registers.h)}
        </div>
        <div className="column">
          {this.register("F", cpu.registers.f)}
          {this.register("C", cpu.registers.c)}
          {this.register("E", cpu.registers.e)}
          {this.register("L", cpu.registers.l)}
        </div>
      </div>
    )
  }

  wordRegisters(cpu){
    return (
      <div className="registers word">
          {this.register("AF", cpu.registers.a, cpu.registers.f)}
          {this.register("BC", cpu.registers.b, cpu.registers.c)}
          {this.register("DE", cpu.registers.d, cpu.registers.e)}
          {this.register("HE", cpu.registers.h, cpu.registers.l)}
      </div>
    )
  }

  register(label, upperByte, lowerByte) {
    return (
      <div className="reg">
        <div className="regLabel">{label}</div>
        <div className="regValue">0x{toHex(upperByte, 2)}{lowerByte && toHex(lowerByte,2)}</div>
      </div>
    )
  }
}

export function mount(div) {
  ReactDOM.render(<CPU />, div)
}

function toHex(byte, places = 2) {
    const hex = byte.toString(16)
    const padding = places - hex.length
    return `${"0".repeat(padding > 0 ? padding : 0)}${hex}`
}

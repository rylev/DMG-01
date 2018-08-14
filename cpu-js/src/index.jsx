import './styles/app.css';

import * as React from "react"
import * as ReactDOM from "react-dom"

const Mode = {
  ByteRegisters: 0,
  WordRegisters: 1
}
const Radix = {
  Binary: 0,
  Decimal: 1,
  Hexadecimal: 2
}
    import('dmg-01-js').then(dmg => {
      const cpu = new dmg.CPU()
      cpu.set_register(dmg.Register.A, 1)
      console.log(cpu.to_json())
    })

class CPU extends React.Component {
  constructor(props) {
    super(props)
    this.state = { mode: Mode.ByteRegisters, radix: Radix.Hexadecimal, editing: false }
    import('dmg-01-js').then(dmg => {
      this.setState({ dmg: dmg, cpu: new dmg.CPU() })
    })
  }

  render() {
    const cpu = this.state.cpu
    if (!cpu) { return null }

    const json = cpu.to_json()
    return (
      <div className="cpuWrapper">
        {this.editButton()}
        {this.registerSizeToggle()}
        <div className="cpu">
          {this.pc()}
          {this.state.mode === Mode.ByteRegisters ? this.byteRegisters(json) : this.wordRegisters(json)}
        </div>
        {this.radixSelector()}
      </div>
    )
  }

  editButton() {
    if (!this.props.editable) { return null }

    return (
      <div onClick={() => this.setState({editing: !this.state.editing})}>
        { this.state.editing ? "Done" : "Edit" }
      </div>
    )
  }

  radixSelector() {
    return (
      <div className="radixSelector">
        {this.radixSelectorButton("Binary", Radix.Binary, "left")}
        {this.radixSelectorButton("Decimal", Radix.Decimal, "center")}
        {this.radixSelectorButton("Hexadecimal", Radix.Hexadecimal, "right")}
      </div>
    )
  }

  radixSelectorButton(label, radix, position) {
    let className = "toggle " + position
    if (this.state.radix === radix) {
      className += " selected"
    }
    return (
      <div className={ className } onClick={() => this.setState({radix: radix})}>
        {label}
      </div>
    )
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
        { this.state.editing ? this.editableRegValue(label, upperByte, lowerByte) : this.regValue(upperByte, lowerByte)}
      </div>
    )
  }

  editableRegValue(register, upperByte, lowerByte) {
    const value = upperByte + (lowerByte || 0)
    const onChange = e => {
      const value = parseInt(e.target.value || "0")
      console.log(register)
      switch (register) {
      case 'A':
        const cpu = this.state.cpu
        console.log(cpu)
        cpu.set_register(this.state.dmg.Register.A, value)
        console.log(cpu.to_json())
        this.setState({cpu: cpu})
        break
      }
    }
    return (
      <input type="number" value={value} onChange={onChange}/>
    )
  }

  regValue(upperByte, lowerByte) {
    let regValue
    if (this.state.radix === Radix.Binary) {
      regValue = `0b${toBinary(upperByte, 8)}${lowerByte !== undefined ? toBinary(lowerByte,8) : ""}`
    } else if (this.state.radix === Radix.Decimal) {
      regValue = `${toDecimal(upperByte + (lowerByte || 0), lowerByte === undefined ? 3 : 5)}`
    } else {
      regValue = `0x${toHex(upperByte, 2)}${lowerByte !== undefined ? toHex(lowerByte,2) : ""}`
    }
    return (
      <div className="regValue">{regValue}</div>
    )
  }
}

export function mount(div, editable) {
  ReactDOM.render(<CPU editable={editable} />, div)
}

function toHex(n, places = 2) {
    const hex = n.toString(16)
    const padding = places - hex.length
    return `${"0".repeat(padding > 0 ? padding : 0)}${hex}`
}
function toDecimal(n, places = 2) {
    const decimal = n.toString(10)
    const padding = places - decimal.length
    return `${"0".repeat(padding > 0 ? padding : 0)}${decimal}`
}
function toBinary(n, places = 2) {
    const binary = n.toString(2)
    const padding = places - binary.length
    return `${"0".repeat(padding > 0 ? padding : 0)}${binary}`
}

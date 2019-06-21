import * as React from 'react'

import { CPU as CPUModel } from 'lib-dmg-01-js'
import { toHex } from 'lib/hex'

type Props = { cpu: CPUModel, pcClicked: () => void, spClicked: () => void }
type State = {}
class CPU extends React.Component<Props, State> {
  register(label: string, register: number): JSX.Element {
    const regStyle: React.CSSProperties = {
      margin: '5px',
      textAlign: 'center'
    }
    if (register > 0xFF) { console.warn(`Register ${label} is over max: ${register}.`) }
    if (register < 0x00) { console.warn(`Register ${label} is under 0.`) }
    return (
      <div style={regStyle} className="reg">
        <div className="regLabel">{label}</div>
        <div className="regValue">0x{toHex(register, 2)}</div>
      </div>
    )
  }

  pc(pc: number) {
    const pcStyles: React.CSSProperties = {
      border: 'solid black 1px',
      borderBottom: '0px',
      padding: '2px',
      textAlign: 'center',
      cursor: 'pointer',
      background: '#8bac0f'

    }
    const { pcClicked } = this.props

    return (
      <div style={pcStyles} className="pc" onClick={pcClicked}>
        <div className="pcLabel">PC</div>
        <div className="pcValue">0x{toHex(pc, 4)}</div>
      </div>
    )
  }

  registers(registers: any): JSX.Element {
    const registersStyle = {
      display: 'flex',
      justifyContent: 'space-between',
      width: '150px',
      border: 'solid black 1px',
      padding: '5px'
    }
    return (
      <div style={registersStyle} className="registers">
        <div className="column1">
          {this.register("A", registers.a)}
          {this.register("B", registers.b)}
          {this.register("D", registers.d)}
          {this.register("H", registers.h)}
        </div>
        <div className="column2">
          {this.register("F", registers.f)}
          {this.register("C", registers.c)}
          {this.register("E", registers.e)}
          {this.register("L", registers.l)}
        </div>
      </div>
    )
  }

  sp(sp: number) {
    const spStyles: React.CSSProperties = {
      padding: '2px',
      textAlign: 'center',
      border: 'solid black 1px',
      borderTop: '0',
      cursor: 'pointer',
      background: '#333d87',
      color: 'white'

    }
    const { spClicked } = this.props

    return (
      <div style={spStyles} className="sp" onClick={spClicked}>
        <div className="spLabel">SP</div>
        <div className="spValue">0x{toHex(sp, 4)}</div>
      </div>
    )
  }

  render(): JSX.Element | null {
    const cpuStyles: React.CSSProperties = {
      display: 'flex',
      flexDirection: 'column',
      justifyContent: 'center'
    }
    const { cpu } = this.props
    const cpuAsJson = cpu.to_json()
    return (
      <div style={cpuStyles} className="cpu">
        {this.pc(cpuAsJson.pc)}
        {this.registers(cpuAsJson.registers)}
        {this.sp(cpuAsJson.sp)}
      </div>
    )
  }
}

export default CPU
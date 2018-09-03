import * as React from 'react'

import CPU from 'components/CPU'
import Memory from 'components/Memory'
// import Background from 'components/Background'
// import TileSet from 'components/TileSet'
import { CPU as CPUModel } from 'lib-dmg-01-js'
// import Debugger from 'Debugger'

const BYTE_SIZE = 8

type Props = {
    cpu: CPUModel,
    // isRunning: boolean,
    // step: () => void,
    // stepFrame: () => void,
    // addBreakPoint: (addr: number) => void
}
type State = {
    showInternals: boolean,
    memoryOffset: number,
}
class Internals extends React.Component<Props, State> {
    constructor(props: Props) {
        super(props)
        const { cpu } = this.props
        this.state = {
            memoryOffset: calculateMemoryOffset(cpu.to_json().pc),
            showInternals: false
        }
    }

  render(): JSX.Element {
    const internalsStyles = {
      width: '1000px',
    }
    const headerStyles = {
      borderColor: '#306230',
      background: '#8bac0f',
      color: 'white',
      padding: '5px',
      borderRadius: '3px',
      display: 'flex',
      justifyContent: 'space-between',
      border: '1px solid',
      cursor: 'pointer',
      width: '100%',
      margin: '2px'
    }
    const arrowStyles = this.state.showInternals ? {} : { transform: 'rotate(90deg)', } 
    return (
      <div style={internalsStyles} className="internals">
        <div style={headerStyles} className="header" onClick={this.toggleInternalsVisibility}>
          <div>Internals</div>
          <div style={arrowStyles} className={`directionArrow ${this.state.showInternals ? "open" : "closed"}`}>▼</div>
        </div>
        {this.content()}
      </div>
    )
  }

  content() {
    const contentStyles: React.CSSProperties = {
      display: 'flex',
      flexDirection: 'column',
      justifyContent: 'center'
    }
    const motherboardStyles = {
      display: 'flex',
      justifyContent: 'space-around'
    }
    const { showInternals } = this.state
    if (!showInternals) { return null }

    const { cpu } = this.props
    return (
      <div style={contentStyles} className="content">
        <div style={motherboardStyles} className="motherboard">
          <CPU cpu={cpu} pcClicked={this.pcClicked} spClicked={this.spClicked} />
          {this.memory()}
        </div>
        {this.props.children}
        {/* {this.controls()} */}
      </div>
    )
    }

    memory() {
        const { memoryOffset } = this.state
        const { cpu, /* addBreakPoint */ } = this.props
        const cpuJson = cpu.to_json()
        return (
            <div className="memory">
                <Memory
                    memorySlice={(start: number, end: number) => cpu.memory_slice(start, end)}
                    isBiosMapped={true}
                    pc={cpuJson.pc}
                    sp={cpuJson.sp}
                    offset={memoryOffset}
                    changeOffset={newOffset => this.setState({ memoryOffset: newOffset })}
                    /* onByteClick={addBreakPoint} */ />
                {/* <div className="visualMemory">
                    <TileSet
                        gpu={cpu.bus.gpu}
                        onClick={() => { }} />
                    <Background
                        gpu={cpu.bus.gpu}
                        onClick={this.backgroundClicked} /> */}
                {/* </div> */}
            </div>
        )
    }

    toggleInternalsVisibility = () => {
        this.setState({showInternals: !this.state.showInternals})
    }

    pcClicked = () => {
        const { cpu } = this.props
        this.setState({memoryOffset: calculateMemoryOffset(cpu.to_json().pc) })
    }

    spClicked = () => {
        const { cpu } = this.props
        this.setState({memoryOffset: calculateMemoryOffset(cpu.to_json().sp) })
    }

    // backgroundClicked = () => {
    //     this.setState({memoryOffset: Math.trunc(this.props.cpu.bus.gpu.backgroundTileMap / BYTE_SIZE) })
    // }

    // controls() {
    //     const { isRunning } = this.props
    //     if (isRunning) { return null }

    //     return (
    //         <div className="controls">
    //             {this.stepButton()}
    //             {this.stepFrameButton()}
    //         </div>
    //     )
    // }

    // stepFrameButton(): JSX.Element | null {
    //     return <button className="stepFrame" onClick={this.stepFrame}>Step Frame</button>
    // }

    // stepButton(): JSX.Element | null {
    //     return <button className="step" onClick={this.step}>Step</button>
    // }

    // step = () => {
    //     const { cpu, step } = this.props
    //     step()
    //     this.setState({ memoryOffset: calculateMemoryOffset(cpu)})
    // }

    // stepFrame = () => {
    //     const { cpu, stepFrame } = this.props
    //     stepFrame()
    //     this.setState({ memoryOffset: calculateMemoryOffset(cpu)})
    // }
}

function calculateMemoryOffset(pointer: number): number {
    return Math.trunc(pointer / BYTE_SIZE)
}

export default Internals
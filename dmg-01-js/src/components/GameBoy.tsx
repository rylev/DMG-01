import * as React from 'react'

import Screen from './Screen'
import _import from './dmg-01-js'

type Props = { bios: Uint8Array | undefined, rom: Uint8Array }
type State = { cpu?: any }

class Gameboy extends React.Component<Props, State> {
  constructor(props: Props) {
    super(props)
    _import().then((lib: any)  => {
      const cpu = lib.CPU.new(this.props.bios!, this.props.rom)
      this.setState({cpu})
    })
  }

  render(): JSX.Element {
    return (
      <div className="gameboy">
        <Screen />
      </div>
    )
  }

  runFrame(cpu: any, previousTimeDiff: number, runContinuously: boolean) {
    const t1 = new Date().getTime()
    setTimeout(() => {
      let cyclesElapsed = 0
      while (cyclesElapsed < 70224) {
        cyclesElapsed += cpu.step()
      }
      if (runContinuously) {

        const t2 = new Date().getTime()
        let timeDiff = 16.7 - (t2 - t1)

        if (previousTimeDiff < 0) { timeDiff = timeDiff + previousTimeDiff }

        this.runFrame(cpu, timeDiff, runContinuously)
      }
    }, Math.max(previousTimeDiff, 0))
  }
}

export default Gameboy
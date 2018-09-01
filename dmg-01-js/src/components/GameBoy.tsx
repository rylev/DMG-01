import * as React from 'react'

import Screen from 'components/Screen'

type Props = { bios: Uint8Array | undefined, rom: Uint8Array }
type State = {}
class Gameboy extends React.Component<Props, State> {
    constructor(props: Props) {
        super(props)
        this.state = {}
    }

    render(): JSX.Element {
        return (
            <div className="gameboy">
                <Screen />
            </div>
        )
    }
}

export default Gameboy
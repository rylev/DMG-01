import * as React from 'react'
import RomInput  from 'components/RomInput'
import Gameboy from 'components/Gameboy'

type Props = {
    bios?: Uint8Array,
    rom?: Uint8Array
}
type State = {
    bios?: Uint8Array,
    rom?: Uint8Array
}

class Main extends React.Component<Props, State> {
    constructor(props: Props) {
        super(props)
        this.state = { bios: this.props.bios , rom: this.props.rom }
    }

    romUploaded = (rom: Uint8Array) => {
        this.setState({rom: rom})
    }

    biosUploaded = (bios: Uint8Array) => {
        this.setState({bios: bios})
    }

    biosUploadMessage(): JSX.Element | null {
        if (this.state.bios !== undefined) {
            return <div>BIOS has been uploaded!</div>
        }
        return null
    }

    romLoading() {
        const { bios, rom } = this.state
        const romLabel = rom === undefined ? "Upload ROM" : "Change ROM"
        const biosLabel = bios === undefined ? "Upload BIOS" : "Change BIOS"
        const loadingStyles = {
            display: 'flex',
            justifyContent: 'center',
            alignItems: 'center'
        }
        return (
            <div style={loadingStyles} className="loading">
                <RomInput id="rom-input" romUploaded={this.romUploaded} label={romLabel} />
                <RomInput id="bios-input" romUploaded={this.biosUploaded} label={biosLabel} />
            </div>
        )
    }

    render () {
        const { bios, rom } = this.state

        if (rom == undefined) {
            return (<div className="page">
                {this.romLoading()}
            </div>)
        } else {
            return (
                <div>
                    <Gameboy bios={bios} rom={rom} />
                    {this.romLoading()}
                </div>
            )
        }
    }
}

export default Main
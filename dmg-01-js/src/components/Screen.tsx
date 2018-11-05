import * as React from 'react'
import * as ReactDOM from 'react-dom'

type Props = {
    buffer: Uint8Array
}
type State = {
    ctx?: CanvasRenderingContext2D
}

class Screen extends React.Component<Props, State> {
    constructor(props: Props) {
        super(props)
        this.state = {}
    }

    componentWillUnmount() {
        this.setState({ctx: undefined})
    }

    componentWillReceiveProps(newProps: Props) {
        this.drawCanvas(newProps.buffer)
    }

    drawCanvas = (data: Uint8Array) => {
        const ctx = this.getCtx()
        if (!ctx) { return }
        const imageData = ctx.createImageData(160, 144)
        for (let i=0; i < data.length; i+=4) {
            imageData.data[i]   = data[i];   //red
            imageData.data[i+1] = data[i+1]; //green
            imageData.data[i+2] = data[i+2]; //blue
            imageData.data[i+3] = data[i+3]; //alpha
        }
        
        ctx.putImageData(imageData, 0, 0)
    }

    getCtx() {
        const { ctx } = this.state
        if (ctx) { return ctx } 
        const canvas = ReactDOM.findDOMNode(this.refs.screen) as HTMLCanvasElement | null
        if (canvas === null) { return undefined }
        const newCtx = canvas.getContext('2d') || undefined

        this.setState({ctx: newCtx})

        return newCtx
    }

    canvas () {
        return <canvas 
            height={144}
            width={160}
            id="screen"
            ref="screen" />
    }

    render() {
        const screenStyles = {
            border: '1px solid black',
            width: '160px',
            height: '144px'
        }
        return (
            <div style={screenStyles} className="screen">
                {this.canvas()}
            </div>
        )
    }
}

export default Screen
import * as React from 'react'
import * as ReactDOM from 'react-dom'

type Props = {
  data: Uint8Array
  height: number,
  width: number,
  label: string,
  scale?: number,
  style?: React.CSSProperties,
  onClick?: () => void,
  onMouseMove?: (x: number, y: number) => void
  onMouseOut?: () => void
}
type State = { ctx?: CanvasRenderingContext2D }

class BufferedCanvas extends React.Component<Props, State> {
  constructor(props: Props) {
    super(props)
    this.state = {}
  }

  componentDidMount() {
    this.drawCanvas(this.props.data)
  }

  componentWillReceiveProps(newProps: Props) {
    this.drawCanvas(newProps.data)
  }

  drawCanvas(data: Uint8Array) {
    const ctx = this.getCtx()
    if (!ctx) { return }
    const imageData = ctx.createImageData(this.props.width, this.props.height)
    for (let i = 0; i < data.length; i += 4) {
      imageData.data[i] = data[i]   //red
      imageData.data[i + 1] = data[i + 1] //green
      imageData.data[i + 2] = data[i + 2] //blue
      imageData.data[i + 3] = data[i + 3] //alpha
    }
    ctx.putImageData(imageData, 0, 0)
  }

  getCtx() {
    if (this.state.ctx) { return this.state.ctx }
    const canvas = ReactDOM.findDOMNode(this.refs[this.props.label]) as HTMLCanvasElement | null
    if (canvas === null) { return }
    const ctx = canvas.getContext('2d') || undefined
    this.setState({ ctx })

    return ctx
  }

  onMouseMove(e: React.MouseEvent<HTMLCanvasElement>) {
    const rect = e.currentTarget.getBoundingClientRect()
    if (this.props.onMouseMove) {
      this.props.onMouseMove(e.clientX - rect.left, e.clientY - rect.top)
    }
  }

  render() {
    return <canvas
      style={this.props.style}
      height={this.props.height * (this.props.scale || 1)}
      width={this.props.width * (this.props.scale || 1)}
      id={this.props.label}
      ref={this.props.label}
      onMouseMove={(e) => this.onMouseMove(e)}
      onMouseOut={() => this.props.onMouseOut && this.props.onMouseOut()}
      onClick={this.props.onClick} />
  }
}

export default BufferedCanvas
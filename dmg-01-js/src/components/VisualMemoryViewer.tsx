import * as React from 'react'
import * as ReactDOM from 'react-dom'

type Props = {
  data: Uint8Array
  height: number,
  width: number,
  label: string,
  header: string,
  onClick?: () => void
}
type State = { ctx?: CanvasRenderingContext2D, isShowing: boolean }

class VisualMemoryViewer extends React.Component<Props, State> {
  constructor(props: Props) {
    super(props)
    this.state = { isShowing: false }
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
    const canvas = ReactDOM.findDOMNode(this.refs[this.props.label]) as HTMLCanvasElement | null
    if (canvas === null) { return }
    const ctx = canvas.getContext('2d') || undefined
    this.setState({ ctx })

    return ctx
  }

  canvas() {
    const canvasStyles = {
      border: '1px solid black'

    }
    return <canvas
      style={canvasStyles}
      height={this.props.height}
      width={this.props.width}
      id={this.props.label}
      ref={this.props.label}
      onClick={this.props.onClick} />
  }

  content() {
    if (!this.state.isShowing) { return null }
    return (
      <div>
        {this.canvas()}
        {this.props.children}
      </div>
    )
  }

  render() {
    const visualMemoryViewer: React.CSSProperties = {
      display: 'flex',
      flexDirection: 'column',
      alignItems: 'center'
    }
    const headerStyles = {
      borderColor: '#306230',
      background: '#8bac0f',
      color: 'white',
      padding: '5px',
      borderCadius: '3px',
      display: 'flex',
      justifyContent: 'space-between',
      border: '1px solid',
      cursor: 'pointer',
      width: '100%',
      margin: '2px',
    }
    const arrowStyles ={
      transform: !this.state.isShowing ? 'rotate(90deg)' : ''
    }
    return (
      <div style={visualMemoryViewer} className="visualMemoryViewer">
        <div style={headerStyles} className="header" onClick={this.toggleVisibility}>
          <div>{this.props.header}</div>
          <div style={arrowStyles}>▼</div>
        </div>
        {this.content()}
      </div>
    )
  }

  toggleVisibility = () => {
    this.setState({ isShowing: !this.state.isShowing }, () => {
      if (this.state.isShowing) {
        this.drawCanvas(this.props.data)
      }
    })
  }
}

export default VisualMemoryViewer
import * as React from 'react'
import BufferedCanvas from './BufferedCanvas'

type Props = {
  data: Uint8Array
  height: number,
  width: number,
  label: string,
  header: string,
  onClick?: () => void,
  onMouseMove?: (x: number, y: number) => void,
  onMouseOut?: () => void
}
type State = { isShowing: boolean }

class VisualMemoryViewer extends React.Component<Props, State> {
  constructor(props: Props) {
    super(props)
    this.state = { isShowing: false }
  }

  canvas() {
    const canvasStyles = {
      border: '1px solid black'
    }
    return <BufferedCanvas
      data={this.props.data}
      style={canvasStyles}
      height={this.props.height}
      width={this.props.width}
      label={this.props.label}
      onMouseMove={this.props.onMouseMove}
      onMouseOut={this.props.onMouseOut}
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
    this.setState({ isShowing: !this.state.isShowing })
  }
}

export default VisualMemoryViewer
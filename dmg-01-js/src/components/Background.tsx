import * as React from 'react'

import VisualMemoryViewer from './VisualMemoryViewer'
import BufferedCanvas from './BufferedCanvas'

type Tile = {
  data: Uint8Array,
  indices: { x: number, y: number }
}

type Props = {
  getData: (showTileOutlines: boolean, showViewportOutline: boolean) => Uint8Array,
  getTileAt: (x: number, y: number) => Tile
}
type State = {
  showTileOutlines: boolean
  showViewportOutline: boolean
  hoveredTile: Tile | undefined 
}
class Background extends React.Component<Props, State> {
  constructor(props: Props) {
    super(props)
    this.state = { showTileOutlines: false, showViewportOutline: false, hoveredTile: undefined }
  }

  onShowTileOutlinesChange = () => {
    this.setState({ showTileOutlines: !this.state.showTileOutlines })
  }

  onShowViewportOutline = () => {
    this.setState({ showViewportOutline: !this.state.showViewportOutline })
  }

  render() {
    return (
      <div>
        <VisualMemoryViewer
          data={this.props.getData(this.state.showTileOutlines, this.state.showViewportOutline)}
          height={256}
          width={256}
          header={"Background"}
          label={"background"} 
          onMouseMove={(x, y) => this.onMouseMove(x, y)}
          onMouseOut={() => this.onMouseOut()}
          >
          <div className="toggles">
            <div className="toggleItem">
              <input type="checkbox" id="showTileOutlines" onChange={this.onShowTileOutlinesChange} checked={this.state.showTileOutlines} />
              <label htmlFor="showTileOutlines">Tile Outlines</label>
            </div>
            <div className="toggleItem">
              <input type="checkbox" id="showViewportOutline" onChange={this.onShowViewportOutline} checked={this.state.showViewportOutline} />
              <label htmlFor="showViewportOutline">Viewport Outline</label>
            </div>
          </div>
          <div> 
            {this.hoveredTile()}
          </div>
        </VisualMemoryViewer>
      </div>
    )
  }

  hoveredTile() {
    if (!this.state.hoveredTile) { return null }
    return (
      <div>
        <BufferedCanvas 
          data={this.state.hoveredTile.data}
          height={8}
          width={8}
          label={"tile"}
          scale={10}
        />
        {JSON.stringify(this.state.hoveredTile.indices)}
      </div>
    )
  }

  onMouseMove(x: number, y: number) {
    const currentHoveredTile = this.state.hoveredTile
    const hoveredTile = this.props.getTileAt(x, y)
    if (!currentHoveredTile ||
          (hoveredTile.indices.x !== currentHoveredTile.indices.x || 
           hoveredTile.indices.y !== currentHoveredTile.indices.y)) {
      this.setState({hoveredTile})
    }
  }

  onMouseOut() {
      this.setState({hoveredTile: undefined})
  }
}

export default Background
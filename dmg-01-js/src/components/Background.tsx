import * as React from 'react'

import VisualMemoryViewer from './VisualMemoryViewer'

type Props = {
  getData: (showTileOutlines: boolean, showViewportOutline: boolean) => Uint8Array
}
type State = {
  showTileOutlines: boolean
  showViewportOutline: boolean
}
class Background extends React.Component<Props, State> {
  constructor(props: Props) {
    super(props)
    this.state = { showTileOutlines: false, showViewportOutline: false }
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
          label={"background"} >
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
        </VisualMemoryViewer>
      </div>
    )
  }
}

export default Background
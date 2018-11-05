import * as React from 'react'

import VisualMemoryViewer from 'components/VisualMemoryViewer'

type Props = { 
    getData: (outlineTiles: boolean) => Uint8Array
}
type State = {}
class TileSet extends React.Component<Props, State> {
    render() {
        return (
            <VisualMemoryViewer
                data={this.props.getData(false)}
                height={128}
                width={192}
                header={"TileSet"}
                label={"tileSet"} />
        )
    }
}

export default TileSet
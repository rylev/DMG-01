import * as React from 'react'

import { toHex } from 'lib/hex'

type Slice = (start: number, end: number) => Uint8Array
type Props = {
  isBiosMapped: boolean,
  pc: number,
  sp: number,
  offset: number,
  memorySlice: Slice,
  changeOffset: (newOffset: number) => void,
  // onByteClick: (addr: number) => void
}
type State = {}
const ROW_COUNT = 16
const OFFSET_COUNT = 8192
const MAX_OFFSET = OFFSET_COUNT - ROW_COUNT

class Memory extends React.Component<Props, State> {
  constructor(props: Props) {
    super(props)
  }

  render(): JSX.Element | null {
    const memoryViewerStyles: React.CSSProperties = {
      height: '550px',
      display: 'flex',
      flexDirection: 'column',
      justifyContent: 'center'
    }
    const memoryStyles: React.CSSProperties = {
      maxHeight: '460px',
      minHeight: '460px',
      maxWidth: '320px',
      minWidth: '320px',
      display: 'flex',
      flexDirection: 'column',
      justifyContent: 'center',
      fontFamily: 'monospace'
    }
    const memoryMoveArrowStyles: React.CSSProperties = {
      height: '20px',
      backgroundColor: '#9c2859',
      borderColor: 'black',
      color: 'white',
      padding: '5px',
      display: 'flex',
      justifyContent: 'center',
      alignItems: 'center',
      cursor: 'pointer',
      borderRadius: '3px',
      boxShadow: '0px 0px 5px black',
      userSelect: 'none'
    }
    const { memorySlice } = this.props
    const normalizedOffset = this.normalizedOffset()
    const options = { size: 8, count: ROW_COUNT, offset: normalizedOffset }
    const rows = mapChunk(memorySlice, options, (chunk, i) => this.row(chunk, options.size, i))
    const moveUpButtonDiabled = normalizedOffset >= MAX_OFFSET
    const moveDownButtonDisabled = normalizedOffset <= 0
    const moveDown = () => { if (!moveDownButtonDisabled) { this.moveDown() } }
    const moveUp = () => { if (!moveUpButtonDiabled) { this.moveUp() } }
    return (
      <div style={memoryViewerStyles} className="memoryViewer">
        <div style={memoryMoveArrowStyles} className="memoryMove memoryDown" onClick={moveDown}>▲</div>
        <div style={memoryStyles} id="memory">
          {rows}
        </div>
        <div style={memoryMoveArrowStyles} className="memoryMove memoryUp" onClick={moveUp}>▼</div>
      </div>
    )
  }

  byte(address: number, value: number): JSX.Element {
    const { pc, sp } = this.props
    const isPC = pc === address 
    const isSP = sp === address 

    const byteStyles: React.CSSProperties = {
      display: 'inline-block',
      padding: '2px',
      width: '15px',
      textAlign: 'center',
      margin: '4px',
      cursor: 'pointer',
      background: isPC ? '#8bac0f' : isSP ? '#333d87' : '',
      borderRadius: isPC ? '2px' : '',
      boxShadow: isPC ? '0 0 1px black' : ''
    }
    return (
      <div style={byteStyles} className={`byte ${isPC} ${isSP}`} key={address} /* onClick={() => this.props.onByteClick(address)} */>
        {toHex(value, 2)}
      </div>
    )
  }

  row(chunk: Uint8Array, numberOfBytes: number, index: number): JSX.Element {
    const firstByteAddress = (this.normalizedOffset() * numberOfBytes) + (index * numberOfBytes)
    const isHeader = firstByteAddress >= 0x0100 && firstByteAddress < 0x014F ? 'isHeader' : ''
    const isBios = firstByteAddress < 0x0100 && this.props.isBiosMapped ? 'isBios' : ''
    const memoryRowStyles = {
      display: 'flex',
      justifyContent: 'space-between',
      alignItems: 'center',
      marginLeft: '10px',
      padding: '0px 5px',
      background: isBios ? 'rgba(150, 53, 136, 0.4)' : isHeader ?  'rgba(53, 150, 53, 0.4)': ''
    }
    const memoryRowAddressStyles: React.CSSProperties = {
      display: 'inline-block',
      fontWeight: 'bold'
    }
    const bytesStyles = {
      display: 'inline-block'
    }
    const bytes = Array.from(chunk).map((byte, i) => this.byte(firstByteAddress + i, byte))

    return (
      <div style={memoryRowStyles} className={`memoryRow ${isHeader} ${isBios}`} key={index}>
        <div style={memoryRowAddressStyles} className="rowAddress">0x{toHex(firstByteAddress, 3)}: </div>
        <div style={bytesStyles} className="bytes">{bytes}</div>
      </div>
    )
  }

  normalizedOffset() {
    return Math.max(0, Math.min(this.props.offset, MAX_OFFSET))
  }

  moveDown = () => {
    this.props.changeOffset(this.normalizedOffset() - 1)
  }

  moveUp = () => {
    this.props.changeOffset(this.normalizedOffset() + 1)
  }
}

type ChunkOptions = { size: number, count: number, offset: number }
function mapChunk<T>(slice: Slice, chunkOptions: ChunkOptions, callback: (slice: Uint8Array, index: number) => T): T[] {
  const result: T[] = []
  let index = 0
  while (index < chunkOptions.count) {
    const start = (chunkOptions.offset * chunkOptions.size) + (index * chunkOptions.size)
    const end = start + chunkOptions.size
    result.push(callback(slice(start, end), index))
    index = index + 1
  }

  return result
}

export default Memory
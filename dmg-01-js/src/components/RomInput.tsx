import * as React from 'react'

type Props = {
    label: string,
    id: string,
    romUploaded: (rom: Uint8Array) => void
}

const romUploadButtonStyles: React.CSSProperties = {
  margin: '10px'
}
const romInputStyles: React.CSSProperties = {
  width: '1px', 
  height: '1px', 
  position: 'absolute', 
  fontSize: 0,
  opacity: 0,
  pointerEvents: 'none'
}
const romInputLabelStyles: React.CSSProperties = {
  cursor: 'pointer',
  width: '100px',
  height: '30px',
  background: '#3498db',
  borderRadius: '28px',
  fontFamily: 'Arial',
  color: '#ffffff',
  fontSize: '20px',
  padding: '10px 20px 10px 20px'
}


class RomInput extends React.Component<Props, {}> {
  onChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const input = e.currentTarget
    const file = input.files && input.files[0]
    if (!file) { return }
    const contents = new Blob([file])
    this.handleFileContents(contents)
  }

  handleFileContents(contents: Blob) {
    const arrayReader = new FileReader()
    arrayReader.readAsArrayBuffer(contents)
    arrayReader.onload = () => {
      const rom = new Uint8Array(arrayReader.result as ArrayBuffer)
      this.props.romUploaded(rom)
    }

    const urlReader = new FileReader()
    urlReader.readAsDataURL(contents)
    urlReader.onload = () => {
      try {
        localStorage.setItem(this.props.id, urlReader.result as string)
      } catch (e) {
        // TODO: Handle Error
        console.error("Storage failed: " + e)
      }
    }
  }

  render() {
    return (
      <div style={romUploadButtonStyles}>
        <input id={this.props.id} style={romInputStyles} type="file" accept=".gb" onChange={this.onChange} />
        <label htmlFor={this.props.id} style={romInputLabelStyles}>
          {this.props.label}
        </label>
      </div>
    )
  }
}


export default RomInput
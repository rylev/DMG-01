const hoverStateClassName = 'hover'
window.onload = function() {
  const elements = $('td')
  for (element of elements) {
    element.onmouseover = function() {
      onRowAndColumn(elements, this.className, function (element) {
        element.className += ' ' + hoverStateClassName
      })
    }

    element.onmouseout = function() {
      onRowAndColumn(elements, this.className, function (element) {
        element.className = element.className.replace(' ' + hoverStateClassName, '')
      })
    }
  }
}

function onRowAndColumn(elements, className, callBack) {
  const column = columnClassName(className)
  const row = rowClassName(className)
  for (element of elements) {
    const columnName = columnClassName(element.className)
    const rowName = rowClassName(element.className)
    if (columnName === column || rowName === row) {
      callBack(element)
    }
  }
}

function columnClassName(fullClassName) {
  const regex = /column\d{1,2}/
  return fullClassName.match(regex)[0]
}
function rowClassName(fullClassName) {
  const regex = /row\d{1,2}/
  return fullClassName.match(regex)[0]
}


function runInstruction(instruction, target) {
  switch (instruction) {
    case 'ADD':
      performInstruction('ADD', target)
      return
  }
}

function performInstruction(instructionName, target) {
  const instructionDiv = $('#' + instructionName.toUpperCase())
  const cpu = getCpuForInstruction(instructionDiv, getRadixValue(instructionDiv))
  const newCpu = wasm_bindgen[instructionName.toLowerCase()](cpu, wasm_bindgen.Register[target])
  setCpuForInstruction(instructionDiv, newCpu, getRadixValue(instructionDiv))
}

function getCpuForInstruction(instructionDiv, radix) {
  const registerDivs = getRegisterDivs(instructionDiv)
  const cpu = new wasm_bindgen.CPU()
  registerDivs.forEach(registerDiv => {
    setRegisterOnCpu(cpu, registerDiv, wasm_bindgen.Register, radix)
  })
  return cpu
}

function setCpuForInstruction(instructionDiv, cpu, radix) {
  const registerDivs = getRegisterDivs(instructionDiv)
  registerDivs.forEach(registerDiv => {
    setRegisterDivValue(registerDiv, cpu.to_json(), radix)
  })
}

function getRadixValue(instructionDiv) {
  return instructionDiv.find($('input[name=radix]:checked'))[0].value
}

function setRegisterOnCpu(cpu, registerDiv, registerNames, currentRadixName) {
  let registerName = registerDiv.className.replace('register-', '').toUpperCase()
  const registerValue = $(registerDiv).find($('.register-value'))[0]
  const valueString = registerName === 'F' ? $(registerValue).text() : registerValue.value
  const value = parseInt(valueString, radixNameToNumber(currentRadixName))
  registerName = registerNames[registerName]
  cpu.set_register(registerName, value)
}

function setRegisterDivValue(registerDiv, cpu, radixName) {
  const registerName = registerDiv.className.replace('register-', '')
  const valueDiv = $(registerDiv).find($('.register-value'))[0]
  const value = cpu.registers[registerName]
  const valueString = value.toString(radixNameToNumber(radixName))
  if (registerName === 'f') {
    $(valueDiv).text(radixPrefix(radixName) + valueString)
  } else {
    valueDiv.value = valueString
  }
}

function radixPrefix(radixName) {
  if (radixName === 'hexadecimal') {
    return '0x'
  } else if (radixName === 'binary') {
    return '0b'
  } else {
    return ''
  }
}
function radixNameToNumber(radixName) {
  return radixName === 'hexadecimal' ? 16 : radixName === 'binary' ? 2 : 10
}

function getRegisterDivs(instructionDiv) {
  return instructionDiv.find($('.cpu')).children().toArray()
}

const registers = ['A', 'B', 'C', 'D', 'E', 'F', 'H', 'L']
let radixValue = 'decimal'
function addPlayground(instruction) {
  const instructionDiv = $('#' + instruction)
  const playground = $('<div/>', { class: 'playground', })
  const cpu = $('<div/>', { class: 'cpu', }).appendTo(playground)

  for (register of registers) {
    const registerDiv = $('<div/>', {
      class: 'register-' + register.toLowerCase(),
    }).appendTo(cpu)

    $('<p/>', { })
      .text(register + ':')
      .css({display:'inline'})
      .appendTo(registerDiv)

    if (register === 'F') {
      $('<p/>', { class: 'register-value' })
        .text(0)
        .css({display:'inline'})
        .appendTo(registerDiv)
    } else {
      $('<input/>', {
        class: 'register-value',
        type: 'number',
        value: 0,
        min: 0,
        max: 255
      }).appendTo(registerDiv)
    }
  }

  ['decimal', 'binary', 'hexadecimal'].forEach(name => {
    $('<input/>', {
      type: 'radio',
      name: 'radix',
      value: name,
      checked: name === 'decimal',
      change: e => {
        const cpu = getCpuForInstruction(instructionDiv, radixValue)
        radixValue = e.currentTarget.value
        setCpuForInstruction(instructionDiv, cpu, radixValue)
      }
    }).appendTo(playground)
    $('<p/>', { })
      .text(name)
      .css({display: 'inline'})
      .appendTo(playground)
  })

  $('<button/>', {
    text: 'Run',
    click: () => { runInstruction(instruction, 'A') }
  }).appendTo(playground)


  instructionDiv.append(playground)
}
wasm_bindgen('./cpu_js_bg.wasm').then(() => {
  addPlayground('ADD')
})

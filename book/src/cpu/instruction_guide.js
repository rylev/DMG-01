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
  const cpu = getCpuForInstruction(instructionDiv)
  const newCpu = wasm_bindgen[instructionName.toLowerCase()](cpu, wasm_bindgen.Register[target])
  setCpuForInstruction(instructionDiv, newCpu, getRadixValue(instructionDiv))
}

function getCpuForInstruction(instructionDiv) {
  const registerDivs = getRegisterDivs(instructionDiv)
  const cpu = new wasm_bindgen.CPU()
  registerDivs.forEach(registerDiv => {
    setRegisterOnCpu(cpu, registerDiv, wasm_bindgen.Register)
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

function setRegisterOnCpu(cpu, registerDiv, registerNames) {
  const registerName = registerDiv.className.replace('register-', '').toUpperCase()
  const registerValue = $(registerDiv).find($('.register-value'))[0]
  let valueString = registerName === 'F' ? $(registerValue).text() : registerValue.value
  const radixName = valueString.includes('0b') ? 'binary' : valueString.includes('0x') ? 'hexadecimal' : 'decimal'
  valueString = valueString.replace('0b', '').replace('0x', '')
  const value = parseInt(valueString, radixNameToNumber(radixName))
  cpu.set_register(registerNames[registerName], value)
}

function setRegisterDivValue(registerDiv, cpu, radixName) {
  const registerName = registerDiv.className.replace('register-', '')
  const valueDiv = $(registerDiv).find($('.register-value'))[0]
  const value = cpu.registers[registerName]
  if (registerName === 'f') {
    const valueString = value.toString(radixNameToNumber(radixName))
    $(valueDiv).text(radixPrefix(radixName) + valueString)
  } else {
    valueDiv.value = value.toString()
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
  if (radixName === 'hexadecimal') {
    return 16
  } else if (radixName === 'binary') {
    return 2
  } else {
    return 10
  }
}

function getRegisterDivs(instructionDiv) {
  return instructionDiv.find($('.cpu')).children().toArray()
}

wasm_bindgen('./cpu_js_bg.wasm').then(() => {
  CPU.mount($('#ADD .playground')[0])
})

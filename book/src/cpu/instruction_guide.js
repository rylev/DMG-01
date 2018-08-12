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


function runInstruction(instruction) {
  switch (instruction) {
    case 'ADD':
      runADD()
      return
  }
}

function runADD() {
  const registers = $('#ADD').find($('.cpu')).children().toArray()
  wasm_bindgen('./cpu_js_bg.wasm').then(() => {
    const cpu = new wasm_bindgen.CPU()
    registers.forEach(registerDiv => {
      const registerName = wasm_bindgen.Register[registerDiv.className.replace('register-', '').toUpperCase()]
      const value = $(registerDiv).find($('input'))[0].value
      cpu.set_register(registerName, value)
    })
    const newCpu = wasm_bindgen.add(cpu, 0)
    const json = newCpu.to_json()
    registers.forEach(registerDiv => {
      const registerName = registerDiv.className.replace('register-', '')
      const value = $(registerDiv).find($('input'))[0].value = json.registers[registerName]
    })
  });
}

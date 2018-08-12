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
  console.log(instruction)
  switch (instruction) {
    case 'ADD':
      runADD(target)
      return
  }
}

function runADD(target) {
  const registers = $('#ADD').find($('.cpu')).children().toArray()
  wasm_bindgen('./cpu_js_bg.wasm').then(() => {
    const cpu = new wasm_bindgen.CPU()
    registers.forEach(registerDiv => {
      const registerName = wasm_bindgen.Register[registerDiv.className.replace('register-', '').toUpperCase()]
      const value = $(registerDiv).find($('input'))[0].value
      cpu.set_register(registerName, value)
    })
    target = wasm_bindgen.Register[target]
    const newCpu = wasm_bindgen.add(cpu, target)
    const json = newCpu.to_json()
    registers.forEach(registerDiv => {
      const registerName = registerDiv.className.replace('register-', '')
      const value = $(registerDiv).find($('input'))[0].value = json.registers[registerName]
    })
  });
}

const registers = ['A', 'B', 'C', 'D', 'E', 'F', 'H', 'L']
function addPlayground(instruction) {
  const parent = $('#' + instruction)
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

    $('<input/>', {
      type: 'number',
      min: 0,
      max: 255
    }).appendTo(registerDiv)
  }

  $('<button/>', {
    text: 'Run',
    click: () => { runInstruction(instruction, 'A') }
  }).appendTo(playground)

  parent.append(playground)
}

addPlayground('ADD')

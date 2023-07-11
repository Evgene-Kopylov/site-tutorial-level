// find the output element
const output = document.getElementById("output");
// initializing the codemirror and pass configuration to support python and dracula theme
const editor = CodeMirror.fromTextArea(document.getElementById("code"), {
              mode: {
                  name: "python",
                  version: 3,
                  singleLineStringErrors: false,
              },
              theme: "dracula",
              lineNumbers: true,
              indentUnit: 4,
              matchBrackets: true,
            });

const function_name = "function";

// set the initial value of the editor
editor.setValue(`
def ${function_name}(
    enemy_units: [(int, int)],
    self_pos: (int, int)
) -> (int, int):
    """
    Выбери цель для стрельбы
    
    @enemy_units: список координат коричневых целей 
    @return: точка прицеливания
    """
    
    ### Вставьте вашь код сюда
    
    distance = 10000
    target = (0, 0)
    for unit in enemy_units:
        dx = self_pos[0] - unit[0]
        dy = self_pos[1] - unit[1]
        dis = (dx * dx + dy * dy) ** 0.5
        if dis < distance:
            distance = dis
            target = (unit[0], unit[1])

    return target
`);
output.value = "Initializing...\n";

// Add pyodide returned value to the output
function addToOutput(stdout) {
  output.value += stdout;
  output.scrollTop = output.scrollHeight;
}

// Clean the output section
function clearHistory() {
  output.value = "";
}

// init Pyodide and show sys.version when it's loaded successfully
async function main() {
  let pyodide = await loadPyodide();
  await pyodide.loadPackage('numpy');

    output.value = pyodide.runPython(`
import sys
sys.version
  `);
  output.value += `
Python ${output.value.split(" ")[0]}

`;

  // Версия Python в заголовке IDE
  let element = window.document.getElementById("python_version");
  element.innerText = output.value.split(" ")[0];

  return pyodide;
}
// run the main funciton
let pyodideReadyPromise = main();

function setParameter(name, value) {
    const queryString = window.location.search;
    const urlParams = new URLSearchParams(queryString);
    urlParams.set(name, value);
    const newurl = window.location.protocol
        + "//" + window.location.host
        + window.location.pathname
        + '?' + urlParams;
    window.history.pushState({path:newurl},'',newurl);
}

function getParameterValue(name) {
    const queryString = window.location.search;
    const urlParams = new URLSearchParams(queryString);
    return decodeURI(urlParams.get(name));
}


function sleep(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
}

// pass the editor value to the pyodide.runPython function and show the result in the output section
async function evaluatePython(test = 'all') {
    let target_pos = getParameterValue('target_pos');
    let enemy_units = getParameterValue('enemy_units');
    let unit_pos = getParameterValue('unit_pos');

    let pyodide = await pyodideReadyPromise;
    try {
        // test_positions = [
        //     '(100, 100)',
        //     '(136, 275)',
        //     '(430, 361)',
        //     '(489, 204)'
        // ]
        // test_positions.push(unit_pos);

        // for (target of enemy_units) {
            console.log(enemy_units);
            pyodide.runPython(`    
import io
sys.stdout = io.StringIO()

enemy_units = ${enemy_units}
self_pos = ${unit_pos}
        `);
        pyodide.runPython(`
${editor.getValue()}    
target = ${function_name}(enemy_units, self_pos)
print(target)
        `);


            let stdout = pyodide.runPython("sys.stdout.getvalue()");

            let result = stdout.toString().trim().split("\n")
            console.log(result);
            // addToOutput(`>>> ${function_name}(${target_pos}, ${unit_pos})\n` + stdout);

            let target = result[0].replace('(', '').replace(')', '').split(', ');
            console.log(target)
            setParameter("target_point_x", target[0]);
            setParameter("target_point_y", target[1]);

            // setParameter("unit_position_y", unit_pos.replace('(', '').replace(')', '').split(', ')[1]);
            // setParameter("rotation", rotation.toString().trim());

            setParameter("command", "Shoot");
            await sleep(1000);
        // };
  } catch (err) {
    addToOutput(err);
  }
}
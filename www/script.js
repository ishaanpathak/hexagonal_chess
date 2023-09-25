const hexagonAList = [7, 21, 35, 10, 24, 38, 52, 66, 80, 41, 55, 69, 83, 97, 58, 72, 86, 100, 114, 128, 89, 103, 117, 131, 145, 120, 134, 148, 162, 165];
const hexagonBList = [6, 20, 9, 23, 37, 51, 65, 26, 40, 54, 68, 82, 96, 57, 71, 85, 99, 113, 74, 88, 102, 116, 130, 144, 105, 119, 133, 147, 161, 150, 164];
const hexagonCList = [5, 8, 22, 36, 50, 25, 39, 53, 67, 81, 42, 56, 70, 84, 98, 112, 73, 87, 101, 115, 129, 90, 104, 118, 132, 146, 160, 135, 149, 163];

let defaultColors = {
    "color1": getComputedStyle(document.documentElement).getPropertyValue('--color-1'),
    "color2": getComputedStyle(document.documentElement).getPropertyValue('--color-2'),
    "color3": getComputedStyle(document.documentElement).getPropertyValue('--color-3'),
}

function decodeIntegers(encodedValue) {
    const x = (encodedValue & 0xF0) >> 4;
    const y = encodedValue & 0x0F;
    return [x, y];
}

function encodeCoordinatesBitwise(x, y) {
    return (x << 4) | y;
}

function createHexagonChessBoard() {
    const boardHexagonCounts = [
        [0, 10, 6], [0, 9, 7], [0, 8, 8], [0, 7, 9], [0, 6, 10], [0, 5, 11],
        [1, 4, 10], [2, 3, 9], [3, 2, 8], [4, 1, 7], [5, 0, 6]
    ].reverse();
    const board = document.getElementById("board");
    board.innerHTML = "";
    for (var i = 0; i < 11; i++) {
        var start = boardHexagonCounts[i][0];
        var y = boardHexagonCounts[i][1];
        var length = boardHexagonCounts[i][2];
        var row = document.createElement("div");
        row.className = "row";
        for (var j = start; j < start + length; j++) {
            var hexagon = document.createElement("div");
            if (hexagonAList.includes(encodeCoordinatesBitwise(j, y))) {
                hexagon.className = "hexagon-a hexagon-common";
            } else if (hexagonBList.includes(encodeCoordinatesBitwise(j, y))) {
                hexagon.className = "hexagon-b hexagon-common";
            } else if (hexagonCList.includes(encodeCoordinatesBitwise(j, y))) {
                hexagon.className = "hexagon-c hexagon-common";
            } else {
                hexagon.className = "hexagon";
            }
            hexagon.id = j + "," + y;
            var hexagonText = document.createElement("span");
            hexagonText.className = "rot";
            hexagonText.innerHTML = j + "," + y;
            hexagonText.innerHTML = "";
            hexagon.appendChild(hexagonText);
            row.appendChild(hexagon);
        }
        board.appendChild(row);
    }
};

var selectedHexagon = [-1, -1];

function addHexagonClickEvents() {
    const hexagonElements = document.getElementsByClassName("hexagon-common");
    for (let i = 0; i < hexagonElements.length; i++) {
        hexagonElements[i].addEventListener("click", function () {
            const [x, y] = hexagonElements[i].id.split(",").map((value) => parseInt(value));
            if (selectedHexagon[0] === -1 && selectedHexagon[1] === -1) {
                hexagonElements[i].style.setProperty("--color-1", "var(--selected)");
                hexagonElements[i].style.setProperty("--color-2", "var(--selected)");
                hexagonElements[i].style.setProperty("--color-3", "var(--selected)");
                selectedHexagon = [x, y];
                return;
            } else if (selectedHexagon[0] === x && selectedHexagon[1] === y) {
                hexagonElements[i].style.setProperty("--color-1", defaultColors["color1"]);
                hexagonElements[i].style.setProperty("--color-2", defaultColors["color2"]);
                hexagonElements[i].style.setProperty("--color-3", defaultColors["color3"]);
                selectedHexagon = [-1, -1];
                return;
            } else {
                const [oldX, oldY] = selectedHexagon;
                const oldHexagonElement = document.getElementById(oldX + "," + oldY);

                oldHexagonElement.style.setProperty("--color-1", defaultColors["color1"]);
                oldHexagonElement.style.setProperty("--color-2", defaultColors["color2"]);
                oldHexagonElement.style.setProperty("--color-3", defaultColors["color3"]);

                hexagonElements[i].style.setProperty("--color-1", "var(--selected)");
                hexagonElements[i].style.setProperty("--color-2", "var(--selected)");
                hexagonElements[i].style.setProperty("--color-3", "var(--selected)");

                selectedHexagon = [x, y];
                return;
            }
        });
    }
}

function resetHexagonColors() {
    const hexagonElements = document.getElementsByClassName("hexagon-common");
    for (let i = 0; i < hexagonElements.length; i++) {
        hexagonElements[i].style.setProperty("--color-1", defaultColors["color1"]);
        hexagonElements[i].style.setProperty("--color-2", defaultColors["color2"]);
        hexagonElements[i].style.setProperty("--color-3", defaultColors["color3"]);
    }
}

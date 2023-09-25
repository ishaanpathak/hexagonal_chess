import * as wasm from "hexagonal_chess";

const game = wasm.Game.new();
let board = JSON.parse(game.get_board());
let moves = JSON.parse(game.get_current_player_moves());

const getBoard = () => {
    const entries = Object.entries(board);
    for (const [key, value] of entries) {
        const hexagonElement = document.getElementById(key);
        if (hexagonElement) {
            const innerElements = hexagonElement.getElementsByTagName("span");
            if (innerElements.length > 0) {
                innerElements[0].innerHTML = value;
            }
        }
    }
}

document.getElementById("get-board").addEventListener("click", () => {
    getBoard();
});

document.getElementById("get-moves").addEventListener("click", () => {
    console.log(moves);
});

const allHexagons = document.querySelectorAll(".hexagon-common");
for (const hexagon of allHexagons) {
    hexagon.addEventListener("click", () => {
        resetHexagonColors();
        const selectedId = hexagon.id;
        const storedId = sessionStorage.getItem("selectedHexagonId");
        if (storedId === selectedId) {
            sessionStorage.setItem("lastSelectedHexagonId", selectedId);
            sessionStorage.setItem("selectedHexagonId", "");
            return;
        }
        const selectedMoves = moves[selectedId];
        if (selectedMoves) {
            for (const currentMove of selectedMoves) {
                const currentHexagon = document.getElementById(currentMove);
                if (currentHexagon) {
                    currentHexagon.style.setProperty("--color-1", "var(--highlighted)");
                    currentHexagon.style.setProperty("--color-2", "var(--highlighted)");
                    currentHexagon.style.setProperty("--color-3", "var(--highlighted)");
                }
            }
        }
        hexagon.style.setProperty("--color-1", "var(--selected)");
        hexagon.style.setProperty("--color-2", "var(--selected)");
        hexagon.style.setProperty("--color-3", "var(--selected)");
        // selectedHexagon = selectedId.split(",").map((value) => parseInt(value));
        sessionStorage.setItem("lastSelectedHexagonId", storedId);
        sessionStorage.setItem("selectedHexagonId", selectedId);
    });
}

const resetBoard = () => {
    for (const hexagon of allHexagons) {
        const innerElements = hexagon.getElementsByTagName("span");
        if (innerElements.length > 0) {
            innerElements[0].innerHTML = "";
        }
    }
}

const refreshBoard = () => {
    resetBoard();
    board = JSON.parse(game.get_board());
    moves = JSON.parse(game.get_current_player_moves());
    const entries = Object.entries(board);
    for (const [key, value] of entries) {
        const hexagonElement = document.getElementById(key);
        if (hexagonElement) {
            const innerElements = hexagonElement.getElementsByTagName("span");
            if (innerElements.length > 0) {
                innerElements[0].innerHTML = value;
            }
        }
    }
}

for (const hexagon of allHexagons) {
    hexagon.addEventListener("click", () => {
        const sessionHexagonId = sessionStorage.getItem("lastSelectedHexagonId");
        if (sessionHexagonId !== "") {
            const allowedMoves = moves[sessionHexagonId];
            if (allowedMoves && allowedMoves.includes(hexagon.id)) {
                console.log("Move from " + sessionHexagonId + " to " + hexagon.id);
                game.make_move(sessionHexagonId, hexagon.id);
                game.switch_player();
                refreshBoard();
                resetHexagonColors();
                if (game.is_in_check()) {
                    console.log("Check!");
                    const entries = Object.entries(board);
                    for (const [key, value] of entries) {
                        if (value === "♔" || value === "♚") {
                            if (moves[key]) {
                                const kingHexagon = document.getElementById(key);
                                if (kingHexagon) {
                                    kingHexagon.style.setProperty("--color-1", "var(--check)");
                                    kingHexagon.style.setProperty("--color-2", "var(--check)");
                                    kingHexagon.style.setProperty("--color-3", "var(--check)");
                                }
                            }
                        }
                    }
                }
            }
        } 
    });
}

// const container = document.getElementById("board");

// container.addEventListener("click", function(event) {
//   const clickedElement = event.target;

//   if (sessionStorage.getItem("selectedHexagonId") !== "") {
//     const from = sessionStorage.getItem("selectedHexagonId");
//     const to = clickedElement.id;
//     if (moves[from] && moves[from].includes(to)) {
//         console.log("Move from " + from + " to " + to);
//     }
//     // resetHexagonColors();
//     // sessionStorage.setItem("selectedHexagonId", "");
//     }
// });`
import { Map } from "dnd-house-monitor";
import { seed, removeAllChildNodes } from "./utils"

const MAP = Map.new()
const ENTITY_LIST = document.getElementById("entityList")
const ADD_BTN = document.getElementById("addBtn")
const NEXT_BTN = document.getElementById("nextBtn")
var SELECTED_ID = null
var SELECTED_FOR_TURN_ID = null

seed(MAP)
showEntities()
setupButtons()

function showEntities() {
    removeAllChildNodes(ENTITY_LIST)
    var entities = MAP.entities()

    console.log(entities);

    for (let e of entities) {
        const node = document.createElement("div")
        node.textContent = `${e.id}  ${e.initiative}`
        node.classList.add("entity")
        if (e.id == SELECTED_FOR_TURN_ID)
            node.classList.add("selectedForTurn")
        if (e.id == SELECTED_ID)
            node.classList.add("selected")
        if (e.is_player === true) {
            node.textContent = node.textContent.concat(" PLAYER")
        }
        node.onclick = function () {
            SELECTED_ID = e.id
            showEntities()
        }


        ENTITY_LIST.appendChild(node)
    }
}

function setupButtons() {
    ADD_BTN.onclick = function (e) {
        let id = document.getElementById("idInput").value
        let initiative = document.getElementById("initiativeInput").value

        MAP.add_entity(id)
        MAP.set_initiative(id, initiative)

        showEntities()
    }
    NEXT_BTN.onclick = function (e) {
        SELECTED_FOR_TURN_ID = MAP.next()
        showEntities()
    }
}



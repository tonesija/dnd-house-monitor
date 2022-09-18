import { Map } from "dnd-house-monitor";

const MAP = Map.new()
const ENTITY_LIST = document.getElementById("entityList")
const ADD_BTN = document.getElementById("addBtn")
const NEXT_BTN = document.getElementById("nextBtn")


showEntities()
setupButtons()

function showEntities() {
    removeAllChildNodes(ENTITY_LIST)
    var entities = MAP.entities()
    entities.sort((a, b) => {
        return b.initiative - a.initiative
    })
    for (let e of entities) {
        console.log(e)
        const node = document.createElement("div")
        node.textContent = `${e.id}  ${e.initiative}`
        node.classList.add("entity")

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
}

function removeAllChildNodes(parent) {
    while (parent.firstChild) {
        parent.removeChild(parent.firstChild);
    }
}
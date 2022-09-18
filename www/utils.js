function seed(map) {
    map.add_entity("a", true)
    map.set_initiative("a", 1)
    map.add_entity("b", false)
    map.set_initiative("b", 2)
    map.add_entity("c", true)
    map.set_initiative("c", 3)
}
function removeAllChildNodes(parent) {
    while (parent.firstChild) {
        parent.removeChild(parent.firstChild);
    }
}
export { seed, removeAllChildNodes }
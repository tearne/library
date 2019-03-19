export function drawGrid(svg, columns, rows, drawFn){
    // var bBox = svg.getBBox();
    let width = svg.node().getBoundingClientRect().width
    let height = svg.node().getBoundingClientRect().height

    let cellWidth = width / columns
    let cellHeight = height / rows

    for (var r = 0; r < rows; r++) {
        for (var c = 0; c < columns; c++) {
            let sub = svg
                .append("svg")
                .attr("transform", "translate("+ (cellWidth * c) +","+ (cellHeight * r) +")")
                .attr("width", cellWidth)
                .attr("height", cellHeight)

            drawFn(sub, r, c)
        }
    }
}
export function drawGrid(svg, margin, columns, rows, drawFn) {

    let chartGroup = svg
        .append("g")
        .attr("transform", "translate(" + margin.left + "," + margin.top + ")");

    let width = svg.attr("width") - margin.left - margin.right,
        height = svg.attr("height") - margin.top - margin.bottom;

    let cellWidth = width / columns,
        cellHeight = height / rows;

    let chartMargins = { left: 5, top: 5 };

    for (var r = 0; r < rows; r++) {
        for (var c = 0; c < columns; c++) {
            let sub = chartGroup
                .append("g")
                .attr("transform", "translate(" + (cellWidth * c) + "," + (cellHeight * r) + ")")
                .attr("width", cellWidth - chartMargins.left)
                .attr("height", cellHeight - chartMargins.top);

            drawFn(sub, r, c);

            // TODO axes for the grid - something like this would be nice, but drawGrid doesn't have the data 
            // if (r == 0) { //?
            //     sub.append("g")
            //         .attr("class", "axis")
            //         .attr("transform", `translate(${margin.left},${height + margin.top})`)
            //         .call(xAxis);
            // }

            // if (c == columns - 1) { //?
            //     sub.append("g")
            //         .attr("class", "axis")
            //         .attr("transform", `translate(${margin.left},${margin.top})`)
            //         .call(yAxis)
            // }
        }
    }
}
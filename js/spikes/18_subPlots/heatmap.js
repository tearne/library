// Placeholder stuff for off-diagonal cells
export function heatmap(svg) {
    let colour = '#' + (0x1000000 + (Math.random()) * 0xffffff).toString(16).substr(1, 6)

    let width = svg.attr("width")
    let height = svg.attr("height")

    let g = svg.append("g")

    g
        .append("rect")
        .attr("width", width)
        .attr("height", height)
        .attr("fill", colour)

    let data = [4, 7, 2, 5, 1, 7, 8, 1, 1, 4]

    let y = d3.scaleLinear()
        .domain([0, 8])
        .range([height, 0]);

    g
        .selectAll("rect")
        .data(data)
        .enter()
        .append("rect")
        .attr("y", d => y(d))
        .attr("x", (d, i) => i * 10)
        .attr("width", 10)
        .attr("height", d => height - y(d))
        .attr("fill", "black");
}
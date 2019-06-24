export function drawAxis(svg, keys, margin) {
    let width = svg.attr("width") - margin.left - margin.right,
        height = svg.attr("height") - margin.top - margin.bottom;

    var x = d3.scaleBand()
        .domain(keys)
        .range([0, width]);

    var y = d3.scaleBand()
        .domain(keys)
        .range([0, height]);

    var xAxis = d3.axisBottom(x)
    var yAxis = d3.axisLeft(y)

    svg.append("g")
        .attr("class", "axis")
        .attr("transform", "translate(" + margin.left + "," + (height + margin.top) + ")")
        .call(xAxis);

    svg.append("g")
        .attr("class", "axis")
        .attr("transform", "translate(" + margin.left + "," + margin.top + ")")
        .call(yAxis)
        .selectAll("text")
        .attrs({
            y: -15,
            x: 0,
            dy: "0.35em"
        })
        .attr("transform", "rotate(-90)")
        .attr("text-anchor", "middle");
}
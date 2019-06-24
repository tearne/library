export function histogram(svg, data) {
    const width = svg.attr("width"),
        height = svg.attr("height")

    let xDomain = d3.extent(data);

    let bins = d3.histogram().domain(xDomain).thresholds(20)(data);

    let x = d3.scaleLinear()
        .domain(xDomain)
        .range([0, width]);

    let y = d3.scaleLinear()
        .domain([0, d3.max(bins, b => b.length)])
        .range([height, 0]);

    svg.append("g")
        .selectAll("rect")
        .data(bins)
        .enter()
        .append("rect")
        .attr("x", d => x(d.x0) + 1)
        .attr("y", d => y(d.length))
        .attr("width", d => x(d.x1) - x(d.x0) - 1)
        .attr("height", d => y(0) - y(d.length))
        .attr("class", "histbars")
}
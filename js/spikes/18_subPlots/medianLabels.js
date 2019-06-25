let formatValue = d3.format(",.4f");

export function medianLabels(svg, data) {
    const width = svg.attr("width");
    const median = d3.median(data);
    const xDomain = d3.extent(data);
    let x = d3.scaleLinear()
        .domain(xDomain)
        .range([0, width]);

    svg.append("text")
        .attrs({
            x: x(0),
            y: 1
        })
        .text(formatValue(median))
        .attr("class", "graphlabels")
}
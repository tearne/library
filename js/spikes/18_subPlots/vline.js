let formatValue = d3.format(",.4f");

export function medianLine(svg, data) {
    const width = svg.attr("width");
    const height = svg.attr("height");
    const median = d3.median(data);

    const xDomain = d3.extent(data);

    let x = d3.scaleLinear()
        .domain(xDomain)
        .range([0, width]);

    svg.append("g")
        .append("line")
        .attrs({
            x1: x(median),
            y1: height,
            x2: x(median),
            y2: 0
        })
        .attr("class", "vertical-line");

    svg.append("text")
        .attrs({
            x: x(0),
            y: 1
        })
        .text(formatValue(median))
        .attr("class","graphlabels")
}
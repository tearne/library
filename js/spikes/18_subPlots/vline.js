export function vline(svg, data) {
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
}

export function hline(svg, data) {
    const width = svg.attr("width");
    const height = svg.attr("height");
    const median = d3.median(data);

    const yDomain = d3.extent(data);

    let y = d3.scaleLinear()
        .domain(yDomain)
        .range([height, 0]);

    svg.append("g")
        .append("line")
        .attrs({
            x1: 0,
            y1: y(median),
            x2: width,
            y2: y(median)
        })
        .attr("class", "vertical-line");
}
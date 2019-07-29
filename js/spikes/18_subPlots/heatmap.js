export function heatmap(svg, xData, yData) {
    let width = svg.attr("width")
    let height = svg.attr("height")

    let data = d3.zip(xData, yData);

    // setup x-scale 
    var x = d3.scaleLinear()
        .domain(d3.extent(xData))
        .range([0, width]);

    // setup y-scale
    var y = d3.scaleLinear()
        .domain(d3.extent(yData))
        .range([height, 0]);

    // setup colour scale 
    var color = d3.scalePow()
        .exponent(0.33)
        .domain([0.025, 1])
        .range(["midnightblue", "red"]);

    // compute the density data
    var densityData = d3.contourDensity()
        .x(function (d) { return x(d[0]); })
        .y(function (d) { return y(d[1]); })
        .size([width, height])
        .bandwidth(0.5)
        .cellSize(8)
        (data);

    // plot contours
    svg.selectAll("path")
        .data(densityData)
        .enter().append("path")
        .attr("d", d3.geoPath())
        .attr("fill", function (d) { return color(d.value); });
}
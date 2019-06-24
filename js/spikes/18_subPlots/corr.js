let formatValue = d3.format(",.4f");

export function corr(svg, data1, data2) {
    let corr = getPearsonsCorrelation(data1, data2);

    const width = svg.attr("width");
    const height = svg.attr("height");

    svg.append("text")
        .attrs({
            x: width / 2,
            y: height / 2
        })
        .text(formatValue(corr))
        .attr("class", "correlations")
}

// Calculate the pearsons correlation for two passed in arrays
// Credit to Steve Gardner for this function
// http://stevegardner.net/2012/06/11/javascript-code-to-calculate-the-pearson-correlation-coefficient/
function getPearsonsCorrelation(x, y) {
    var shortestArrayLength = 0;
    if (x.length == y.length) {
        shortestArrayLength = x.length;
    }
    else if (x.length > y.length) {
        shortestArrayLength = y.length;
        console.error('x has more items in it, the last ' + (x.length - shortestArrayLength) + ' item(s) will be ignored');
    }
    else {
        shortestArrayLength = x.length;
        console.error('y has more items in it, the last ' + (y.length - shortestArrayLength) + ' item(s) will be ignored');
    }

    var xy = [];
    var x2 = [];
    var y2 = [];

    for (var i = 0; i < shortestArrayLength; i++) {
        xy.push(x[i] * y[i]);
        x2.push(x[i] * x[i]);
        y2.push(y[i] * y[i]);
    }

    var sum_x = 0;
    var sum_y = 0;
    var sum_xy = 0;
    var sum_x2 = 0;
    var sum_y2 = 0;

    for (var i = 0; i < shortestArrayLength; i++) {
        sum_x += x[i];
        sum_y += y[i];
        sum_xy += xy[i];
        sum_x2 += x2[i];
        sum_y2 += y2[i];
    }

    var step1 = (shortestArrayLength * sum_xy) - (sum_x * sum_y);
    var step2 = (shortestArrayLength * sum_x2) - (sum_x * sum_x);
    var step3 = (shortestArrayLength * sum_y2) - (sum_y * sum_y);
    var step4 = Math.sqrt(step2 * step3);
    var answer = step1 / step4;

    if (isNaN(answer)) return 0;
    return answer;
}
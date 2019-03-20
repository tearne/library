import * as ss from "https://unpkg.com/simple-statistics@7.0.2/index.js?module"

export function density(svg, data, adjust = 1){
  const size = svg.node().getBoundingClientRect()

  const numBins = 200
  const xDomain = d3.extent(data);
  let x = d3.scaleLinear()
    .domain(xDomain)
    .range([0, size.width]);
  const binWidth = (xDomain[1] - xDomain[0]) / numBins

  let bins = (() => {
    let scale = d3.scaleLinear().domain([0, numBins]).range(xDomain);
    let thresholds = d3.range(numBins + 1).map(scale);
    return d3.histogram().domain(xDomain).thresholds(thresholds)(data);
  })();

  let kernel = buildArrayKernel(data, binWidth, adjust);

  let points = (() => {
    let n = data.length;

    let result = (() => {
      let res = [];
      res.length = bins.length;
      res.fill(0)
      res = res.map(() => ({x:0, y:0}))
      return res;
    })();

    bins.forEach( (bin, idx) => {
      result[idx].x = (bin.x0 + bin.x1)/2

      kernel.forEach((kObj) => {
        let resultIdx = idx + kObj.offset
        if(resultIdx >= 0 && resultIdx < result.length){
          result[resultIdx].y = (result[resultIdx].y || 0) + bin.length * kObj.value / n
        }
      });
    });

    return result;
  })();

  let y = d3.scaleLinear()
    .domain([0, d3.max(points, d => d.y)])
    .range([size.height, 0])

  let line = d3.line()
    .x(d => x(d.x))
    .y(d => y(d.y))
    // .curve(d3.curveBasis)

  svg.append("g")
    .append("path")
    .datum(points)
    .attr("fill", "none")
    .attr("stroke", "blue")
    .attr("stroke-width", 1.5)
    .attr("stroke-linejoin", "round")
    .attr("d",  line);
}

function buildArrayKernel(data, interval, adjust = 1) {
  // https://en.wikipedia.org/wiki/Kernel_density_estimation
  // https://stat.ethz.ch/R-manual/R-devel/library/MASS/html/bandwidth.nrd.html

  let h = (() => {
    let sd = ss.standardDeviation(data);
    let iqr = ss.interquartileRange(data);
    if (typeof iqr === 'number') {
        sd = Math.min(sd, iqr / 1.34);
    }
    return 1.06 * sd * Math.pow(data.length, -0.2);
  })();

  function getGaussian(h) {
    let SQRT_2PI = Math.sqrt(2 * Math.PI);
    function k(x) { return Math.exp(- 0.5 * x * x) * SQRT_2PI; };
    let f = function(x) { return (k(x / h) / h); };
    f.width = 3 * h;
    f.h = h;
    return f;
  }

  let kernel = getGaussian(h * adjust);

  let lastY;
  let values = [];
  let x = 0;

  do {
    lastY = kernel(x);
    values.push(lastY);
    x = x + interval;
  } while (lastY > 1e-100 && values.length < 1000)

  let reversed = values.slice().reverse();
  values.shift();
  let length = values.length;
  let array = reversed.concat(values).map((v,i) => ({
    offset: i - length,
    value: v
  }));

  array.kernel = kernel;

  return array;
}
'use strict';

const fs = require('fs');
const sp = require('./pkg/seraplot.js');

class Chart {
  constructor(html) {
    this.html = html;
  }

  call(name, argsJson) {
    this.html = sp.applyChartMethod(this.html, name, argsJson === undefined ? '{}' : argsJson);
    return this;
  }

  toSvg() {
    const start = this.html.indexOf('<svg');
    const end = this.html.lastIndexOf('</svg>');
    return start === -1 || end === -1 ? '' : this.html.slice(start, end + 6);
  }

  save(path) {
    fs.writeFileSync(path, this.html);
    return this;
  }

  toString() {
    return this.html;
  }
}

function call(name, json) {
  return new Chart(sp.call(name, json));
}

module.exports = Object.assign({}, sp, { Chart, call });

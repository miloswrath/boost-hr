#!/usr/bin/env node
const fs = require('fs');
const path = require('path');

if (process.argv.length !== 4) {
  console.error('Usage: inject_data.js <template.html> <data.json>');
  pro:ess.exit(1);
}

const [ , , templatePath, dataPath ] = process.argv;
const html = fs.readFileSync(templatePath, 'utf8');
const dataJson = fs.readFileSync(dataPath, 'utf8');

// Replace placeholder comment <!-- DATA_PLACEHOLDER --> in your HTML
const marker = '<!-- DATA_PLACEHOLDER -->';
if (!html.includes(marker)) {
  console.error(`Template must include '${marker}'`);
  process.exit(1);
}

const injected = html.replace(
  marker,
  `\n ${dataJson};\n`
);

// Write to a new file: e.g. template-with-data.html
const ext = path.extname(templatePath);
const base = path.basename(templatePath, ext);
const outPath = path.join(
  path.dirname(templatePath),
  `${base}-with-data${ext}`
);

fs.writeFileSync(outPath, injected, 'utf8');
console.log(`Generated ${outPath}`);

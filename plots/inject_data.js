// inject_data.js
// Usage: node inject_data.js template.html data.json

const fs = require("fs");
const path = require("path");

if (process.argv.length !== 4) {
  console.error("Usage: node inject_data.js <template.html> <data.json>");
  process.exit(1);
}

const [ , , htmlPath, jsonPath ] = process.argv;

try {
  const html = fs.readFileSync(htmlPath, "utf8");
  const json = fs.readFileSync(jsonPath, "utf8");

  const placeholder = "<!-- DATA_PLACEHOLDER -->";
  if (!html.includes(placeholder)) {
    console.error(`Missing placeholder: ${placeholder}`);
    process.exit(1);
  }

  const outputHtml = html.replace(
    placeholder,
    `\n<script>window.PLOT_DATA = ${json};</script>\n`
  );

  const outputPath = path.join(
    path.dirname(htmlPath),
    `${path.basename(htmlPath, path.extname(htmlPath))}-with-data.html`
  );

  fs.writeFileSync(outputPath, outputHtml);
  console.log(`✅ Injected data into: ${outputPath}`);
} catch (err) {
  console.error("Error during injection:", err);
  process.exit(1);
}

import { execFileSync } from "node:child_process";
import { readFile, writeFile } from "node:fs/promises";

const checkOnly = process.argv.includes("--check");
const markdownFiles = execFileSync("git", ["ls-files", "-z", "--", "*.md"], { encoding: "utf8" })
  .split("\0")
  .filter(Boolean);
const changed = [];

function formatComment(match, body) {
  if (!body.includes("\n")) return match;

  const sourceLines = body
    .split("\n")
    .map((line) => line.trim())
    .filter(Boolean);
  const output = [];
  let paragraph = "";
  let listItem = "";

  const flushParagraph = () => {
    if (paragraph) output.push(paragraph);
    paragraph = "";
  };
  const flushListItem = () => {
    if (listItem) output.push(listItem);
    listItem = "";
  };

  for (const line of sourceLines) {
    if (/^(?:[-*+] |\d+[.)] )/.test(line)) {
      flushParagraph();
      flushListItem();
      listItem = line;
      continue;
    }

    if (listItem) {
      if (/^[a-z0-9_`([{"']/.test(line)) {
        listItem += ` ${line}`;
        continue;
      }
      flushListItem();
    }

    paragraph = paragraph ? `${paragraph} ${line}` : line;
  }

  flushListItem();
  flushParagraph();
  return `<!-- ${output.join("\n")} -->`;
}

for (const file of markdownFiles) {
  const source = await readFile(file, "utf8");
  const formatted = source.replace(/<!--([\s\S]*?)-->/g, formatComment);

  if (formatted === source) continue;
  changed.push(file);
  if (!checkOnly) await writeFile(file, formatted);
}

if (checkOnly && changed.length > 0) {
  console.error("Markdown comments contain line-length wrapping:");
  for (const file of changed) console.error(`- ${file}`);
  process.exit(1);
}

if (checkOnly) {
  console.log("Verified Markdown comments have no line-length wrapping.");
} else {
  for (const file of changed) console.log(file);
}

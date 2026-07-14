import { readdir, readFile } from "node:fs/promises";
import { join, relative } from "node:path";

const siteRoot = new URL("../site/", import.meta.url);
const allowedHeroBreak = /docslime-hero-wordmark-light"[^>]*><br>\s*<img[^>]*docslime-hero-wordmark-dark/g;
const failures = [];

async function checkDirectory(directory) {
  for (const entry of await readdir(directory, { withFileTypes: true })) {
    const path = join(directory, entry.name);

    if (entry.isDirectory()) {
      await checkDirectory(path);
      continue;
    }

    if (!entry.name.endsWith(".html")) continue;

    const html = await readFile(path, "utf8");
    const unexpectedBreaks = html.replace(allowedHeroBreak, "").match(/<br>/g)?.length ?? 0;

    if (unexpectedBreaks > 0) {
      failures.push(`${relative(siteRoot.pathname, path)}: ${unexpectedBreaks} unexpected <br> element(s)`);
    }
  }
}

await checkDirectory(siteRoot.pathname);

if (failures.length > 0) {
  console.error("Rendered documentation contains source-wrapped prose:");
  for (const failure of failures) console.error(`- ${failure}`);
  process.exit(1);
}

console.log("Verified rendered documentation has no unintended line breaks.");

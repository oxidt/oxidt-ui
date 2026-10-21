// Screenshot a running app's pages in light and dark, for `diff.ts`.
//
//   bun shoot.ts <base-url> <out-dir> <pages-file> [--cookie name=value]... [--storage auth.json]
//
// The pages file lists one page per line, `<name> <path>`; `#` starts a comment.
// `--storage` takes a Playwright storage state for signed-in pages; record one
// with `bunx playwright codegen --save-storage=auth.json <base-url>`.

import { chromium } from "playwright";
import { mkdirSync, readFileSync } from "node:fs";

const [base, out, pagesFile, ...rest] = process.argv.slice(2);
if (!base || !out || !pagesFile) {
    console.error("usage: bun shoot.ts <base-url> <out-dir> <pages-file> [--cookie name=value]... [--storage auth.json]");
    process.exit(2);
}

const cookies: { name: string; value: string; url: string }[] = [];
let storage: string | undefined;
for (let i = 0; i < rest.length; i++) {
    if (rest[i] === "--cookie") {
        const [name, ...value] = rest[++i].split("=");
        cookies.push({ name, value: value.join("="), url: base });
    } else if (rest[i] === "--storage") {
        storage = rest[++i];
    }
}

const pages = readFileSync(pagesFile, "utf8")
    .split("\n")
    .map((line) => line.replace(/#.*/, "").trim())
    .filter(Boolean)
    .map((line) => line.split(/\s+/) as [string, string]);

mkdirSync(out, { recursive: true });
const browser = await chromium.launch();
const context = await browser.newContext({
    viewport: { width: 1440, height: 900 },
    reducedMotion: "reduce",
    storageState: storage,
});
await context.addCookies(cookies);
const page = await context.newPage();

let failed = 0;
for (const [name, path] of pages) {
    for (const theme of ["light", "dark"]) {
        try {
            await page.emulateMedia({ colorScheme: theme as "light" | "dark" });
            const response = await page.goto(base + path, { waitUntil: "networkidle" });
            // An error page or a redirect (say, to a login page) would look
            // like a valid screenshot and hide the problem.
            if (!response || response.status() >= 400) {
                throw new Error(`HTTP ${response?.status() ?? "no response"}`);
            }
            if (new URL(page.url()).pathname !== new URL(base + path).pathname) {
                throw new Error(`redirected to ${new URL(page.url()).pathname}`);
            }
            // Pin the theme as a theme script would (attribute plus colour
            // scheme, which `light-dark()` reads) and freeze motion, so only
            // the CSS differs between runs.
            await page.evaluate((t) => {
                document.documentElement.dataset.theme = t;
                document.documentElement.style.colorScheme = t;
                const style = document.createElement("style");
                // `.loading` is an SVG mask with SMIL animation, which
                // `animation: none` cannot stop, so hide it instead.
                style.textContent =
                    "*,*::before,*::after{animation:none!important;transition:none!important;caret-color:transparent!important}.loading{visibility:hidden!important}";
                document.head.appendChild(style);
                for (const svg of document.querySelectorAll("svg")) svg.pauseAnimations();
            }, theme);
            await page.waitForTimeout(150);
            await page.screenshot({ path: `${out}/${name}-${theme}.png`, fullPage: true });
        } catch (e) {
            failed++;
            console.error(`${name} (${theme}): ${(e as Error).message.split("\n")[0]}`);
        }
    }
}
await browser.close();
console.log(`${pages.length * 2 - failed} screenshots in ${out}`);
process.exit(failed ? 1 : 0);

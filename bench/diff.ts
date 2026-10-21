// Compare two `shoot.ts` runs pixel by pixel, worst page first.
//
//   bun diff.ts <before-dir> <after-dir> [--threshold N]
//
// A pixel counts as changed when a channel differs by more than N (default
// 24), which ignores anti-aliasing noise; pass a low N such as 2 to see
// deliberate colour tweaks. For each page it prints the changed share, any
// height change and the first changed row bands (y ranges), and writes a mask
// of the changed pixels to `<after-dir>-diff/`.

import { PNG } from "pngjs";
import { mkdirSync, readdirSync, readFileSync, writeFileSync } from "node:fs";

const [before, after, ...rest] = process.argv.slice(2);
if (!before || !after) {
    console.error("usage: bun diff.ts <before-dir> <after-dir> [--threshold N]");
    process.exit(2);
}
const t = rest.indexOf("--threshold");
const THRESHOLD = t >= 0 ? Number(rest[t + 1]) : 24;
if (!Number.isInteger(THRESHOLD) || THRESHOLD < 0 || THRESHOLD > 255) {
    console.error(`--threshold must be an integer from 0 to 255, got ${rest[t + 1]}`);
    process.exit(2);
}
const out = `${after.replace(/\/$/, "")}-diff`;
mkdirSync(out, { recursive: true });

const rows: { pct: number; name: string; note: string }[] = [];
const pngs = (dir: string) => readdirSync(dir).filter((f) => f.endsWith(".png"));
const inBefore = new Set(pngs(before));
const inAfter = new Set(pngs(after));
for (const name of [...new Set([...inBefore, ...inAfter])].sort()) {
    if (!inAfter.has(name)) {
        rows.push({ pct: 100, name, note: "missing in after" });
        continue;
    }
    if (!inBefore.has(name)) {
        rows.push({ pct: 100, name, note: "new in after" });
        continue;
    }
    const b = PNG.sync.read(readFileSync(`${after}/${name}`));
    const a = PNG.sync.read(readFileSync(`${before}/${name}`));
    const width = Math.max(a.width, b.width);
    const height = Math.max(a.height, b.height);
    const mask = new PNG({ width, height });
    const bands: [number, number][] = [];
    let changed = 0;
    for (let y = 0; y < height; y++) {
        let rowChanged = 0;
        for (let x = 0; x < width; x++) {
            // Pixels outside the shorter image count as changed.
            const inA = x < a.width && y < a.height;
            const inB = x < b.width && y < b.height;
            let diff = inA !== inB;
            if (inA && inB) {
                const i = (y * a.width + x) * 4;
                const j = (y * b.width + x) * 4;
                diff =
                    Math.abs(a.data[i] - b.data[j]) > THRESHOLD ||
                    Math.abs(a.data[i + 1] - b.data[j + 1]) > THRESHOLD ||
                    Math.abs(a.data[i + 2] - b.data[j + 2]) > THRESHOLD;
            }
            const k = (y * width + x) * 4;
            mask.data[k + 3] = 255;
            if (diff) {
                changed++;
                rowChanged++;
                mask.data[k] = 255;
            }
        }
        if (rowChanged > 3) {
            const last = bands.at(-1);
            if (last && y - last[1] <= 6) last[1] = y;
            else bands.push([y, y]);
        }
    }
    if (changed) writeFileSync(`${out}/${name}`, PNG.sync.write(mask));
    const notes = [];
    if (a.height !== b.height) notes.push(`height ${a.height}->${b.height}`);
    if (bands.length) notes.push(`rows ${bands.slice(0, 6).map(([s, e]) => `${s}-${e}`).join(" ")}`);
    rows.push({ pct: (100 * changed) / (width * height), name, note: notes.join("  ") });
}

rows.sort((x, y) => y.pct - x.pct);
for (const { pct, name, note } of rows) {
    console.log(`${pct.toFixed(2).padStart(6)}%  ${name}  ${note}`);
}
const pcts = rows.map((r) => r.pct).sort((x, y) => x - y);
const median = pcts.length ? pcts[Math.floor(pcts.length / 2)] : 0;
console.log(`\n${rows.length} pages, max ${(pcts.at(-1) ?? 0).toFixed(2)}%, median ${median.toFixed(2)}%; masks in ${out}`);

// Browser regression checks for the theme: each test pins a bug that was
// fixed once, measured in Chromium against fixture/checks.html.
//
//   TAILWIND=/path/to/tailwindcss bun test
//
// TAILWIND is a standalone Tailwind CLI (default `tailwindcss` on PATH). The
// suite compiles the fixture with it, so CI can run it per Tailwind version.

import { afterAll, beforeAll, expect, test } from "bun:test";
import { chromium, type Browser, type Page } from "playwright";
import { mkdtempSync, readFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

const fixture = join(import.meta.dir, "fixture");
const tmp = mkdtempSync(join(tmpdir(), "oxidt-ui-checks-"));
const W = 1280;
const H = 900;

let server: ReturnType<typeof Bun.serve>;
let browser: Browser;
let page: Page;
let base: string;

beforeAll(async () => {
    const tailwind = process.env.TAILWIND ?? "tailwindcss";
    const out = join(tmp, "checks.css");
    const build = Bun.spawnSync([tailwind, "-i", join(fixture, "checks.input.css"), "-o", out]);
    if (build.exitCode !== 0) throw new Error(`tailwind failed: ${build.stderr}`);
    const css = readFileSync(out, "utf8");
    // The same sheet with anchor positioning switched off, for the fallback.
    const noAnchor = css.replaceAll("@supports (position-area: block-end)", "@supports (not-a-feature: 1)");
    const html = readFileSync(join(fixture, "checks.html"), "utf8");

    server = Bun.serve({
        port: 0,
        fetch(req) {
            const path = new URL(req.url).pathname;
            if (path === "/checks.css") return new Response(css, { headers: { "content-type": "text/css" } });
            if (path === "/no-anchor.css") return new Response(noAnchor, { headers: { "content-type": "text/css" } });
            if (path === "/checks.html") return new Response(html, { headers: { "content-type": "text/html" } });
            if (path === "/no-anchor.html")
                return new Response(html.replace("/checks.css", "/no-anchor.css"), { headers: { "content-type": "text/html" } });
            return new Response("not found", { status: 404 });
        },
    });
    base = `http://127.0.0.1:${server.port}`;
    browser = await chromium.launch();
    page = await browser.newPage({ viewport: { width: W, height: H } });
    await page.goto(`${base}/checks.html`);
});

afterAll(async () => {
    await browser?.close();
    server?.stop();
    rmSync(tmp, { recursive: true, force: true });
});

const dialogOpen = (id: string) => page.evaluate((id) => (document.getElementById(id) as HTMLDialogElement).open, id);

/** WCAG contrast of an element's text against the first opaque background behind it. */
const contrast = (id: string) =>
    page.evaluate((id) => {
        const toRgb = (c: string) => {
            const ctx = document.createElement("canvas").getContext("2d")!;
            ctx.fillStyle = c;
            ctx.fillRect(0, 0, 1, 1);
            return [...ctx.getImageData(0, 0, 1, 1).data].slice(0, 3);
        };
        const lum = (c: number[]) => {
            const [r, g, b] = c.map((v) => ((v /= 255) <= 0.03928 ? v / 12.92 : ((v + 0.055) / 1.055) ** 2.4));
            return 0.2126 * r + 0.7152 * g + 0.0722 * b;
        };
        const el = document.getElementById(id)!;
        let bg = "rgba(0, 0, 0, 0)";
        for (let e: Element | null = el; e && bg === "rgba(0, 0, 0, 0)"; e = e.parentElement) {
            bg = getComputedStyle(e).backgroundColor;
        }
        const f = lum(toRgb(getComputedStyle(el).color));
        const b = lum(toRgb(bg));
        return (Math.max(f, b) + 0.05) / (Math.min(f, b) + 0.05);
    }, id);

// Runs first: the fixture opens #m2 on load, which blocks clicks until closed.
test("modal(open: true) opens a real modal that Escape closes", async () => {
    expect(await page.evaluate(() => document.getElementById("m2")!.matches(":modal"))).toBe(true);
    await page.keyboard.press("Escape");
    expect(await dialogOpen("m2")).toBe(false);
});

test("modal_close inside a form closes without submitting or validating", async () => {
    await page.click("#open-m");
    expect(await dialogOpen("m")).toBe(true);
    await page.click("#cancel");
    expect(await dialogOpen("m")).toBe(false);
    expect(page.url()).toBe(`${base}/checks.html`);
});

test("modal buttons fall back to onclick without invoker commands", async () => {
    await page.evaluate(() => {
        delete (HTMLButtonElement.prototype as { commandForElement?: unknown }).commandForElement;
        // Without `command` the native invoker does nothing, so only the fallback can act.
        for (const id of ["open-m", "cancel"]) document.getElementById(id)!.removeAttribute("command");
    });
    expect(await page.evaluate(() => "commandForElement" in HTMLButtonElement.prototype)).toBe(false);
    await page.click("#open-m");
    expect(await page.evaluate(() => document.getElementById("m")!.matches(":modal"))).toBe(true);
    await page.click("#cancel");
    expect(await dialogOpen("m")).toBe(false);
    await page.reload();
    await page.keyboard.press("Escape");
});

test("neutral outline, soft and ghost stay readable in dark mode", async () => {
    for (const id of ["neutral-outline", "neutral-soft", "neutral-soft-btn", "neutral-ghost", "neutral-outline-badge"]) {
        expect(await contrast(id), id).toBeGreaterThanOrEqual(4.5);
    }
});

test("a light section inside a dark page gets light tokens and no dark: styles", async () => {
    const s = await page.evaluate(() => {
        const get = (id: string) => getComputedStyle(document.getElementById(id)!);
        return {
            bg: get("nested-light").backgroundColor,
            scheme: get("nested-light").colorScheme,
            text: get("nested-text").color,
            darkText: get("dark-text").color,
        };
    });
    expect(s).toEqual({ bg: "rgb(255, 255, 255)", scheme: "light", text: "rgb(10, 10, 10)", darkText: s.darkText });
    // Outside the light section `dark:text-error` still applies.
    expect(s.darkText).not.toBe("rgb(237, 237, 237)");
});

test("a popover is centred where anchor positioning is missing", async () => {
    const p = await browser.newPage({ viewport: { width: W, height: H } });
    await p.goto(`${base}/no-anchor.html`);
    await p.keyboard.press("Escape");
    await p.evaluate(() => (document.getElementById("pop") as HTMLElement).showPopover());
    await p.waitForTimeout(300);
    const c = await p.evaluate(() => {
        const r = document.getElementById("pop")!.getBoundingClientRect();
        return { x: r.x + r.width / 2, y: r.y + r.height / 2 };
    });
    await p.close();
    expect(Math.abs(c.x - W / 2)).toBeLessThan(3);
    expect(Math.abs(c.y - H / 2)).toBeLessThan(3);
});

test("popover-top and popover-end combine", async () => {
    await page.click("#pop-trigger");
    await page.waitForTimeout(300);
    const r = await page.evaluate(() => {
        const t = document.getElementById("pop-trigger")!.getBoundingClientRect();
        const p = document.getElementById("pop")!.getBoundingClientRect();
        return { above: p.bottom <= t.top + 1, endGap: Math.abs(p.right - t.right) };
    });
    await page.keyboard.press("Escape");
    expect(r.above).toBe(true);
    expect(r.endGap).toBeLessThan(2);
});

test("join-vertical lg:join-horizontal is horizontal at lg", async () => {
    const items = await page.evaluate(() =>
        [...document.querySelectorAll("#join > .join-item")].map((e) => {
            const s = getComputedStyle(e);
            return {
                mt: s.marginTop,
                ms: s.marginInlineStart,
                radii: [s.borderStartStartRadius, s.borderStartEndRadius, s.borderEndEndRadius, s.borderEndStartRadius],
            };
        }),
    );
    expect(items.map((i) => i.mt)).toEqual(["0px", "0px", "0px"]);
    expect(items.map((i) => i.ms)).toEqual(["0px", "-1px", "-1px"]);
    expect(items[0].radii[0]).not.toBe("0px");
    expect(items[0].radii[1]).toBe("0px");
    expect(items[1].radii).toEqual(["0px", "0px", "0px", "0px"]);
    expect(items[2].radii[3]).toBe("0px");
    expect(items[2].radii[2]).not.toBe("0px");
});

test("a checkbox-driven collapse shows keyboard focus", async () => {
    await page.focus("#collapse-input");
    await page.keyboard.press("Shift+Tab");
    await page.keyboard.press("Tab");
    const s = await page.evaluate(() => ({
        focused: document.activeElement?.id,
        outline: getComputedStyle(document.getElementById("collapse")!).outlineStyle,
    }));
    expect(s).toEqual({ focused: "collapse-input", outline: "solid" });
});

test("a checked checkbox shows its tick", async () => {
    const tick = await page.evaluate(() => {
        const b = getComputedStyle(document.getElementById("checked")!, "::before");
        return { width: parseFloat(b.width), opacity: b.opacity };
    });
    expect(tick.width).toBeGreaterThan(0);
    expect(tick.opacity).toBe("1");
});

test("an indeterminate checkbox shows a dash", async () => {
    await page.evaluate(() => {
        (document.getElementById("indet") as HTMLInputElement).indeterminate = true;
    });
    await page.waitForTimeout(300);
    const mark = await page.evaluate(() => {
        const b = getComputedStyle(document.getElementById("indet")!, "::before");
        return { opacity: b.opacity, dash: b.maskImage.includes("M5 12h14"), width: parseFloat(b.width) };
    });
    expect(mark).toEqual({ opacity: "1", dash: true, width: mark.width });
    expect(mark.width).toBeGreaterThan(0);
});

test("expired toasts collapse instead of holding their place", async () => {
    await page.waitForTimeout(600);
    const t = await page.evaluate(() => {
        const bottom = document.getElementById("toaster")!.getBoundingClientRect().bottom;
        const item = (id: string) => {
            const e = document.getElementById(id)!;
            return { h: e.getBoundingClientRect().height, bottom: e.getBoundingClientRect().bottom, mb: getComputedStyle(e).marginBottom };
        };
        return { bottom, error: item("t-error"), a: item("t-1"), b: item("t-2") };
    });
    expect([t.a.h, t.b.h, t.a.mb, t.b.mb]).toEqual([0, 0, "0px", "0px"]);
    // The error toast sits at the stack's edge, 0.25rem in, with no hole below.
    expect(Math.round(t.bottom - t.error.bottom)).toBe(4);
});

test("radar_chart shows only the picked comparison", async () => {
    const shown = () =>
        page.evaluate(() =>
            [1, 2, 3].map((k) => getComputedStyle(document.querySelector(`#radar-wrap polygon.radar-only-${k}`)!).visibility),
        );
    await page.waitForTimeout(400);
    expect(await shown()).toEqual(["visible", "hidden", "hidden"]);
    await page.click("#radar-wrap input.radar-pick-3");
    await page.waitForTimeout(400);
    expect(await shown()).toEqual(["hidden", "hidden", "visible"]);
});

test("radar_chart shows an axis tooltip on keyboard focus", async () => {
    await page.focus("#radar-wrap input.radar-pick-3");
    await page.keyboard.press("Tab");
    await page.waitForTimeout(300);
    const s = await page.evaluate(() => {
        const axis = document.activeElement as SVGGElement;
        return {
            isAxis: axis.classList.contains("radar-axis"),
            label: axis.getAttribute("aria-label"),
            tip: getComputedStyle(axis.querySelector(".radar-focus")!).opacity,
            row: getComputedStyle(axis.querySelector(".radar-tip .radar-only-3")!).visibility,
        };
    });
    expect(s).toEqual({ isAxis: true, label: "Talent pool: Lisbon 70, Berlin 88, Toronto 75, Austin 65", tip: "1", row: "visible" });
});

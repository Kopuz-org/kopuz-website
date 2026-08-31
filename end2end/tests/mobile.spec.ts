import type { Page } from "@playwright/test";
import { devices, expect, test } from "@playwright/test";

// The spec carries its own phone so it means the same thing under the desktop
// project as it does under mobile-chromium.
test.use({ ...devices["Pixel 5"] });

test.skip(
  ({ browserName }) => browserName === "firefox",
  "Firefox has no mobile emulation",
);

const routes = [
  "/",
  "/features",
  "/download",
  "/guides",
  "/guides/spotify",
  "/support",
  "/privacy",
  "/nope",
] as const;

// Every measurement below is in CSS pixels, so the page has to be done moving:
// the release stamp lands after hydration and the web fonts re-flow the text.
async function ready(page: Page, route: string) {
  // The dev server drops the odd connection when the whole suite arrives at
  // once, and one empty response is not a defect worth failing a run over.
  const response = await page.goto(route).catch(() => page.goto(route));
  // /nope answers 404 with the not-found page, which is a rendered page too.
  expect(response?.status()).toBeLessThan(500);
  await page.locator("html[data-hydrated]").waitFor();
  await page.waitForLoadState("networkidle");
  await page.evaluate(() => document.fonts.ready.then(() => undefined));
}

test.describe("nav menu", () => {
  test("the menu button opens and closes the panel", async ({ page }) => {
    await ready(page, "/");

    const panel = page.locator("#nav-menu");
    const open = page.getByRole("button", { name: "Open menu" });
    await expect(open).toHaveAttribute("aria-expanded", "false");
    await expect(panel).toBeHidden();

    await open.tap();
    const close = page.getByRole("button", { name: "Close menu" });
    await expect(close).toHaveAttribute("aria-expanded", "true");
    await expect(panel).toBeVisible();

    await close.tap();
    await expect(open).toHaveAttribute("aria-expanded", "false");
    await expect(panel).toBeHidden();
  });

  test("a link in the panel navigates and closes it", async ({ page }) => {
    await ready(page, "/");

    await page.getByRole("button", { name: "Open menu" }).tap();
    await page.locator("#nav-menu").getByRole("link", { name: "Download" }).tap();

    await expect(page).toHaveURL(/\/download$/);
    await expect(page.locator("#nav-menu")).toBeHidden();
    await expect(page.getByRole("button", { name: "Open menu" })).toHaveAttribute(
      "aria-expanded",
      "false",
    );
  });
});

test.describe("the player bar covers nothing", () => {
  for (const route of routes) {
    test(`${route} ends above the player bar`, async ({ page }) => {
      await ready(page, route);

      const covered = await page.evaluate(async () => {
        const settle = () =>
          new Promise((resolve) =>
            requestAnimationFrame(() => requestAnimationFrame(resolve)),
          );
        // Twice: the first scroll can reveal lazy images that grow the page.
        for (let pass = 0; pass < 2; pass += 1) {
          window.scrollTo({
            top: document.documentElement.scrollHeight,
            behavior: "instant",
          });
          await settle();
        }

        const player = document.querySelector(".player");
        const limit = player
          ? player.getBoundingClientRect().top
          : window.innerHeight;
        const hidden: string[] = [];
        document
          .querySelectorAll("main *, footer *, .shelf *")
          .forEach((element) => {
            if (element.children.length || element.closest(".player")) return;
            const text = (element.textContent ?? "").trim();
            if (!text && element.tagName !== "IMG") return;
            const box = element.getBoundingClientRect();
            if (!box.width || !box.height) return;
            const style = getComputedStyle(element);
            if (style.visibility === "hidden" || style.opacity === "0") return;
            if (box.bottom > limit + 0.5 && box.top < window.innerHeight) {
              hidden.push(`${element.tagName.toLowerCase()} "${text.slice(0, 24)}"`);
            }
          });
        return hidden;
      });

      expect(covered, `${route} keeps content under the player bar`).toEqual([]);
    });
  }
});

test.describe("tap targets", () => {
  // 40x40, or 24px with 8px of clearance from the next target.
  const measure = () => {
    const selector =
      '#nav-menu a, button, [role="button"], select, summary, .shelf-copy, .copy-btn, .snip-copy';
    const nodes = Array.from(
      document.querySelectorAll<HTMLElement>(selector),
    ).filter((element) => {
      if (element.closest('[aria-hidden="true"]')) return false;
      if (element.tabIndex < 0) return false;
      const box = element.getBoundingClientRect();
      if (!box.width || !box.height) return false;
      const style = getComputedStyle(element);
      return style.visibility !== "hidden" && style.display !== "none";
    });
    const boxes = nodes.map((element) => element.getBoundingClientRect());
    const apart = (a: DOMRect, b: DOMRect) =>
      Math.hypot(
        Math.max(a.left - b.right, b.left - a.right, 0),
        Math.max(a.top - b.bottom, b.top - a.bottom, 0),
      );

    return nodes.reduce<string[]>((small, element, index) => {
      const box = boxes[index];
      if (box.width >= 40 && box.height >= 40) return small;
      const clearance = boxes.reduce((least, other, position) => {
        if (position === index) return least;
        if (
          element.contains(nodes[position]) ||
          nodes[position].contains(element)
        ) {
          return least;
        }
        return Math.min(least, apart(box, other));
      }, Number.POSITIVE_INFINITY);
      if (box.width >= 24 && box.height >= 24 && clearance >= 8) return small;
      small.push(
        `${element.tagName.toLowerCase()}.${element.className} ${Math.round(
          box.width,
        )}x${Math.round(box.height)} clearance=${Math.round(clearance)}`,
      );
      return small;
    }, []);
  };

  for (const route of routes) {
    test(`${route} keeps every control thumb sized`, async ({ page }) => {
      await ready(page, route);
      await page.getByRole("button", { name: "Open menu" }).tap();
      await page.evaluate(() => {
        document.querySelectorAll("details").forEach((details) => {
          details.open = true;
        });
      });

      expect(await page.evaluate(measure)).toEqual([]);
    });
  }
});

test.describe("page behaviour", () => {
  test.use({ permissions: ["clipboard-read", "clipboard-write"] });

  test("the download copy button reports the copy", async ({ page }) => {
    await ready(page, "/download");

    const copy = page.locator(".snip-copy").first();
    await copy.scrollIntoViewIfNeeded();
    await expect(copy).toHaveText("Copy");

    await copy.tap();
    await expect(copy).toHaveText("Copied");
  });

  test("the features jump list scrolls sideways under the nav", async ({
    page,
  }) => {
    await ready(page, "/features");

    const list = page.locator(".feat-jump ul");
    const width = await list.evaluate((element) => ({
      scroll: element.scrollWidth,
      client: element.clientWidth,
    }));
    expect(width.scroll).toBeGreaterThan(width.client);

    await list.evaluate((element) =>
      element.scrollBy({ left: 200, behavior: "instant" }),
    );
    expect(await list.evaluate((element) => element.scrollLeft)).toBeGreaterThan(
      0,
    );

    // The rail stays under the nav while the page scrolls past it.
    await page.evaluate(() =>
      window.scrollTo({ top: 2000, behavior: "instant" }),
    );
    const rail = await page.locator(".feat-jump").boundingBox();
    expect(rail?.y).toBeLessThan(120);
    await expect(page.locator(".feat-jump a.is-current")).toHaveCount(1);
  });

  test("the theming card steps through every theme", async ({ page }) => {
    await ready(page, "/");

    const labels = await page.evaluate(() =>
      Array.from(
        document.querySelectorAll("[data-theme-stage] .theme-layer"),
      ).map((layer) => layer.getAttribute("data-theme-label") ?? ""),
    );
    expect(labels.length).toBe(4);

    const shown = await page.evaluate(async () => {
      const card = document.querySelector<HTMLElement>(".moment-themes");
      if (!card) return [];
      const caption = document.querySelector("[data-theme-caption]");
      const settle = () =>
        new Promise((resolve) =>
          requestAnimationFrame(() => requestAnimationFrame(resolve)),
        );

      const seen: string[] = [];
      const end = card.offsetTop + card.offsetHeight;
      for (let y = card.offsetTop - 400; y < end; y += 60) {
        window.scrollTo({ top: y, behavior: "instant" });
        await settle();
        await settle();
        const text = (caption?.textContent ?? "").trim();
        if (text && seen[seen.length - 1] !== text) seen.push(text);
      }
      return seen;
    });

    expect(shown).toEqual(labels);
  });
});

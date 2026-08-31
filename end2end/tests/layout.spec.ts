import { expect, test } from "@playwright/test";

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

const widths = [360, 390, 768, 1024, 1440, 1920] as const;

test.describe("layout holds at every width", () => {
  for (const width of widths) {
    test(`no horizontal overflow at ${width}px`, async ({ page }) => {
      await page.setViewportSize({ width, height: 900 });

      for (const route of routes) {
        await page.goto(route);
        await page.waitForLoadState("networkidle");

        const overflow = await page.evaluate(() => {
          const limit = window.innerWidth + 1;
          const wide: string[] = [];
          document.querySelectorAll("body *").forEach((el) => {
            const box = el.getBoundingClientRect();
            if (box.width > 0 && box.right > limit) {
              wide.push(`${el.tagName.toLowerCase()}.${el.className}`);
            }
          });
          return {
            scrollWidth: document.documentElement.scrollWidth,
            innerWidth: window.innerWidth,
            wide: wide.slice(0, 5),
          };
        });

        expect(
          overflow.scrollWidth,
          `${route} at ${width}px overflows: ${overflow.wide.join(", ")}`,
        ).toBeLessThanOrEqual(overflow.innerWidth);
      }
    });
  }
});

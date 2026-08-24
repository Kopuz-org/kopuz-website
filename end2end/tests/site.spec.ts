import { expect, test } from "@playwright/test";

const routes = [
  {
    path: "/",
    title: "Kopuz Music Player",
    heading: "Local files.",
    canonical: "https://kopuz.moe",
    activeNav: "Kopuz",
  },
  {
    path: "/features",
    title: "Features | Kopuz",
    heading: "Features",
    canonical: "https://kopuz.moe/features",
    activeNav: "Features",
  },
  {
    path: "/download",
    title: "Download | Kopuz",
    heading: "Download Kopuz",
    canonical: "https://kopuz.moe/download",
    activeNav: "Download",
  },
  {
    path: "/guides",
    title: "Guides | Kopuz",
    heading: "Guides",
    canonical: "https://kopuz.moe/guides",
    activeNav: "Guides",
  },
  {
    path: "/support",
    title: "Support | Kopuz",
    heading: "Support Kopuz",
    canonical: "https://kopuz.moe/support",
    activeNav: "Support Kopuz",
  },
  {
    path: "/privacy",
    title: "Privacy Policy | Kopuz",
    heading: "Privacy Policy",
    canonical: "https://kopuz.moe/privacy",
  },
] as const;

test.describe("site routes", () => {
  for (const route of routes) {
    test(`${route.path} has its own content and metadata`, async ({ page }) => {
      const response = await page.goto(route.path);

      expect(response?.ok()).toBe(true);
      await expect(page).toHaveTitle(route.title);
      await expect(page.locator("main")).toHaveCount(1);
      await expect(page.locator("main h1")).toContainText(route.heading);
      await expect(page.locator('link[rel="canonical"]')).toHaveAttribute(
        "href",
        route.canonical,
      );
      await expect(page.locator(".nav-tab").first()).toBeVisible();

      if ("activeNav" in route) {
        await expect(
          page.getByRole("link", { name: route.activeNav, exact: true }).first(),
        ).toHaveAttribute("aria-current", "page");
      }
    });
  }
});

test.describe("site themes", () => {
  test("follows the OS scheme and persists an explicit choice", async ({
    context,
    page,
  }) => {
    await page.emulateMedia({ colorScheme: "dark" });
    await page.goto("/features");
    await expect(page.locator(".site")).toHaveClass(/\bdark\b/);

    await page.emulateMedia({ colorScheme: "light" });
    await page.reload();
    await expect(page.locator(".site")).toHaveClass(/\blight\b/);

    await page.getByRole("button", { name: "Use dark theme" }).click();
    await expect(page.locator(".site")).toHaveClass(/\bdark\b/);
    expect(
      (await context.cookies()).find((cookie) => cookie.name === "kopuz-theme")
        ?.value,
    ).toBe("dark");

    await page.goto("/download");
    await expect(page.locator(".site")).toHaveClass(/\bdark\b/);
  });

  test("paints a saved dark theme before hydration", async ({
    context,
    page,
  }) => {
    const baseURL =
      (test.info().project.use.baseURL as string | undefined) ??
      "http://127.0.0.1:3000";
    await context.addCookies([
      { name: "kopuz-theme", value: "dark", url: new URL(baseURL).origin },
    ]);
    await page.route("**/pkg/*.js*", (route) => route.abort());

    const response = await page.goto("/features", {
      waitUntil: "domcontentloaded",
    });

    expect(response?.ok()).toBe(true);
    await expect(page.locator("html")).toHaveAttribute("data-theme", "dark");
    expect(
      await page.locator("body").evaluate((element) => {
        return getComputedStyle(element).backgroundColor;
      }),
    ).toBe("rgb(23, 20, 15)");
    expect(
      await page.locator(".site").evaluate((element) => {
        return getComputedStyle(element).color;
      }),
    ).toBe("rgb(241, 236, 226)");
  });

  test("migrates the previous theme cookie", async ({ context, page }) => {
    await page.emulateMedia({ colorScheme: "dark" });
    await page.goto("/features");
    await page.evaluate(() => {
      document.cookie = "kopuz-moe=1; Path=/; SameSite=Strict";
    });
    await page.reload();

    await expect
      .poll(async () => {
        const cookies = await context.cookies();
        return cookies.find((cookie) => cookie.name === "kopuz-theme")?.value;
      })
      .toBe("light");
    await expect(page.locator(".site")).toHaveClass(/\blight\b/);
    expect(
      (await context.cookies()).find((cookie) => cookie.name === "kopuz-moe"),
    ).toBeUndefined();
  });

  test("moe overrides the saved scheme and follows internal navigation", async ({
    context,
    page,
  }) => {
    await page.goto("/features");
    await page.getByRole("button", { name: "Use dark theme" }).click();
    await page.goto("/features?moe");

    const site = page.locator(".site");
    await expect(site).toHaveClass(/\bmoe\b/);
    await expect(site).not.toHaveClass(/\bdark\b/);
    await expect(
      page.getByRole("button", { name: "Leave moe mode" }),
    ).toBeVisible();

    const download = page.locator('a.nav-tab[href="/download?moe"]');
    await expect(download).toBeVisible();
    await download.click();
    await expect(page).toHaveURL(/\/download\?moe$/);
    await expect(site).toHaveClass(/\bmoe\b/);
    expect(
      (await context.cookies()).find((cookie) => cookie.name === "kopuz-theme")
        ?.value,
    ).toBe("dark");

    await page.goto("/download?lang=en&moe");
    await page.getByRole("button", { name: "Leave moe mode" }).click();
    await expect(page).toHaveURL(/\/download\?lang=en$/);
    await expect(page.locator(".site")).toHaveClass(/\bdark\b/);
    expect(
      (await context.cookies()).find((cookie) => cookie.name === "kopuz-theme")
        ?.value,
    ).toBe("dark");
  });
});

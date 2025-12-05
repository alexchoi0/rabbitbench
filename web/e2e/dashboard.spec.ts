import { test, expect } from "@playwright/test";

test.describe("Projects Page", () => {
  test("displays projects heading", async ({ page }) => {
    await page.goto("/projects");

    await expect(page.getByRole("heading", { name: "Projects" })).toBeVisible();
  });

  test("displays page description", async ({ page }) => {
    await page.goto("/projects");

    await expect(page.getByText("Manage your benchmark projects")).toBeVisible();
  });

  test("has new project button", async ({ page }) => {
    await page.goto("/projects");

    await expect(page.getByRole("link", { name: "New Project" })).toBeVisible();
  });

  test("shows projects list or empty state", async ({ page }) => {
    await page.goto("/projects");

    const emptyState = page.getByText("No projects yet");
    const projectsList = page.locator("[class*='grid']").filter({ has: page.locator("a[href^='/projects/']") });

    const hasEmptyState = await emptyState.isVisible().catch(() => false);
    const hasProjects = await projectsList.isVisible().catch(() => false);

    expect(hasEmptyState || hasProjects).toBe(true);
  });

  test("new project button navigates to create page", async ({ page }) => {
    await page.goto("/projects");

    await page.getByRole("link", { name: "New Project" }).click();
    await expect(page).toHaveURL("/projects/new");
  });
});

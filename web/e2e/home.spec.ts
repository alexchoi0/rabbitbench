import { test, expect } from "@playwright/test";

test.describe("Home Page", () => {
  test("displays hero section", async ({ page }) => {
    await page.goto("/");

    await expect(page.getByRole("heading", { level: 1 })).toContainText(
      "Track Your Benchmark Performance"
    );
    await expect(page.getByText("Continuous benchmarking for Rust projects")).toBeVisible();
  });

  test("displays navigation", async ({ page }) => {
    await page.goto("/");

    await expect(page.getByRole("link", { name: "RabbitBench" })).toBeVisible();
    await expect(page.getByRole("link", { name: "Get Started" })).toBeVisible();
  });

  test("displays feature cards", async ({ page }) => {
    await page.goto("/");

    await expect(page.getByRole("heading", { name: "Collect" })).toBeVisible();
    await expect(page.getByRole("heading", { name: "Visualize" })).toBeVisible();
    await expect(page.getByRole("heading", { name: "Alert" })).toBeVisible();
  });

  test("displays CLI example", async ({ page }) => {
    await page.goto("/");

    await expect(page.getByText("Easy to Use CLI")).toBeVisible();
    await expect(page.getByText("cargo install rabbitbench")).toBeVisible();
  });

  test("get started link navigates to login", async ({ page }) => {
    await page.goto("/");

    await page.getByRole("link", { name: "Get Started" }).click();
    await expect(page).toHaveURL("/login");
  });
});

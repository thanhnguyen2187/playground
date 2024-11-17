import { test, expect } from "@playwright/test";

test("has title", async ({ page }) => {
  await page.goto("/");

  const taskInput = page.getByPlaceholder('Enter your task here');
  await expect(taskInput).toBeVisible();
});

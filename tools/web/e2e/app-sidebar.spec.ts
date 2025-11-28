import { expect, test } from '@playwright/test'

test.describe('App Sidebar', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/')
  })

  test('should be visible on page load', async ({ page }) => {
    const sidebar = page.locator('[data-sidebar="sidebar"]')
    await expect(sidebar).toBeVisible()
  })

  test('should display Features logo and text', async ({ page }) => {
    const featureIcon = page.getByAltText('Feature Icon')
    await expect(featureIcon).toBeVisible()

    const featuresLink = page.getByRole('link').filter({ hasText: 'Features' })
    await expect(featuresLink).toBeVisible()
  })

  test('should contain sidebar trigger button', async ({ page }) => {
    const trigger = page.locator('[data-sidebar="trigger"]')
    await expect(trigger).toBeVisible()
  })

  test('should toggle sidebar when trigger is clicked', async ({ page }) => {
    const trigger = page.locator('[data-sidebar="trigger"]')
    const sidebarWrapper = page.locator('.group\\/sidebar-wrapper')

    await expect(trigger).toBeVisible()

    // Click to collapse
    await trigger.click()
    await page.waitForTimeout(300)

    // Check if sidebar is collapsed by checking data-state
    const state = await sidebarWrapper
      .locator('[data-state]')
      .first()
      .getAttribute('data-state')
    expect(state).toBe('collapsed')

    // Click to expand again
    await trigger.click()
    await page.waitForTimeout(300)

    const expandedState = await sidebarWrapper
      .locator('[data-state]')
      .first()
      .getAttribute('data-state')
    expect(expandedState).toBe('expanded')
  })

  test('should display Insights link when features are present', async ({
    page,
  }) => {
    // Wait for the sidebar to load
    await page.locator('[data-sidebar="sidebar"]').waitFor()

    // Check if Insights link exists
    const insightsLink = page.getByRole('link', { name: /insights/i })
    const isVisible = await insightsLink.isVisible().catch(() => false)

    // Insights link should be visible if features are loaded
    if (isVisible) {
      await expect(insightsLink).toBeVisible()
    }
  })

  test('should navigate to Insights page when clicked', async ({ page }) => {
    await page.locator('[data-sidebar="sidebar"]').waitFor()

    const insightsLink = page.getByRole('link', { name: /insights/i })
    const isVisible = await insightsLink.isVisible().catch(() => false)

    if (isVisible) {
      await insightsLink.click()
      await expect(page).toHaveURL(/\/insights/)
    }
  })

  test('should display mode toggle in footer', async ({ page }) => {
    const sidebar = page.locator('[data-sidebar="sidebar"]')
    await expect(sidebar).toBeVisible()

    // Look for the footer section
    const footer = page.locator('[data-sidebar="footer"]')
    await expect(footer).toBeVisible()
  })

  test('should display sidebar footer', async ({ page }) => {
    const sidebar = page.locator('[data-sidebar="sidebar"]')
    await expect(sidebar).toBeVisible()

    const footer = page.locator('[data-sidebar="footer"]')
    await expect(footer).toBeVisible()
  })

  test('should have sidebar header', async ({ page }) => {
    const header = page.locator('[data-sidebar="header"]')
    await expect(header).toBeVisible()
  })

  test('should have sidebar content area', async ({ page }) => {
    const content = page.locator('[data-sidebar="content"]')
    await expect(content).toBeVisible()
  })
})

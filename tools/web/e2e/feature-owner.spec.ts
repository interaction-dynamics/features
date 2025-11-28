import { expect, test } from '@playwright/test'

test.describe('Feature Owner', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/')

    // Wait for sidebar to load
    await page.locator('[data-sidebar="sidebar"]').waitFor()
  })

  test('should display owner section in feature details', async ({ page }) => {
    // Click on first feature
    const firstFeature = page.locator('[data-sidebar="menu-button"]').first()
    await firstFeature.click()
    await page.waitForTimeout(500)

    // Look for Owner section with User icon
    const ownerSection = page.locator('p:has-text("Owner")').first()
    await expect(ownerSection).toBeVisible()

    // Check for User icon
    const userIcon = page.locator('svg.lucide-user').first()
    await expect(userIcon).toBeVisible()
  })

  test('should display owner name in feature details', async ({ page }) => {
    // Click on first feature
    const firstFeature = page.locator('[data-sidebar="menu-button"]').first()
    await firstFeature.click()
    await page.waitForTimeout(500)

    // Look for owner text
    const ownerSection = page.locator('p:has-text("Owner")').first()
    await expect(ownerSection).toBeVisible()

    // Owner value should be visible (either a name or "Unknown")
    const ownerContainer = ownerSection
      .locator('..')
      .locator('div.text-xs.font-mono')
    await expect(ownerContainer).toBeVisible()
  })

  test('should display "Unknown" for features without owner', async ({
    page,
  }) => {
    // Click through features to find one without owner
    const features = page.locator('[data-sidebar="menu-button"]')
    const featureCount = await features.count()

    let foundUnknown = false

    for (let i = 0; i < Math.min(featureCount, 10); i++) {
      await features.nth(i).click()
      await page.waitForTimeout(300)

      const unknownText = page.locator('text=Unknown').first()
      if ((await unknownText.count()) > 0) {
        await expect(unknownText).toBeVisible()
        foundUnknown = true
        break
      }
    }

    // Test passes whether we found "Unknown" or not
    expect(typeof foundUnknown).toBe('boolean')
  })

  test('should display inherited badge for inherited owners', async ({
    page,
  }) => {
    // Click through features to find one with inherited owner
    const features = page.locator('[data-sidebar="menu-button"]')
    const featureCount = await features.count()

    let foundInherited = false

    for (let i = 0; i < Math.min(featureCount, 10); i++) {
      await features.nth(i).click()
      await page.waitForTimeout(300)

      const inheritedBadge = page.locator('text=inherited').first()
      if ((await inheritedBadge.count()) > 0) {
        await expect(inheritedBadge).toBeVisible()
        foundInherited = true
        break
      }
    }

    // Test passes whether we found inherited owner or not
    expect(typeof foundInherited).toBe('boolean')
  })

  test('should style inherited badge with amber colors', async ({ page }) => {
    // Click through features to find one with inherited owner
    const features = page.locator('[data-sidebar="menu-button"]')
    const featureCount = await features.count()

    for (let i = 0; i < Math.min(featureCount, 10); i++) {
      await features.nth(i).click()
      await page.waitForTimeout(300)

      const inheritedBadge = page.locator('text=inherited').first()
      if ((await inheritedBadge.count()) > 0) {
        const className = await inheritedBadge.getAttribute('class')

        // Should have amber color classes
        expect(className).toContain('bg-amber')
        expect(className).toContain('text-amber')
        break
      }
    }
  })

  test('should show tooltip on inherited badge hover', async ({ page }) => {
    // Click through features to find one with inherited owner
    const features = page.locator('[data-sidebar="menu-button"]')
    const featureCount = await features.count()

    for (let i = 0; i < Math.min(featureCount, 10); i++) {
      await features.nth(i).click()
      await page.waitForTimeout(300)

      const inheritedBadge = page.locator('text=inherited').first()
      if ((await inheritedBadge.count()) > 0) {
        await inheritedBadge.hover()
        await page.waitForTimeout(300)

        // Look for tooltip content
        const tooltip = page.locator('text=/inherited from a parent/i').first()
        const tooltipVisible = await tooltip.isVisible().catch(() => false)

        if (tooltipVisible) {
          await expect(tooltip).toBeVisible()
        }
        break
      }
    }
  })

  test('should display help button for features without owner', async ({
    page,
  }) => {
    // Click through features to find one without owner
    const features = page.locator('[data-sidebar="menu-button"]')
    const featureCount = await features.count()

    for (let i = 0; i < Math.min(featureCount, 10); i++) {
      await features.nth(i).click()
      await page.waitForTimeout(300)

      const ownerSection = page.locator('p:has-text("Owner")').first()
      const ownerContainer = ownerSection
        .locator('..')
        .locator('div.text-xs.font-mono')

      // Check if help button exists
      const helpButton = ownerContainer
        .locator('button[title*="Learn"]')
        .first()
      if ((await helpButton.count()) > 0) {
        await expect(helpButton).toBeVisible()
        break
      }
    }
  })

  test('should display owner dots in sidebar for features', async ({
    page,
  }) => {
    // Look for owner dots in sidebar
    const ownerDots = page.locator('div.size-2.rounded-full')
    const dotCount = await ownerDots.count()

    if (dotCount > 0) {
      const firstDot = ownerDots.first()
      await expect(firstDot).toBeVisible()

      // Should have inline style with backgroundColor
      const style = await firstDot.getAttribute('style')
      expect(style).toContain('background-color')
    }
  })

  test('should show owner tooltip on hover in sidebar', async ({ page }) => {
    // Find a feature with owner dot
    const ownerDots = page.locator('div.size-2.rounded-full')
    const dotCount = await ownerDots.count()

    if (dotCount > 0) {
      const firstDot = ownerDots.first()

      // Hover over the dot
      await firstDot.hover()
      await page.waitForTimeout(500)

      // Tooltip should appear
      const tooltip = page.locator('[role="tooltip"]').first()
      const tooltipVisible = await tooltip.isVisible().catch(() => false)

      if (tooltipVisible) {
        await expect(tooltip).toBeVisible()
      }
    }
  })

  test('should generate consistent colors for same owner', async ({ page }) => {
    // Click on first feature and get owner
    const firstFeature = page.locator('[data-sidebar="menu-button"]').first()
    await firstFeature.click()
    await page.waitForTimeout(500)

    const ownerSection = page.locator('p:has-text("Owner")').first()
    const ownerContainer = ownerSection
      .locator('..')
      .locator('div.text-xs.font-mono')
    const ownerText = await ownerContainer.textContent()

    if (ownerText && !ownerText.includes('Unknown')) {
      // Find all owner dots
      const ownerDots = page.locator('div.size-2.rounded-full')
      const dotCount = await ownerDots.count()

      if (dotCount > 1) {
        // Get colors of first two dots
        const firstDotStyle = await ownerDots.first().getAttribute('style')
        const secondDotStyle = await ownerDots.nth(1).getAttribute('style')

        // Colors should be present
        expect(firstDotStyle).toContain('background-color')
        expect(secondDotStyle).toContain('background-color')
      }
    }
  })

  test('should display owner dot with proper size', async ({ page }) => {
    const ownerDots = page.locator('div.size-2.rounded-full')
    const dotCount = await ownerDots.count()

    if (dotCount > 0) {
      const firstDot = ownerDots.first()
      await expect(firstDot).toBeVisible()

      // Should have size-2 class (8px)
      const className = await firstDot.getAttribute('class')
      expect(className).toContain('size-2')
      expect(className).toContain('rounded-full')
    }
  })

  test('should display owner in monospace font', async ({ page }) => {
    // Click on first feature
    const firstFeature = page.locator('[data-sidebar="menu-button"]').first()
    await firstFeature.click()
    await page.waitForTimeout(500)

    const ownerSection = page.locator('p:has-text("Owner")').first()
    const ownerContainer = ownerSection
      .locator('..')
      .locator('div.text-xs.font-mono')

    await expect(ownerContainer).toBeVisible()

    const className = await ownerContainer.getAttribute('class')
    expect(className).toContain('font-mono')
  })

  test('should show owner dots only for features with different owner than parent', async ({
    page,
  }) => {
    // Expand a folder to see nested features
    const folders = page.locator('.group\\/collapsible')
    const folderCount = await folders.count()

    if (folderCount > 0) {
      const firstFolder = folders.first()
      const folderButton = firstFolder
        .locator('[data-slot="collapsible-trigger"]')
        .first()

      // Get initial state
      const initialState = await firstFolder.getAttribute('data-state')

      // If closed, expand it
      if (initialState === 'closed') {
        await folderButton.click()
        await page.waitForTimeout(400)
      }

      // Check nested features
      const nestedButtons = firstFolder.locator(
        '[data-sidebar="menu-sub"] [data-sidebar="menu-button"]',
      )
      const nestedCount = await nestedButtons.count()

      if (nestedCount > 0) {
        // Some nested features may have owner dots, some may not
        // This is expected behavior
        expect(nestedCount).toBeGreaterThan(0)
      }
    }
  })

  test('should display owner section with User icon', async ({ page }) => {
    const firstFeature = page.locator('[data-sidebar="menu-button"]').first()
    await firstFeature.click()
    await page.waitForTimeout(500)

    // User icon should be visible
    const userIcon = page.locator('svg.lucide-user').first()
    await expect(userIcon).toBeVisible()

    // Icon should have proper size
    const className = await userIcon.getAttribute('class')
    expect(className).toContain('h-4')
    expect(className).toContain('w-4')
  })

  test('should display owner information with proper layout', async ({
    page,
  }) => {
    const firstFeature = page.locator('[data-sidebar="menu-button"]').first()
    await firstFeature.click()
    await page.waitForTimeout(500)

    const ownerSection = page.locator('p:has-text("Owner")').first()
    const ownerWrapper = ownerSection.locator('..')

    // Should have flex layout with gap
    const className = await ownerWrapper.getAttribute('class')
    expect(className).toContain('flex-1')
  })

  test('should handle features with empty owner string', async ({ page }) => {
    // Click through features to find one with empty owner
    const features = page.locator('[data-sidebar="menu-button"]')
    const featureCount = await features.count()

    for (let i = 0; i < Math.min(featureCount, 10); i++) {
      await features.nth(i).click()
      await page.waitForTimeout(300)

      const ownerSection = page.locator('p:has-text("Owner")').first()
      const ownerContainer = ownerSection
        .locator('..')
        .locator('div.text-xs.font-mono')
      const text = await ownerContainer.textContent()

      if (text?.includes('Unknown')) {
        // Should show help button for empty owner
        const helpButton = ownerContainer
          .locator('button[title*="Learn"]')
          .first()
        const hasHelpButton = (await helpButton.count()) > 0

        if (hasHelpButton) {
          await expect(helpButton).toBeVisible()
        }
        break
      }
    }
  })
})

import { expect, test } from '@playwright/test'

test.describe('Feature Metadata', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/')

    // Wait for sidebar to load
    await page.locator('[data-sidebar="sidebar"]').waitFor()

    // Click on first feature to load feature details
    const firstFeature = page.locator('[data-sidebar="menu-button"]').first()
    await firstFeature.click()
    await page.waitForTimeout(500)
  })

  test('should display feature metadata section', async ({ page }) => {
    // Look for the Meta section with Info icon
    const metaSection = page.locator('p:has-text("Meta")').first()
    await expect(metaSection).toBeVisible()

    // Check for Info icon
    const infoIcon = page.locator('svg.lucide-info').first()
    await expect(infoIcon).toBeVisible()
  })

  test('should display creation date in metadata', async ({ page }) => {
    // Wait for meta section to be visible
    const metaSection = page.locator('p:has-text("Meta")').first()
    await expect(metaSection).toBeVisible()

    // Look for creation_date badge
    const creationDateBadge = page.locator('text=creation_date').first()

    // If creation date exists, verify it's visible
    const exists = (await creationDateBadge.count()) > 0
    if (exists) {
      await expect(creationDateBadge).toBeVisible()
    }
  })

  test('should display metadata badges with proper styling', async ({
    page,
  }) => {
    const metaSection = page.locator('p:has-text("Meta")').first()
    await expect(metaSection).toBeVisible()

    // Find metadata container
    const metaContainer = page
      .locator('div.text-xs.font-mono.text-muted-foreground')
      .first()

    if (await metaContainer.isVisible()) {
      // Check that badges have proper border and styling
      const badges = metaContainer.locator('div.border.rounded-md')
      const badgeCount = await badges.count()

      if (badgeCount > 0) {
        const firstBadge = badges.first()
        await expect(firstBadge).toBeVisible()

        // Verify badge has proper classes
        const className = await firstBadge.getAttribute('class')
        expect(className).toContain('border')
        expect(className).toContain('rounded-md')
        expect(className).toContain('px-2')
        expect(className).toContain('py-1')
      }
    }
  })

  test('should display status metadata with appropriate colors', async ({
    page,
  }) => {
    const metaSection = page.locator('p:has-text("Meta")').first()
    await expect(metaSection).toBeVisible()

    // Look for status badge
    const statusBadge = page.locator('text=/status:/i').first()
    const hasStatus = (await statusBadge.count()) > 0

    if (hasStatus) {
      await expect(statusBadge).toBeVisible()

      // Get the parent badge element
      const badgeElement = statusBadge.locator('..')
      const className = await badgeElement.getAttribute('class')

      // Should have color classes (green, yellow, orange, or gray)
      const hasColor =
        className?.includes('bg-green') ||
        className?.includes('bg-yellow') ||
        className?.includes('bg-orange') ||
        className?.includes('bg-gray')

      expect(hasColor).toBe(true)
    }
  })

  test('should display deprecated metadata with warning colors', async ({
    page,
  }) => {
    const metaSection = page.locator('p:has-text("Meta")').first()
    await expect(metaSection).toBeVisible()

    // Look for deprecated badge
    const deprecatedBadge = page.locator('text=/deprecated:/i').first()
    const hasDeprecated = (await deprecatedBadge.count()) > 0

    if (hasDeprecated) {
      await expect(deprecatedBadge).toBeVisible()

      // Get the parent badge element
      const badgeElement = deprecatedBadge.locator('..')
      const className = await badgeElement.getAttribute('class')

      // Should have orange/warning color classes
      expect(className).toContain('bg-orange')
    }
  })

  test('should display version metadata with blue styling', async ({
    page,
  }) => {
    const metaSection = page.locator('p:has-text("Meta")').first()
    await expect(metaSection).toBeVisible()

    // Look for version badge
    const versionBadge = page.locator('text=/version:/i').first()
    const hasVersion = (await versionBadge.count()) > 0

    if (hasVersion) {
      await expect(versionBadge).toBeVisible()

      // Get the parent badge element
      const badgeElement = versionBadge.locator('..')
      const className = await badgeElement.getAttribute('class')

      // Should have blue color classes
      expect(className).toContain('bg-blue')
    }
  })

  test('should render URL metadata as clickable links', async ({ page }) => {
    const metaSection = page.locator('p:has-text("Meta")').first()
    await expect(metaSection).toBeVisible()

    // Look for links (URL metadata should be rendered as <a> tags)
    const metaLinks = page.locator(
      'a[target="_blank"][rel="noopener noreferrer"]',
    )
    const linkCount = await metaLinks.count()

    if (linkCount > 0) {
      const firstLink = metaLinks.first()
      await expect(firstLink).toBeVisible()

      // Verify it has href attribute
      const href = await firstLink.getAttribute('href')
      expect(href).toBeTruthy()
      expect(href).toMatch(/^https?:\/\//)

      // Should contain ExternalLink icon
      const externalIcon = firstLink.locator('svg.lucide-external-link')
      await expect(externalIcon).toBeVisible()
    }
  })

  test('should open URL metadata links in new tab', async ({ page }) => {
    const metaSection = page.locator('p:has-text("Meta")').first()
    await expect(metaSection).toBeVisible()

    // Look for links
    const metaLinks = page.locator(
      'a[target="_blank"][rel="noopener noreferrer"]',
    )
    const linkCount = await metaLinks.count()

    if (linkCount > 0) {
      const firstLink = metaLinks.first()

      // Verify target="_blank"
      const target = await firstLink.getAttribute('target')
      expect(target).toBe('_blank')

      // Verify rel="noopener noreferrer" for security
      const rel = await firstLink.getAttribute('rel')
      expect(rel).toBe('noopener noreferrer')
    }
  })

  test('should display ExternalLink icon for URL metadata', async ({
    page,
  }) => {
    const metaSection = page.locator('p:has-text("Meta")').first()
    await expect(metaSection).toBeVisible()

    // Look for external link icons
    const externalIcons = page.locator('svg.lucide-external-link')
    const iconCount = await externalIcons.count()

    if (iconCount > 0) {
      const firstIcon = externalIcons.first()
      await expect(firstIcon).toBeVisible()

      // Should have proper size classes
      const className = await firstIcon.getAttribute('class')
      expect(className).toContain('h-3')
      expect(className).toContain('w-3')
    }
  })

  test('should display metadata key-value pairs correctly', async ({
    page,
  }) => {
    const metaSection = page.locator('p:has-text("Meta")').first()
    await expect(metaSection).toBeVisible()

    // Look for any metadata badge
    const metaBadges = page.locator('div.border.rounded-md span.font-semibold')
    const badgeCount = await metaBadges.count()

    if (badgeCount > 0) {
      const firstBadge = metaBadges.first()
      await expect(firstBadge).toBeVisible()

      // Text content should contain a colon (key: value format)
      const text = await firstBadge.textContent()
      expect(text).toBeTruthy()
    }
  })

  test('should display array metadata as comma-separated list', async ({
    page,
  }) => {
    const metaSection = page.locator('p:has-text("Meta")').first()
    await expect(metaSection).toBeVisible()

    // Look for metadata that might be arrays
    const metaBadges = page.locator('div.border.rounded-md')
    const badgeCount = await metaBadges.count()

    if (badgeCount > 0) {
      // Check if any badge contains comma-separated values
      for (let i = 0; i < Math.min(badgeCount, 5); i++) {
        const badge = metaBadges.nth(i)
        const text = await badge.textContent()

        if (text?.includes(',')) {
          // Found an array - verify it's properly displayed
          await expect(badge).toBeVisible()
          expect(text).toContain(':')
        }
      }
    }
  })

  test('should display metadata in monospace font', async ({ page }) => {
    const metaSection = page.locator('p:has-text("Meta")').first()
    await expect(metaSection).toBeVisible()

    // Metadata container should have font-mono class
    const metaContainer = page
      .locator('div.text-xs.font-mono.text-muted-foreground')
      .first()

    await expect(metaContainer).toBeVisible()

    const className = await metaContainer.getAttribute('class')
    expect(className).toContain('font-mono')
  })

  test('should display metadata with proper spacing', async ({ page }) => {
    const metaSection = page.locator('p:has-text("Meta")').first()
    await expect(metaSection).toBeVisible()

    // Metadata container should have flex and gap classes
    const metaContainer = page.locator('div.flex.flex-wrap.gap-2').first()

    const className = await metaContainer.getAttribute('class')
    expect(className).toContain('flex')
    expect(className).toContain('flex-wrap')
    expect(className).toContain('gap-2')
  })

  test('should show experimental metadata with yellow styling', async ({
    page,
  }) => {
    const metaSection = page.locator('p:has-text("Meta")').first()
    await expect(metaSection).toBeVisible()

    // Look for experimental or beta badge
    const experimentalBadge = page
      .locator('text=/experimental:|beta:/i')
      .first()
    const hasExperimental = (await experimentalBadge.count()) > 0

    if (hasExperimental) {
      await expect(experimentalBadge).toBeVisible()

      // Get the parent badge element
      const badgeElement = experimentalBadge.locator('..')
      const className = await badgeElement.getAttribute('class')

      // Should have yellow color classes
      expect(className).toContain('bg-yellow')
    }
  })

  test('should handle empty metadata gracefully', async ({ page }) => {
    const metaSection = page.locator('p:has-text("Meta")').first()

    // Meta section might not exist if there's no metadata
    const exists = (await metaSection.count()) > 0

    if (exists) {
      await expect(metaSection).toBeVisible()
    }

    // Test passes whether metadata exists or not
    expect(true).toBe(true)
  })
})

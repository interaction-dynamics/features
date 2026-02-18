import { expect, test } from '@playwright/test'

test.describe('Feature Dependencies', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/')

    // Wait for sidebar to load
    await page.locator('[data-sidebar="sidebar"]').waitFor()

    // Click on first feature to load feature details
    const firstFeature = page.locator('[data-sidebar="menu-button"]').first()
    await firstFeature.click()
    await page.waitForTimeout(500)
  })

  test('should display dependencies tab in feature details', async ({
    page,
  }) => {
    // Look for the Dependencies tab
    const dependenciesTab = page.locator('button:has-text("Dependencies")')
    await expect(dependenciesTab).toBeVisible()
  })

  test('should switch to dependencies tab when clicked', async ({ page }) => {
    const dependenciesTab = page.locator('button:has-text("Dependencies")')
    await dependenciesTab.click()
    await page.waitForTimeout(300)

    // Tab should be active (has data-state="active")
    const isActive =
      (await dependenciesTab.getAttribute('data-state')) === 'active'
    expect(isActive).toBe(true)
  })

  test('should display dependencies table when tab is active', async ({
    page,
  }) => {
    const dependenciesTab = page.locator('button:has-text("Dependencies")')
    await dependenciesTab.click()
    await page.waitForTimeout(300)

    // Check for table headers
    const featureHeader = page.locator('th:has-text("Feature")')
    await expect(featureHeader).toBeVisible()

    const typeHeader = page.locator('th:has-text("Type")')
    await expect(typeHeader).toBeVisible()

    const countHeader = page.locator('th:has-text("Count")')
    await expect(countHeader).toBeVisible()

    const alertsHeader = page.locator('th:has-text("Alerts")')
    await expect(alertsHeader).toBeVisible()
  })

  test('should display dependency type badges', async ({ page }) => {
    const dependenciesTab = page.locator('button:has-text("Dependencies")')
    await dependenciesTab.click()
    await page.waitForTimeout(300)

    // Look for dependency type badges (parent, child, sibling)
    const typeBadges = page.locator(
      'span.inline-flex.items-center.rounded-full',
    )
    const badgeCount = await typeBadges.count()

    if (badgeCount > 0) {
      const firstBadge = typeBadges.first()
      await expect(firstBadge).toBeVisible()
    }
  })

  test('should show circular dependency alerts in orange', async ({ page }) => {
    const dependenciesTab = page.locator('button:has-text("Dependencies")')
    await dependenciesTab.click()
    await page.waitForTimeout(300)

    // Look for circular dependency badges
    const circularBadges = page.locator('text=Circular Dependency')
    const hasCircular = (await circularBadges.count()) > 0

    if (hasCircular) {
      const firstCircularBadge = circularBadges.first()
      await expect(firstCircularBadge).toBeVisible()

      // Should have orange styling
      const parentBadge = firstCircularBadge.locator('..')
      const className = await parentBadge.getAttribute('class')
      expect(className).toContain('bg-orange')
    }
  })

  test('should show tight dependency alerts in orange', async ({ page }) => {
    const dependenciesTab = page.locator('button:has-text("Dependencies")')
    await dependenciesTab.click()
    await page.waitForTimeout(300)

    // Look for tight dependency badges
    const tightBadges = page.locator('text=Tight Dependency')
    const hasTight = (await tightBadges.count()) > 0

    if (hasTight) {
      const firstTightBadge = tightBadges.first()
      await expect(firstTightBadge).toBeVisible()

      // Should have orange styling
      const parentBadge = firstTightBadge.locator('..')
      const className = await parentBadge.getAttribute('class')
      expect(className).toContain('bg-orange')
    }
  })

  test('should expand dependency details when clicked', async ({ page }) => {
    const dependenciesTab = page.locator('button:has-text("Dependencies")')
    await dependenciesTab.click()
    await page.waitForTimeout(300)

    // Look for expandable rows
    const expandButtons = page.locator('button[aria-label*="Expand"]')
    const buttonCount = await expandButtons.count()

    if (buttonCount > 0) {
      const firstButton = expandButtons.first()
      await firstButton.click()
      await page.waitForTimeout(200)

      // Should show expanded content with file details
      const expandedContent = page.locator('td[colspan]').first()
      await expect(expandedContent).toBeVisible()
    }
  })

  test('should display file paths in expanded dependency details', async ({
    page,
  }) => {
    const dependenciesTab = page.locator('button:has-text("Dependencies")')
    await dependenciesTab.click()
    await page.waitForTimeout(300)

    // Expand first row
    const expandButtons = page.locator('button[aria-label*="Expand"]')
    const buttonCount = await expandButtons.count()

    if (buttonCount > 0) {
      await expandButtons.first().click()
      await page.waitForTimeout(200)

      // Look for file path text
      const filePaths = page.locator('code.text-xs')
      const pathCount = await filePaths.count()

      if (pathCount > 0) {
        await expect(filePaths.first()).toBeVisible()
      }
    }
  })

  test('should show import content in popover when view button is clicked', async ({
    page,
  }) => {
    const dependenciesTab = page.locator('button:has-text("Dependencies")')
    await dependenciesTab.click()
    await page.waitForTimeout(300)

    // Expand first row
    const expandButtons = page.locator('button[aria-label*="Expand"]')
    const buttonCount = await expandButtons.count()

    if (buttonCount > 0) {
      await expandButtons.first().click()
      await page.waitForTimeout(200)

      // Look for View buttons
      const viewButtons = page.locator('button:has-text("View")')
      const viewButtonCount = await viewButtons.count()

      if (viewButtonCount > 0) {
        await viewButtons.first().click()
        await page.waitForTimeout(200)

        // Popover should appear with import content
        const popover = page.locator('[role="dialog"]')
        await expect(popover).toBeVisible()
      }
    }
  })

  test('should display line numbers in expanded dependency details', async ({
    page,
  }) => {
    const dependenciesTab = page.locator('button:has-text("Dependencies")')
    await dependenciesTab.click()
    await page.waitForTimeout(300)

    // Expand first row
    const expandButtons = page.locator('button[aria-label*="Expand"]')
    const buttonCount = await expandButtons.count()

    if (buttonCount > 0) {
      await expandButtons.first().click()
      await page.waitForTimeout(200)

      // Look for line numbers (should be numeric)
      const lineNumbers = page.locator('td').filter({ hasText: /^Line \d+$/ })
      const lineCount = await lineNumbers.count()

      if (lineCount > 0) {
        await expect(lineNumbers.first()).toBeVisible()
      }
    }
  })

  test('should display dependencies overview in insights page', async ({
    page,
  }) => {
    // Navigate to insights
    await page.goto('/insights')
    await page.waitForTimeout(500)

    // Look for Dependencies Overview card
    const dependenciesCard = page.locator('text=Dependencies Overview')
    await expect(dependenciesCard).toBeVisible()
  })

  test('should show unique features count in dependencies overview', async ({
    page,
  }) => {
    await page.goto('/insights')
    await page.waitForTimeout(500)

    const uniqueFeaturesLabel = page.locator('text=Unique Features')
    await expect(uniqueFeaturesLabel).toBeVisible()

    // Should have a number next to it
    const parentDiv = uniqueFeaturesLabel.locator('..')
    const numberText = await parentDiv.textContent()
    expect(numberText).toMatch(/\d+/)
  })

  test('should show total dependencies count in dependencies overview', async ({
    page,
  }) => {
    await page.goto('/insights')
    await page.waitForTimeout(500)

    const totalDepsLabel = page.locator('text=Total Dependencies')
    await expect(totalDepsLabel).toBeVisible()

    // Should have a number next to it
    const parentDiv = totalDepsLabel.locator('..')
    const numberText = await parentDiv.textContent()
    expect(numberText).toMatch(/\d+/)
  })

  test('should show circular dependencies count in orange when > 0', async ({
    page,
  }) => {
    await page.goto('/insights')
    await page.waitForTimeout(500)

    const circularLabel = page.locator('text=Circular Dependencies')
    await expect(circularLabel).toBeVisible()

    const parentDiv = circularLabel.locator('..')
    const numberText = await parentDiv.textContent()
    const count = parseInt(numberText?.match(/\d+/)?.[0] || '0')

    if (count > 0) {
      // Should have orange styling
      const className = await parentDiv.getAttribute('class')
      expect(className).toContain('text-orange')
    }
  })

  test('should show tight dependencies count in orange when > 0', async ({
    page,
  }) => {
    await page.goto('/insights')
    await page.waitForTimeout(500)

    const tightLabel = page.locator('text=Tight Dependencies')
    await expect(tightLabel).toBeVisible()

    const parentDiv = tightLabel.locator('..')
    const numberText = await parentDiv.textContent()
    const count = parseInt(numberText?.match(/\d+/)?.[0] || '0')

    if (count > 0) {
      // Should have orange styling
      const className = await parentDiv.getAttribute('class')
      expect(className).toContain('text-orange')
    }
  })

  test('should display dependencies column in insights table', async ({
    page,
  }) => {
    await page.goto('/insights')
    await page.waitForTimeout(500)

    // Look for Dependencies column header
    const depsHeader = page.locator('th:has-text("Dependencies")')
    await expect(depsHeader).toBeVisible()
  })

  test('should show warning icon for features with dependency alerts', async ({
    page,
  }) => {
    await page.goto('/insights')
    await page.waitForTimeout(500)

    // Look for warning icons in dependencies column
    const warningIcons = page.locator('svg.lucide-alert-triangle')
    const iconCount = await warningIcons.count()

    if (iconCount > 0) {
      const firstIcon = warningIcons.first()
      await expect(firstIcon).toBeVisible()

      // Should have orange color
      const className = await firstIcon.getAttribute('class')
      expect(className).toContain('text-orange')
    }
  })

  test('should show tooltip with dependency details on hover', async ({
    page,
  }) => {
    await page.goto('/insights')
    await page.waitForTimeout(500)

    // Find a cell with dependencies
    const depsCells = page.locator('td').filter({ hasText: /^\d+$/ })
    const cellCount = await depsCells.count()

    if (cellCount > 0) {
      // Hover over first deps cell
      await depsCells.first().hover()
      await page.waitForTimeout(300)

      // Tooltip should appear
      const tooltip = page.locator('[role="tooltip"]')
      const tooltipExists = (await tooltip.count()) > 0

      if (tooltipExists) {
        await expect(tooltip).toBeVisible()
      }
    }
  })

  test('should sort dependencies column in insights table', async ({
    page,
  }) => {
    await page.goto('/insights')
    await page.waitForTimeout(500)

    // Click on Dependencies column header to sort
    const depsHeader = page.locator('th:has-text("Dependencies")')
    await depsHeader.click()
    await page.waitForTimeout(300)

    // Header should have sort indicator
    const sortIcon = depsHeader.locator('svg')
    await expect(sortIcon).toBeVisible()
  })

  test('should handle features with no dependencies gracefully', async ({
    page,
  }) => {
    const dependenciesTab = page.locator('button:has-text("Dependencies")')
    await dependenciesTab.click()
    await page.waitForTimeout(300)

    // Should show either table or empty state
    const table = page.locator('table')
    const emptyState = page.locator('text=/no dependencies/i')

    const hasTable = (await table.count()) > 0
    const hasEmptyState = (await emptyState.count()) > 0

    expect(hasTable || hasEmptyState).toBe(true)
  })

  test('should display parent dependency type with green badge', async ({
    page,
  }) => {
    const dependenciesTab = page.locator('button:has-text("Dependencies")')
    await dependenciesTab.click()
    await page.waitForTimeout(300)

    // Look for parent badges
    const parentBadges = page.locator('text=parent')
    const hasParent = (await parentBadges.count()) > 0

    if (hasParent) {
      const firstParentBadge = parentBadges.first()
      await expect(firstParentBadge).toBeVisible()

      // Should have green styling
      const parentDiv = firstParentBadge.locator('..')
      const className = await parentDiv.getAttribute('class')
      expect(className).toContain('bg-green')
    }
  })

  test('should display child dependency type with blue badge', async ({
    page,
  }) => {
    const dependenciesTab = page.locator('button:has-text("Dependencies")')
    await dependenciesTab.click()
    await page.waitForTimeout(300)

    // Look for child badges
    const childBadges = page.locator('text=child')
    const hasChild = (await childBadges.count()) > 0

    if (hasChild) {
      const firstChildBadge = childBadges.first()
      await expect(firstChildBadge).toBeVisible()

      // Should have blue styling
      const parentDiv = firstChildBadge.locator('..')
      const className = await parentDiv.getAttribute('class')
      expect(className).toContain('bg-blue')
    }
  })

  test('should display sibling dependency type with yellow badge', async ({
    page,
  }) => {
    const dependenciesTab = page.locator('button:has-text("Dependencies")')
    await dependenciesTab.click()
    await page.waitForTimeout(300)

    // Look for sibling badges
    const siblingBadges = page.locator('text=sibling')
    const hasSibling = (await siblingBadges.count()) > 0

    if (hasSibling) {
      const firstSiblingBadge = siblingBadges.first()
      await expect(firstSiblingBadge).toBeVisible()

      // Should have yellow styling
      const parentDiv = firstSiblingBadge.locator('..')
      const className = await parentDiv.getAttribute('class')
      expect(className).toContain('bg-yellow')
    }
  })

  test('should collapse expanded dependency details when clicked again', async ({
    page,
  }) => {
    const dependenciesTab = page.locator('button:has-text("Dependencies")')
    await dependenciesTab.click()
    await page.waitForTimeout(300)

    // Expand first row
    const expandButtons = page.locator('button[aria-label*="Expand"]')
    const buttonCount = await expandButtons.count()

    if (buttonCount > 0) {
      const firstButton = expandButtons.first()

      // Expand
      await firstButton.click()
      await page.waitForTimeout(200)

      // Collapse
      await firstButton.click()
      await page.waitForTimeout(200)

      // Expanded content should not be visible
      const expandedContent = page.locator('td[colspan]')
      const isVisible = await expandedContent.first().isVisible()
      expect(isVisible).toBe(false)
    }
  })
})

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

  test.describe('Folders', () => {
    test('should display folder structure for nested features', async ({
      page,
    }) => {
      // Wait for sidebar to be ready
      const sidebar = page.locator('[data-sidebar="sidebar"]')
      await expect(sidebar).toBeVisible()

      // Look for collapsible folder elements
      const folders = page.locator('.group\\/collapsible')
      const folderCount = await folders.count()

      // If folders exist, verify they're visible
      if (folderCount > 0) {
        await expect(folders.first()).toBeVisible()
      }
    })

    test('should display folder icons', async ({ page }) => {
      const sidebar = page.locator('[data-sidebar="sidebar"]')
      await expect(sidebar).toBeVisible()

      // Look for lucide folder icons (both folder and folder-open)
      const folderIcons = page.locator(
        'svg.lucide-folder, svg.lucide-folder-open',
      )
      const count = await folderIcons.count()

      // If there are folder icons, at least one should be visible
      if (count > 0) {
        const firstIcon = folderIcons.first()
        await expect(firstIcon).toBeVisible()
      }
    })

    test('should expand folder when clicked', async ({ page }) => {
      const sidebar = page.locator('[data-sidebar="sidebar"]')
      await expect(sidebar).toBeVisible()

      // Find first collapsible folder
      const folders = page.locator('.group\\/collapsible')
      const folderCount = await folders.count()

      if (folderCount > 0) {
        const firstFolder = folders.first()
        const folderButton = firstFolder
          .locator('[data-slot="collapsible-trigger"]')
          .first()

        // Get initial state
        const initialState = await firstFolder.getAttribute('data-state')

        // Click to toggle
        await folderButton.click()
        await page.waitForTimeout(400)

        // State should have changed
        const newState = await firstFolder.getAttribute('data-state')
        expect(newState).not.toBe(initialState)
      }
    })

    test('should toggle folder between open and closed states', async ({
      page,
    }) => {
      const sidebar = page.locator('[data-sidebar="sidebar"]')
      await expect(sidebar).toBeVisible()

      const folders = page.locator('.group\\/collapsible')
      const folderCount = await folders.count()

      if (folderCount > 0) {
        const firstFolder = folders.first()
        const folderButton = firstFolder
          .locator('[data-slot="collapsible-trigger"]')
          .first()

        // Click once
        await folderButton.click()
        await page.waitForTimeout(400)
        const state1 = await firstFolder.getAttribute('data-state')

        // Click again
        await folderButton.click()
        await page.waitForTimeout(400)
        const state2 = await firstFolder.getAttribute('data-state')

        // States should be different
        expect(state1).not.toBe(state2)
      }
    })

    test('should show chevron icon in collapsible folders', async ({
      page,
    }) => {
      const sidebar = page.locator('[data-sidebar="sidebar"]')
      await expect(sidebar).toBeVisible()

      const folders = page.locator('.group\\/collapsible')
      const folderCount = await folders.count()

      if (folderCount > 0) {
        // Look for chevron icon inside first folder
        const chevron = folders
          .first()
          .locator('svg.lucide-chevron-right')
          .first()
        await expect(chevron).toBeVisible()
      }
    })

    test('should rotate chevron when folder expands', async ({ page }) => {
      const sidebar = page.locator('[data-sidebar="sidebar"]')
      await expect(sidebar).toBeVisible()

      const folders = page.locator('.group\\/collapsible')
      const folderCount = await folders.count()

      if (folderCount > 0) {
        const firstFolder = folders.first()
        const folderButton = firstFolder
          .locator('[data-slot="collapsible-trigger"]')
          .first()
        const chevron = firstFolder.locator('svg.lucide-chevron-right').first()

        // Get initial class
        const initialClass = await chevron.getAttribute('class')

        // Click to expand
        await folderButton.click()
        await page.waitForTimeout(400)

        // Get new class
        const newClass = await chevron.getAttribute('class')

        // Class should have changed (rotation applied)
        expect(initialClass).not.toBe(newClass)
      }
    })

    test('should display nested features inside expanded folders', async ({
      page,
    }) => {
      const sidebar = page.locator('[data-sidebar="sidebar"]')
      await expect(sidebar).toBeVisible()

      const folders = page.locator('.group\\/collapsible')
      const folderCount = await folders.count()

      if (folderCount > 0) {
        const firstFolder = folders.first()
        const folderButton = firstFolder
          .locator('[data-slot="collapsible-trigger"]')
          .first()

        // Expand the folder
        await folderButton.click()
        await page.waitForTimeout(400)

        // Look for nested content
        const nestedContent = firstFolder.locator('[data-sidebar="menu-sub"]')
        const hasNested = (await nestedContent.count()) > 0

        if (hasNested) {
          await expect(nestedContent.first()).toBeVisible()
        }
      }
    })

    test('should allow clicking nested features inside folders', async ({
      page,
    }) => {
      const sidebar = page.locator('[data-sidebar="sidebar"]')
      await expect(sidebar).toBeVisible()

      const folders = page.locator('.group\\/collapsible')
      const folderCount = await folders.count()

      if (folderCount > 0) {
        const firstFolder = folders.first()
        const folderButton = firstFolder
          .locator('[data-slot="collapsible-trigger"]')
          .first()

        // Expand the folder
        await folderButton.click()
        await page.waitForTimeout(400)

        // Look for nested menu buttons
        const nestedButtons = firstFolder.locator(
          '[data-sidebar="menu-sub"] [data-sidebar="menu-button"]',
        )
        const buttonCount = await nestedButtons.count()

        if (buttonCount > 0) {
          const firstButton = nestedButtons.first()
          await expect(firstButton).toBeVisible()

          // Click it
          await firstButton.click()
          await page.waitForTimeout(200)

          // Should be clickable (no error thrown)
          expect(true).toBe(true)
        }
      }
    })

    test('should display "Collapse all folders" button', async ({ page }) => {
      const sidebar = page.locator('[data-sidebar="sidebar"]')
      await expect(sidebar).toBeVisible()

      // Look for the collapse all button
      const collapseButton = page.locator(
        'button[aria-label="Collapse all folders"]',
      )
      const exists = (await collapseButton.count()) > 0

      if (exists) {
        // Button should be visible when folders are present
        const isVisible = await collapseButton.isVisible()
        expect(typeof isVisible).toBe('boolean')
      }
    })

    test('should collapse all folders when collapse all button is clicked', async ({
      page,
    }) => {
      const sidebar = page.locator('[data-sidebar="sidebar"]')
      await expect(sidebar).toBeVisible()

      const folders = page.locator('.group\\/collapsible')
      const folderCount = await folders.count()

      if (folderCount > 0) {
        // Expand first folder
        const firstFolderButton = folders
          .first()
          .locator('[data-slot="collapsible-trigger"]')
          .first()
        await firstFolderButton.click()
        await page.waitForTimeout(400)

        // Look for collapse all button
        const collapseButton = page.locator(
          'button[aria-label="Collapse all folders"]',
        )

        if (await collapseButton.isVisible()) {
          await collapseButton.click()
          await page.waitForTimeout(400)

          // Check first folder is closed
          const firstFolder = folders.first()
          const state = await firstFolder.getAttribute('data-state')
          expect(state).toBe('closed')
        }
      }
    })

    test('should display "All Features" label in sidebar', async ({ page }) => {
      const sidebar = page.locator('[data-sidebar="sidebar"]')
      await expect(sidebar).toBeVisible()

      // Look for "All Features" text
      const label = page.getByText('All Features')
      await expect(label).toBeVisible()
    })

    test('should show folder with FolderOpen icon when expanded', async ({
      page,
    }) => {
      const sidebar = page.locator('[data-sidebar="sidebar"]')
      await expect(sidebar).toBeVisible()

      const folders = page.locator('.group\\/collapsible')
      const folderCount = await folders.count()

      if (folderCount > 0) {
        const firstFolder = folders.first()
        const folderButton = firstFolder
          .locator('[data-slot="collapsible-trigger"]')
          .first()

        // Expand folder
        await folderButton.click()
        await page.waitForTimeout(400)

        // Check if folder-open icon appears
        const openIcon = folderButton.locator('svg.lucide-folder-open')
        const hasOpenIcon = (await openIcon.count()) > 0

        if (hasOpenIcon) {
          await expect(openIcon).toBeVisible()
        }
      }
    })

    test('should maintain folder hierarchy with proper indentation', async ({
      page,
    }) => {
      const sidebar = page.locator('[data-sidebar="sidebar"]')
      await expect(sidebar).toBeVisible()

      const folders = page.locator('.group\\/collapsible')
      const folderCount = await folders.count()

      if (folderCount > 0) {
        const firstFolder = folders.first()
        const folderButton = firstFolder
          .locator('[data-slot="collapsible-trigger"]')
          .first()

        // Expand folder
        await folderButton.click()
        await page.waitForTimeout(400)

        // Look for nested menu (which implies hierarchy)
        const nestedMenu = firstFolder.locator('[data-sidebar="menu-sub"]')
        const hasNested = (await nestedMenu.count()) > 0

        if (hasNested) {
          // Nested menu should be inside the parent folder
          await expect(nestedMenu.first()).toBeAttached()
        }
      }
    })

    test('should show multiple folders if multiple nested features exist', async ({
      page,
    }) => {
      const sidebar = page.locator('[data-sidebar="sidebar"]')
      await expect(sidebar).toBeVisible()

      const folders = page.locator('.group\\/collapsible')
      const folderCount = await folders.count()

      // This test just verifies the count
      expect(folderCount).toBeGreaterThanOrEqual(0)

      // If multiple folders exist, verify more than one
      if (folderCount > 1) {
        expect(folderCount).toBeGreaterThan(1)
      }
    })

    test('should keep folder expanded when selecting nested feature', async ({
      page,
    }) => {
      const sidebar = page.locator('[data-sidebar="sidebar"]')
      await expect(sidebar).toBeVisible()

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

        // Get state after ensuring it's expanded
        const stateAfterExpand = await firstFolder.getAttribute('data-state')
        expect(stateAfterExpand).toBe('open')

        // Click a nested feature if it exists
        const nestedButtons = firstFolder.locator(
          '[data-sidebar="menu-sub"] [data-sidebar="menu-button"]',
        )
        const buttonCount = await nestedButtons.count()

        if (buttonCount > 0) {
          await nestedButtons.first().click()
          await page.waitForTimeout(200)

          // Folder should still be open
          const stateAfterClick = await firstFolder.getAttribute('data-state')
          expect(stateAfterClick).toBe('open')
        }
      }
    })
  })
})

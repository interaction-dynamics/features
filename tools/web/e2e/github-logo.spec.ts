import { expect, test } from '@playwright/test'

test.describe('GitHub Logo', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/')
  })

  test('should be visible in header', async ({ page }) => {
    const githubSvg = page
      .locator('svg')
      .filter({ has: page.locator('title', { hasText: 'Github' }) })
    await expect(githubSvg).toBeVisible()
  })

  test('should have correct SVG structure', async ({ page }) => {
    const githubSvg = page
      .locator('svg')
      .filter({ has: page.locator('title', { hasText: 'Github' }) })
    await expect(githubSvg).toBeVisible()

    // Verify viewBox attribute
    const viewBox = await githubSvg.getAttribute('viewBox')
    expect(viewBox).toBe('0 0 98 96')
  })

  test('should have correct path element with fill rule', async ({ page }) => {
    const githubSvg = page
      .locator('svg')
      .filter({ has: page.locator('title', { hasText: 'Github' }) })
    const path = githubSvg.locator('path')

    await expect(path).toBeVisible()

    // Check for required attributes (HTML uses lowercase attribute names)
    const fillRule = await path.getAttribute('fill-rule')
    const clipRule = await path.getAttribute('clip-rule')

    expect(fillRule).toBe('evenodd')
    expect(clipRule).toBe('evenodd')
  })

  test('should be inside a button with correct styling', async ({ page }) => {
    const githubButton = page
      .getByRole('button')
      .filter({ has: page.locator('svg title', { hasText: 'Github' }) })

    await expect(githubButton).toBeVisible()
    await expect(githubButton).toBeEnabled()

    // Check if button has outline variant styling
    const classList = await githubButton.getAttribute('class')
    expect(classList).toContain('outline')
  })

  test('should open GitHub repository in new tab when clicked', async ({
    page,
    context,
  }) => {
    const githubButton = page
      .getByRole('button')
      .filter({ has: page.locator('svg title', { hasText: 'Github' }) })

    await expect(githubButton).toBeVisible()

    // Listen for new page (tab) to open
    const pagePromise = context.waitForEvent('page')

    await githubButton.click()

    const newPage = await pagePromise
    await newPage.waitForLoadState()

    // Verify the URL of the new tab
    expect(newPage.url()).toContain('github.com/interaction-dynamics/features')

    await newPage.close()
  })

  test('should be positioned in the header next to feedback button', async ({
    page,
  }) => {
    const header = page.locator('header')
    await expect(header).toBeVisible()

    const feedbackButton = page.getByRole('button', { name: /give feedback/i })
    const githubButton = page
      .getByRole('button')
      .filter({ has: page.locator('svg title', { hasText: 'Github' }) })

    await expect(feedbackButton).toBeVisible()
    await expect(githubButton).toBeVisible()

    // Both buttons should be in the same header
    const feedbackInHeader = await feedbackButton.evaluate((el) => {
      return el.closest('header') !== null
    })
    const githubInHeader = await githubButton.evaluate((el) => {
      return el.closest('header') !== null
    })

    expect(feedbackInHeader).toBe(true)
    expect(githubInHeader).toBe(true)
  })

  test('should have proper size styling', async ({ page }) => {
    const githubSvg = page
      .locator('svg')
      .filter({ has: page.locator('title', { hasText: 'Github' }) })

    // Check if SVG has the h-4 w-4 classes (height and width of 1rem)
    const classList = await githubSvg.getAttribute('class')
    expect(classList).toContain('h-4')
    expect(classList).toContain('w-4')
  })

  test('should have title element for accessibility', async ({ page }) => {
    // The title element exists in the DOM for accessibility, even if not visually visible
    const title = page.locator('svg title', { hasText: 'Github' })
    await expect(title).toHaveCount(1)

    // Verify the text content
    const titleText = await title.textContent()
    expect(titleText).toBe('Github')
  })

  test('should have correct fill color', async ({ page }) => {
    const githubSvg = page
      .locator('svg')
      .filter({ has: page.locator('title', { hasText: 'Github' }) })
    const path = githubSvg.locator('path')

    const fill = await path.getAttribute('fill')
    expect(fill).toBe('currentColor')
  })
})

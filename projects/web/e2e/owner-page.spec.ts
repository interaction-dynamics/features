import { expect, test } from '@playwright/test'

// The end-to-end tests run with the experimental pages enabled
// (VITE_EXPERIMENTAL_FEATURES in playwright.config.ts)
test.describe('Owner page', () => {
  test('opens from the ownership table of Overview', async ({ page }) => {
    await page.goto('/#/insights')
    await page.getByRole('tab', { name: 'Ownership' }).click()

    const link = page.getByTestId('owner-link').first()
    const owner = (await link.textContent()) ?? ''
    await link.click()

    await expect(page).toHaveURL(/#\/owners\//)
    await expect(page.getByTestId('owner-title')).toHaveText(owner)
    await expect(page.getByText('Experimental').first()).toBeVisible()
    await expect(page.getByTestId('owner-feature-row').first()).toBeVisible()
  })

  test('decodes owner names with spaces', async ({ page }) => {
    await page.goto(`/#/owners/${encodeURIComponent('John Doe')}`)

    await expect(page.getByTestId('owner-title')).toHaveText('John Doe')
    await expect(page.getByTestId('owner-feature-row').first()).toBeVisible()
  })

  test('shows a not-found state for an unknown owner', async ({ page }) => {
    await page.goto(`/#/owners/${encodeURIComponent('@nobody/team')}`)

    await expect(page.getByTestId('owner-not-found')).toBeVisible()
    await expect(page.getByTestId('owner-title')).toHaveText('@nobody/team')
  })
})

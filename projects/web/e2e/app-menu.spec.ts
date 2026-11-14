import { expect, test } from '@playwright/test'

const REPOSITORY_URL = 'https://github.com/interaction-dynamics/features'

test.describe('App menu', () => {
  test.beforeEach(async ({ page }) => {
    await page.route('**/metadata.json', (route) =>
      route.fulfill({ json: { version: '1.2.3' } }),
    )
    await page.goto('/')
  })

  const trigger = (page: import('@playwright/test').Page) =>
    page.locator('[data-sidebar="footer"]').getByRole('button', {
      name: /Features/,
    })

  test('should show the Features logo and version at the bottom', async ({
    page,
  }) => {
    await expect(trigger(page)).toBeVisible()
    await expect(page.getByAltText('Features logo')).toBeVisible()
    await expect(trigger(page)).toContainText('v1.2.3')
  })

  test('should no longer show feedback and GitHub buttons in the header', async ({
    page,
  }) => {
    const header = page.locator('header')
    await expect(header).toBeVisible()
    await expect(
      header.getByRole('button', { name: /give feedback/i }),
    ).toHaveCount(0)
  })

  test('should open external links in a new tab', async ({ page }) => {
    await trigger(page).click()
    const menu = page.getByRole('menu')
    await expect(menu).toBeVisible()

    const links: [RegExp, string][] = [
      [/^Docs$/, `${REPOSITORY_URL}#readme`],
      [/^Give feedback$/, `${REPOSITORY_URL}/issues/new/choose`],
      [/^GitHub$/, REPOSITORY_URL],
      [/^Full changelog$/, `${REPOSITORY_URL}/releases`],
    ]
    for (const [name, href] of links) {
      const item = menu.getByRole('menuitem', { name })
      await expect(item).toHaveAttribute('href', href)
      await expect(item).toHaveAttribute('target', '_blank')
    }
  })

  test('should list the latest changes under "What\'s new"', async ({
    page,
  }) => {
    await trigger(page).click()
    const menu = page.getByRole('menu')
    await expect(menu.getByText("What's new")).toBeVisible()

    const changes = menu.locator('a[href*="/pull/"]')
    await expect(changes).toHaveCount(3)
  })

  test('should switch the theme from the Theme submenu', async ({ page }) => {
    await trigger(page).click()
    await page.getByRole('menuitem', { name: 'Theme' }).click()
    await page.getByRole('menuitemradio', { name: 'Dark' }).click()
    await expect(page.locator('html')).toHaveClass(/dark/)

    await trigger(page).click()
    await page.getByRole('menuitem', { name: 'Theme' }).click()
    await page.getByRole('menuitemradio', { name: 'Light' }).click()
    await expect(page.locator('html')).toHaveClass(/light/)
  })

  test('should clear the unread changes dot once the menu was opened', async ({
    page,
  }) => {
    const unreadDot = trigger(page).locator('[title="New changes"]')
    await expect(unreadDot).toBeVisible()

    await trigger(page).click()
    await expect(page.getByRole('menu')).toBeVisible()
    await page.keyboard.press('Escape')

    await expect(unreadDot).toHaveCount(0)
    await page.reload()
    await expect(trigger(page)).toBeVisible()
    await expect(unreadDot).toHaveCount(0)
  })
})

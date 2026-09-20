import asyncio
from playwright.async_api import async_playwright

async def run():
    async with async_playwright() as p:
        browser = await p.chromium.launch(headless=True)
        context = await browser.new_context(viewport={'width': 1440, 'height': 900})
        page = await context.new_page()
        await page.goto("http://localhost:8992")
        await page.wait_for_timeout(1000)
        await page.screenshot(path=r"C:\Users\pc\.gemini\antigravity\brain\d994c5fe-4115-48b5-96eb-bc5773d58886\decluttered_two_button_teleprompter.png", full_page=True)
        
        # Also click one of the quick scenario chips (e.g. Guarantee) to show live teleprompter response
        chips = page.locator(".sample-chip")
        if await chips.count() > 1:
            await chips.nth(1).click() # Click 'Guarantee & Refunds'
            await page.wait_for_timeout(800)
            await page.screenshot(path=r"C:\Users\pc\.gemini\antigravity\brain\d994c5fe-4115-48b5-96eb-bc5773d58886\decluttered_teleprompter_active.png", full_page=True)

        await browser.close()

if __name__ == "__main__":
    asyncio.run(run())

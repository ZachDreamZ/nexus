"""Convert logo SVG to PNG using Playwright."""
import asyncio, sys
from pathlib import Path
sys.path.insert(0, r"C:\Users\Vendex\AppData\Local\Programs\Python\Python312\Lib\site-packages")

LOGO_HTML = r"""<!DOCTYPE html>
<html><body style="margin:0;background:#0f172a;display:flex;align-items:center;justify-content:center;height:100vh;">
<svg xmlns="http://www.w3.org/2000/svg" width="200" height="200" viewBox="0 0 200 200">
  <rect width="200" height="200" rx="16" fill="#0f172a"/>
  <text x="100" y="55" text-anchor="middle" fill="#a78bfa" font-family="Arial,sans-serif" font-size="14" font-weight="bold">NEXUS</text>
  <circle cx="100" cy="95" r="12" fill="none" stroke="#38bdf8" stroke-width="2.5"/>
  <circle cx="60" cy="135" r="10" fill="none" stroke="#a78bfa" stroke-width="2"/>
  <circle cx="140" cy="135" r="10" fill="none" stroke="#a78bfa" stroke-width="2"/>
  <circle cx="70" cy="170" r="8" fill="none" stroke="#f59e0b" stroke-width="1.5"/>
  <circle cx="130" cy="170" r="8" fill="none" stroke="#f59e0b" stroke-width="1.5"/>
  <circle cx="100" cy="135" r="6" fill="none" stroke="#10b981" stroke-width="1.5"/>
  <circle cx="45" cy="100" r="6" fill="none" stroke="#f97316" stroke-width="1.5"/>
  <circle cx="155" cy="100" r="6" fill="none" stroke="#f97316" stroke-width="1.5"/>
  <line x1="100" y1="107" x2="100" y2="129" stroke="#38bdf8" stroke-width="1.5" opacity="0.6"/>
  <line x1="100" y1="95" x2="60" y1="125" stroke="#a78bfa" stroke-width="1.5" opacity="0.6"/>
  <line x1="100" y1="95" x2="140" y1="125" stroke="#a78bfa" stroke-width="1.5" opacity="0.6"/>
  <line x1="60" y1="145" x2="70" y1="162" stroke="#f59e0b" stroke-width="1" opacity="0.5"/>
  <line x1="140" y1="145" x2="130" y1="162" stroke="#f59e0b" stroke-width="1" opacity="0.5"/>
  <line x1="100" y1="141" x2="100" y1="160" stroke="#10b981" stroke-width="1.5" opacity="0.6"/>
  <line x1="100" y1="95" x2="45" y1="94" stroke="#f97316" stroke-width="1" opacity="0.4"/>
  <line x1="100" y1="95" x2="155" y1="94" stroke="#f97316" stroke-width="1" opacity="0.4"/>
  <circle cx="100" cy="95" r="4" fill="#38bdf8"/>
  <circle cx="85" cy="185" r="2" fill="#475569"/>
  <circle cx="95" cy="185" r="2" fill="#475569"/>
  <circle cx="105" cy="185" r="2" fill="#475569"/>
</svg>
</body></html>"""

async def main():
    assets_dir = Path(r"D:\workspace\nexus\assets")
    from playwright.async_api import async_playwright
    async with async_playwright() as p:
        browser = await p.chromium.launch(headless=True)
        page = await browser.new_page(viewport={"width": 400, "height": 400})
        await page.set_content(LOGO_HTML)
        await page.wait_for_timeout(1500)
        svg_box = await page.locator("svg").bounding_box()
        if svg_box:
            await page.screenshot(
                path=str(assets_dir / "logo.png"),
                clip={"x": svg_box["x"], "y": svg_box["y"], "width": svg_box["width"], "height": svg_box["height"]},
            )
        else:
            await page.screenshot(path=str(assets_dir / "logo.png"))
        await browser.close()
        print(f"logo.png -> {(assets_dir / 'logo.png').stat().st_size} bytes")

asyncio.run(main())

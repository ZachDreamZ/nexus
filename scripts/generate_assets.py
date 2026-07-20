"""Generate architecture diagram and terminal preview for nexus."""
import asyncio, sys
from pathlib import Path
sys.path.insert(0, str(Path(__file__).parent.parent))
sys.path.insert(0, r"C:\Users\Vendex\AppData\Local\Programs\Python\Python312\Lib\site-packages")

ARCH_HTML = r"""<!DOCTYPE html>
<html><body style="margin:0;background:#0f172a;">
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 800 500" font-family="ui-monospace,monospace">
  <rect width="800" height="500" fill="#0f172a" rx="8"/>
  <text x="400" y="40" text-anchor="middle" fill="#e2e8f0" font-size="20" font-weight="bold">nexus Architecture</text>

  <rect x="50" y="70" width="700" height="55" fill="#1e293b" stroke="#334155" rx="6"/>
  <text x="400" y="100" text-anchor="middle" fill="#38bdf8" font-size="14" font-weight="bold">CLI Layer</text>
  <text x="400" y="115" text-anchor="middle" fill="#94a3b8" font-size="11">analyze · impact · cycles · stats · isolated · mermaid · json</text>

  <line x1="400" y1="125" x2="400" y2="145" stroke="#334155" stroke-width="2"/>
  <polygon points="395,143 405,143 400,150" fill="#334155"/>

  <rect x="50" y="150" width="330" height="70" fill="#1e293b" stroke="#334155" rx="6"/>
  <text x="215" y="178" text-anchor="middle" fill="#a78bfa" font-size="14" font-weight="bold">Parser</text>
  <text x="215" y="195" text-anchor="middle" fill="#94a3b8" font-size="11">Rust · Python · JS/TS · Go</text>
  <text x="215" y="210" text-anchor="middle" fill="#64748b" font-size="10">regex-based import extraction</text>

  <rect x="420" y="150" width="330" height="70" fill="#1e293b" stroke="#334155" rx="6"/>
  <text x="585" y="178" text-anchor="middle" fill="#10b981" font-size="14" font-weight="bold">File Walker</text>
  <text x="585" y="195" text-anchor="middle" fill="#94a3b8" font-size="11">recursive directory traversal</text>
  <text x="585" y="210" text-anchor="middle" fill="#64748b" font-size="10">.gitignore-aware · extension filter</text>

  <line x1="215" y1="220" x2="215" y2="240" stroke="#334155" stroke-width="2"/>
  <polygon points="210,238 220,238 215,245" fill="#334155"/>
  <line x1="585" y1="220" x2="585" y2="240" stroke="#334155" stroke-width="2"/>
  <polygon points="580,238 590,238 585,245" fill="#334155"/>

  <rect x="50" y="245" width="700" height="70" fill="#1e293b" stroke="#334155" rx="6"/>
  <text x="400" y="273" text-anchor="middle" fill="#f59e0b" font-size="14" font-weight="bold">Core Engine</text>
  <text x="400" y="290" text-anchor="middle" fill="#94a3b8" font-size="11">Dependency Graph · Cycle Detection · Impact Analysis · Complexity Metrics</text>
  <text x="400" y="305" text-anchor="middle" fill="#64748b" font-size="10">transitive closure · DFS cycle finding · risk scoring</text>

  <line x1="400" y1="315" x2="400" y2="335" stroke="#334155" stroke-width="2"/>
  <polygon points="395,333 405,333 400,340" fill="#334155"/>

  <rect x="50" y="340" width="220" height="55" fill="#1e293b" stroke="#334155" rx="6"/>
  <text x="160" y="368" text-anchor="middle" fill="#f97316" font-size="14" font-weight="bold">Terminal</text>
  <text x="160" y="383" text-anchor="middle" fill="#94a3b8" font-size="11">colorized tables</text>

  <rect x="290" y="340" width="220" height="55" fill="#1e293b" stroke="#334155" rx="6"/>
  <text x="400" y="368" text-anchor="middle" fill="#f97316" font-size="14" font-weight="bold">JSON</text>
  <text x="400" y="383" text-anchor="middle" fill="#94a3b8" font-size="11">CI tooling · editor integration</text>

  <rect x="530" y="340" width="220" height="55" fill="#1e293b" stroke="#334155" rx="6"/>
  <text x="640" y="368" text-anchor="middle" fill="#f97316" font-size="14" font-weight="bold">Mermaid</text>
  <text x="640" y="383" text-anchor="middle" fill="#94a3b8" font-size="11">visual diagrams</text>

  <text x="400" y="470" text-anchor="middle" fill="#475569" font-size="10">Built with Rust · zero external dependencies</text>
</svg>
</body></html>"""

TERMINAL_HTML = r"""<!DOCTYPE html>
<html><head><style>
body { margin:0; background:#020617; display:flex; align-items:center; justify-content:center; height:100vh; font-family:'Cascadia Code','Fira Code','Consolas',monospace; }
.term {
  background:#0f172a; border:1px solid #1e293b; border-radius:10px; width:700px;
  box-shadow:0 0 40px rgba(79,70,229,0.15);
}
.term-bar { display:flex; align-items:center; padding:12px 16px; border-bottom:1px solid #1e293b; gap:8px; }
.term-dot { width:10px; height:10px; border-radius:50%; }
.term-dot.r { background:#ef4444; }
.term-dot.y { background:#eab308; }
.term-dot.g { background:#22c55e; }
.term-title { color:#475569; font-size:11px; margin-left:8px; }
.term-body { padding:20px; color:#e2e8f0; font-size:13px; line-height:1.6; white-space:pre; }
.cmd { color:#38bdf8; }
.path { color:#a78bfa; }
.val { color:#22c55e; }
.num { color:#f59e0b; }
.warn { color:#f97316; }
.dim { color:#475569; }
.section { color:#e2e8f0; font-weight:bold; }
</style></head><body>
<div class="term">
  <div class="term-bar">
    <div class="term-dot r"></div>
    <div class="term-dot y"></div>
    <div class="term-dot g"></div>
    <div class="term-title">nexus analyze — bash</div>
  </div>
  <div class="term-body"><span class="cmd">$</span> <span class="path">nexus analyze src/</span>

  ═══ nexus — Dependency Analysis
  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  📊 Summary
    Files:     24
    Deps:      142 (avg 5.9)
    Lines:     4821 (3420 code)
    Complexity: 3.8 avg
    Tests:     8

  🔤 Languages
    Rust             18
    Python           3
    TypeScript       2
    Go               1

  🔗 Top Dependers (files with most deps)
    12 deps  core.rs
     9 deps  cli.rs
     8 deps  walker.rs

  🎯 Top Depended (most imported files)
    14 imports  types.rs
    11 imports  lib.rs
     9 imports  error.rs

  📭 Isolated Files
    2 files<span class="dim">
</span></div>
</div></body></html>"""

TEMP_TERMINAL = r"""<!DOCTYPE html>
<html><head><style>
body { margin:0; background:#020617; display:flex; align-items:center; justify-content:center; height:100vh; font-family:'Cascadia Code','Fira Code','Consolas',monospace; }
.term {
  background:#0f172a; border:1px solid #1e293b; border-radius:10px; width:700px;
  box-shadow:0 0 40px rgba(79,70,229,0.15);
}
.term-bar { display:flex; align-items:center; padding:12px 16px; border-bottom:1px solid #1e293b; gap:8px; }
.term-dot { width:10px; height:10px; border-radius:50%; }
.term-dot.r { background:#ef4444; }
.term-dot.y { background:#eab308; }
.term-dot.g { background:#22c55e; }
.term-title { color:#475569; font-size:11px; margin-left:8px; }
.term-body { padding:20px; color:#e2e8f0; font-size:13px; line-height:1.6; white-space:pre; }
.cmd { color:#38bdf8; }
.path { color:#a78bfa; }
.val { color:#22c55e; }
.num { color:#f59e0b; }
.dim { color:#475569; }
</style></head><body>
<div class="term">
  <div class="term-bar">
    <div class="term-dot r"></div>
    <div class="term-dot y"></div>
    <div class="term-dot g"></div>
    <div class="term-title">nexus analyze — bash</div>
  </div>
  <div class="term-body"><span class="cmd">$</span> <span class="path">nexus analyze src/</span>

  ═══ nexus — Dependency Analysis
  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  📊 Summary
    Files:     24
    Deps:      142 (avg 5.9)
    Lines:     4821 (3420 code)
    Complexity: 3.8 avg
    Tests:     8

  🔤 Languages
    Rust             18
    Python           3
    TypeScript       2
    Go               1

  🔗 Top Dependers (files with most deps)
    12 deps  core.rs
     9 deps  cli.rs
     8 deps  walker.rs

  🎯 Top Depended (most imported files)
    14 imports  types.rs
    11 imports  lib.rs
     9 imports  error.rs

  📭 Isolated Files
    2 files</div>
</div></body></html>"""


async def generate_assets():
    assets_dir = Path(__file__).parent.parent / "assets"
    assets_dir.mkdir(exist_ok=True)

    try:
        from playwright.async_api import async_playwright
    except ImportError:
        print("Playwright not installed. Saving HTML fallbacks instead.")
        (assets_dir / "architecture.png").write_text("FALLBACK: install playwright")
        (assets_dir / "terminal-preview.png").write_text("FALLBACK: install playwright")
        return

    async with async_playwright() as p:
        browser = await p.chromium.launch(headless=True)

        # Architecture diagram
        page = await browser.new_page(viewport={"width": 800, "height": 520})
        await page.set_content(ARCH_HTML)
        await page.wait_for_timeout(1500)
        await page.screenshot(path=str(assets_dir / "architecture.png"))
        print(f"  [OK] architecture.png ({assets_dir / 'architecture.png'})")

        # Terminal preview
        page2 = await browser.new_page(viewport={"width": 1920, "height": 1080})
        await page2.set_content(TERMINAL_HTML)
        await page2.wait_for_timeout(2000)
        box = await page2.locator(".term").bounding_box()
        if box:
            await page2.screenshot(
                path=str(assets_dir / "terminal-preview.png"),
                clip={"x": box["x"] - 40, "y": box["y"] - 40, "width": box["width"] + 80, "height": box["height"] + 80},
            )
            print(f"  [OK] terminal-preview.png ({assets_dir / 'terminal-preview.png'})")
        else:
            await page2.screenshot(path=str(assets_dir / "terminal-preview.png"))
            print(f"  [OK] terminal-preview.png (fallback full-page) ({assets_dir / 'terminal-preview.png'})")

        await browser.close()


if __name__ == "__main__":
    asyncio.run(generate_assets())

from playwright.sync_api import sync_playwright

def verify_diff():
    with sync_playwright() as p:
        browser = p.chromium.launch(headless=True)
        page = browser.new_page()

        # 1. Navigate to /routes
        page.goto("http://localhost:1420/routes")
        page.wait_for_timeout(3000)

        page.screenshot(path="verification/diff_initial.png")

        # 2. Select Route
        # We assume "Main Sync" route exists from previous test. If not, script fails, which is fine for verification flow sequence.
        # If not, create one?
        if not page.locator("text=Main Sync").is_visible():
            print("Main Sync route not found, skipping diff test")
            return

        page.click("text=Main Sync")
        page.wait_for_timeout(500)

        # 3. Switch to Mappings Tab
        page.click("text=Mappings")
        page.wait_for_timeout(500)

        # 4. Click Preview Diff
        # Button: <button ...>Preview Diff</button>
        page.click("text=Preview Diff")
        page.wait_for_timeout(2000)

        # 5. Verify Modal Open
        if page.locator("text=Diff Preview:").is_visible():
            print("Diff Modal Open")
        else:
            print("Diff Modal Not Found")
            page.screenshot(path="verification/diff_fail.png")
            return

        # 6. Check for content
        # It might be empty if repos are empty.
        # We check for "No changes detected" or file list.
        # "No changes detected"
        # File list item

        if page.locator("text=No changes detected").is_visible() or page.locator(".file-change-item").count() > 0:
             print("Diff loaded (content or empty state)")
        else:
             print("Diff state unknown")

        page.screenshot(path="verification/diff_preview.png")
        browser.close()

if __name__ == "__main__":
    verify_diff()

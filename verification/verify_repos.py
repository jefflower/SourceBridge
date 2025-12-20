from playwright.sync_api import sync_playwright

def verify_repos():
    with sync_playwright() as p:
        browser = p.chromium.launch(headless=True)
        page = browser.new_page()

        # 1. Navigate to /repos
        page.goto("http://localhost:1420/repos")
        page.wait_for_timeout(3000)

        # Take initial screenshot
        page.screenshot(path="verification/repos_debug.png")

        # 2. Open Add Group Dialog
        # Try finding by icon or class if title selector fails due to i18n
        # The structure is: div.w-64.border-r... > div.p-4.border-b... > div.flex.gap-1 > button

        # Let's inspect the page content in debug if needed, but assuming structure is rendered.
        # We will try a very specific CSS selector based on the template structure provided.
        # Sidebar -> Header -> Button Group -> First Button (FolderPlus)

        try:
            # Locate the sidebar header
            header = page.locator(".w-64 .p-4.border-b")
            # Click the first button in the gap-1 container
            header.locator("button").first.click()
        except Exception as e:
            print(f"Failed to click button: {e}")
            print(page.content())
            return

        page.wait_for_timeout(1000)

        # 3. Enter details
        try:
            page.fill("input[placeholder='Repository Name']", "Test Group")
            page.click("text=Create")
        except Exception as e:
             print(f"Failed to fill form: {e}")
             page.screenshot(path="verification/form_fail.png")
             return

        page.wait_for_timeout(1000)

        # 4. Verify Tree
        if page.locator("text=Test Group").is_visible():
            print("SUCCESS: Test Group created and visible")
        else:
            print("FAILURE: Test Group not found")
            page.screenshot(path="verification/tree_fail.png")

        # Take final screenshot
        page.screenshot(path="verification/repos_tree.png")

        browser.close()

if __name__ == "__main__":
    verify_repos()

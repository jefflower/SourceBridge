from playwright.sync_api import sync_playwright

def verify_app_shell():
    with sync_playwright() as p:
        browser = p.chromium.launch(headless=True)
        page = browser.new_page()

        # Navigate to dashboard
        page.goto("http://localhost:1420/")

        # Take a screenshot immediately to see what's rendering
        page.screenshot(path="verification/debug_start.png")

        # Wait for loading
        page.wait_for_timeout(2000)

        # Verify TitleBar
        if page.locator("text=SourceBridge").is_visible():
            print("TitleBar Visible")
        else:
            print("TitleBar NOT Visible")
            print(page.content()) # Print content to debug


        # Take screenshot of Dashboard
        page.screenshot(path="verification/dashboard.png")
        print("Dashboard screenshot taken")

        browser.close()

if __name__ == "__main__":
    verify_app_shell()

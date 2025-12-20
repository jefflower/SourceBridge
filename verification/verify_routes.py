from playwright.sync_api import sync_playwright

def verify_routes():
    with sync_playwright() as p:
        browser = p.chromium.launch(headless=True)
        page = browser.new_page()

        # 1. Navigate to /routes
        page.goto("http://localhost:1420/routes")
        page.wait_for_timeout(3000)

        page.screenshot(path="verification/routes_initial.png")

        # 2. Add Route Group
        # Locate "New Route Group" button (first one in header)
        try:
            # More robust locator: find sidebar header, then button with title 'New Route Group' (if translation loaded)
            # OR first button

            # The structure in Routes.vue template:
            # <div class="w-64 border-r flex flex-col bg-muted/10">
            #   <div class="p-4 border-b flex items-center justify-between">
            #     ...
            #     <div class="flex gap-1">
            #       <button ... title="$t('route.group.new')"> ... </button>

            # Let's target by SVG icon class logic if title fails (title depends on i18n loading)
            # FolderPlus is for group. Plus is for route.

            # Trying generic nth-child in the button container again with wait
            header = page.locator(".w-64 .p-4.border-b")
            # Wait for header to be visible
            header.wait_for()

            # Click first button
            header.locator("button").first.click()

        except Exception as e:
            print(f"Failed to click New Group button: {e}")
            page.screenshot(path="verification/routes_failed_click.png")
            return

        page.wait_for_timeout(500)
        page.fill("input[placeholder='Route Name']", "Sync Group")
        page.click("text=Create")
        page.wait_for_timeout(1000)

        # 3. Add Route
        # Click "Add Route" button (second one)
        page.locator(".w-64 .p-4.border-b button").nth(1).click()
        page.wait_for_timeout(500)

        page.fill("input[placeholder='Route Name']", "Main Sync")
        # Selects might be empty if no repos, but creation should work.
        page.click("text=Create")
        page.wait_for_timeout(1000)

        # 4. Verify Tree and Selection
        if page.locator("text=Main Sync").is_visible():
            print("Route created")
            page.click("text=Main Sync")
            page.wait_for_timeout(500)
        else:
            print("Route not found in tree")
            page.screenshot(path="verification/route_fail.png")
            return

        # 5. Add Mapping Rule
        # Switch to Mappings tab
        page.click("text=Mappings")
        page.wait_for_timeout(500)

        # Click Add Rule
        # Button text is "+ Add Rule" (translated from route.mapping.add)
        # Using partial text match or class
        # In RouteDetail.vue: <button ...>+ {{ $t('route.mapping.add') }}</button>
        # English: "+ Add Rule"

        try:
            page.click("button:has-text('Add Rule')")
        except:
             # Try generic selector for the button in the header of mappings tab
             page.click(".flex.justify-between.mb-4 button")

        page.wait_for_timeout(200)

        # Fill source/target (inputs in table)
        # First row inputs
        page.locator("tbody tr").first.locator("input").first.fill("src/*.ts")
        page.locator("tbody tr").first.locator("input").nth(1).fill("dest/")

        # Save
        page.click("text=Save")
        page.wait_for_timeout(1000)

        # 6. Test Match
        page.fill("input[placeholder='Test Path (e.g. src/main.ts)']", "src/test.ts")

        # Click Check button
        try:
            page.click("text=Check")
        except:
            page.click("button:has-text('Check')")

        page.wait_for_timeout(500)

        # Check result
        # Text: "Matches Rule #1: src/test.ts -> dest/"
        # We look for "Matches Rule"
        if page.locator("text=Matches Rule").is_visible():
            print("SUCCESS: Test Match worked")
        else:
            print("FAILURE: Test Match failed")
            # print(page.content())

        page.screenshot(path="verification/routes_verified.png")
        browser.close()

if __name__ == "__main__":
    verify_routes()

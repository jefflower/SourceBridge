from playwright.sync_api import sync_playwright

def verify_sync():
    with sync_playwright() as p:
        browser = p.chromium.launch(headless=True)
        page = browser.new_page()

        # 1. Navigate to /tasks
        page.goto("http://localhost:1420/tasks")
        page.wait_for_timeout(3000)

        page.screenshot(path="verification/sync_initial.png")

        # 2. Check if a task exists, if not create one
        # Assuming verify_tasks ran before, "Test Task" might exist.
        if not page.locator("text=Test Task").is_visible():
            print("Creating sync task")
            page.click("text=+ New Task")
            page.wait_for_timeout(500)
            page.fill("input[placeholder='My Task']", "Sync Task")

            # Add Sync Step
            page.click("text=Sync Route")
            page.wait_for_timeout(200)

            # Since we don't have a real route selector in the mock form (it was an input),
            # we just leave it or fill dummy ID if needed.
            # In SyncStepForm.vue: <input ... placeholder="Route ID ...">
            # We need to fill it with something.
            # Finding the input in the step card.
            page.locator("input[placeholder='Route ID (Use Select in real app)']").fill("route-123")

            page.click("text=Save")
            page.wait_for_timeout(1000)

        # 3. Find the task card
        card = page.locator("div", has_text="Sync Task").first

        # 4. Open Logs
        # Click the List icon button
        # Locator: button[title='Logs']
        try:
            card.locator("button[title='Logs']").click()
        except:
            print("Log button not found")
            page.screenshot(path="verification/sync_fail_logs.png")
            return

        page.wait_for_timeout(500)

        # 5. Verify Log Console Open
        if page.locator("text=Execution Logs").is_visible():
            print("Log Console Open")
        else:
            print("Log Console Not Found")

        # 6. Verify Log Content (Might be empty or have previous run)
        page.screenshot(path="verification/sync_logs.png")

        # Close logs
        page.click("text=Close")
        page.wait_for_timeout(500)

        # 7. Run Task
        card.locator("button[title='Run Now']").click()
        print("Running Sync Task")
        page.wait_for_timeout(2000)

        # 8. Check Logs again
        card.locator("button[title='Logs']").click()
        page.wait_for_timeout(500)
        page.screenshot(path="verification/sync_logs_after.png")

        browser.close()

if __name__ == "__main__":
    verify_sync()

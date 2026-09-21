from playwright.sync_api import sync_playwright
import time

def capture():
    with sync_playwright() as p:
        browser = p.chromium.launch(
            headless=True,
            args=["--use-fake-ui-for-media-stream", "--use-fake-device-for-media-stream"]
        )
        context = browser.new_context(
            viewport={"width": 1366, "height": 960},
            permissions=["microphone"]
        )
        page = context.new_page()
        
        # 1. Standby Screenshot
        page.goto("http://127.0.0.1:8992/", timeout=15000)
        page.wait_for_timeout(1000)
        page.screenshot(path=r"C:\Users\pc\.gemini\antigravity\brain\d994c5fe-4115-48b5-96eb-bc5773d58886\apple_teleprompter_standby_verified.png")
        print("Captured clean standby: apple_teleprompter_standby_verified.png")
        
        # 2. Activate Client Talking tile and simulate live stream speech
        page.click("#btnClient")
        page.wait_for_timeout(400)
        
        # Call handleIncomingLiveTranscript
        page.evaluate('''() => {
            handleIncomingLiveTranscript("Your price is way too expensive, we don't have the budget for this right now.", "customer");
        }''')
        page.wait_for_timeout(1200)
        page.screenshot(path=r"C:\Users\pc\.gemini\antigravity\brain\d994c5fe-4115-48b5-96eb-bc5773d58886\apple_teleprompter_active_verified.png")
        print("Captured clean active: apple_teleprompter_active_verified.png")
        
        browser.close()

if __name__ == "__main__":
    capture()

import sys
import os
import asyncio
import json
import time
import numpy as np
import websockets
from concurrent.futures import ThreadPoolExecutor
from faster_whisper import WhisperModel

if sys.platform == "win32":
    sys.stdout.reconfigure(encoding="utf-8", errors="replace", line_buffering=True)
    sys.stderr.reconfigure(encoding="utf-8", errors="replace", line_buffering=True)

print("[LOCAL-WHISPER] Loading faster-whisper-tiny on CPU (int8)...", flush=True)
model = WhisperModel("tiny", device="cpu", compute_type="int8")
# Warm up with zero array
segs, _ = model.transcribe(np.zeros(16000, dtype=np.float32), language="en", beam_size=1, condition_on_previous_text=False)
list(segs)
print("[LOCAL-WHISPER] Engine ready & warmed up on ws://127.0.0.1:8994", flush=True)

executor = ThreadPoolExecutor(max_workers=2)

def do_transcribe(audio_np):
    try:
        segments, _ = model.transcribe(
            audio_np,
            language="en",
            beam_size=1,
            without_timestamps=True,
            vad_filter=False,
            condition_on_previous_text=False
        )
        return " ".join([s.text for s in segments]).strip()
    except Exception as e:
        print(f"[LOCAL-WHISPER] Transcribe error: {e}", flush=True)
        return ""

async def handler(websocket):
    print("[LOCAL-WHISPER] Browser microphone connected to live stream", flush=True)
    audio_buffer = []
    last_process_time = time.time()
    last_emitted_text = ""
    is_transcribing = False

    try:
        async for message in websocket:
            if isinstance(message, bytes):
                chunk = np.frombuffer(message, dtype=np.float32)
                audio_buffer.extend(chunk)

                now = time.time()
                # Run transcription every 350ms if we have at least 0.4s (6400 samples)
                if len(audio_buffer) >= 6400 and (now - last_process_time) >= 0.35 and not is_transcribing:
                    last_process_time = now
                    audio_window = np.array(audio_buffer[-32000:], dtype=np.float32)
                    
                    peak = float(np.max(np.abs(audio_window)))
                    rms = float(np.sqrt(np.mean(audio_window**2)))
                    
                    if peak > 0.008 or rms > 0.0015:
                        is_transcribing = True
                        loop = asyncio.get_running_loop()
                        text = await loop.run_in_executor(executor, do_transcribe, audio_window)
                        is_transcribing = False
                        
                        if text:
                            clean = text.replace("[BLANK_AUDIO]", "").strip()
                            if len(clean) > 1 and clean != last_emitted_text:
                                last_emitted_text = clean
                                print(f"[LOCAL-WHISPER] Spoken: '{clean}'", flush=True)
                                await websocket.send(json.dumps({
                                    "type": "transcript",
                                    "text": clean,
                                    "is_final": True
                                }))

                    # Keep rolling buffer up to 3 seconds
                    if len(audio_buffer) > 48000:
                        audio_buffer = audio_buffer[-32000:]

            elif isinstance(message, str):
                try:
                    data = json.loads(message)
                    if data.get("type") == "reset":
                        audio_buffer = []
                        last_emitted_text = ""
                except Exception:
                    pass
    except websockets.ConnectionClosed:
        print("[LOCAL-WHISPER] Browser mic stream closed", flush=True)
    except Exception as e:
        print(f"[LOCAL-WHISPER] Stream error: {e}", flush=True)

async def main():
    async with websockets.serve(handler, "127.0.0.1", 8994):
        print("[LOCAL-WHISPER] WebSocket server listening on ws://127.0.0.1:8994", flush=True)
        await asyncio.Future()

if __name__ == "__main__":
    asyncio.run(main())

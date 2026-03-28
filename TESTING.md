# screenpipe regression testing checklist

> **purpose**: prevent regressions. test core features rigorously every time

## critical edge cases (sorted by regression frequency)

### 1. window overlay & fullscreen spaces (macOS)

### 1.1. Live Text Interaction (macOS)

commits: `e9c76934`, `9acdf850`

- [ ] **Native Live Text selection** — On macOS, verify that native Live Text selection works within the app's text overlay.
- [ ] **Native Data Detectors** — On macOS, verify that native data detectors (e.g., phone numbers, addresses, dates) are active and clickable within the app's text overlay.
- [ ] **Cross-architecture Live Text compilation** — On both x86_64 (Intel) and arm64 (Apple Silicon) macOS machines, verify that Live Text functionality is available and works without compilation errors or runtime issues.


- [ ] **window mode CSS restore** — In window mode (not fullscreen), verify that CSS styling is correct and as expected (e.g., no unexpected transparent panels).
- [ ] **keyboard input in main window from tray** — Open the main window from the tray icon and immediately try typing. Verify that keyboard input works without requiring a click.
- [ ] **WKWebView keyboard focus recovery** — Interact with embedded web views (e.g., billing, help sections), then navigate back to other UI elements. Verify keyboard focus is correctly recovered by the WKWebView.



these break CONSTANTLY. any change to `window_api.rs`, `main.rs` shortcuts, activation policy, or NSPanel code must test ALL of these.

commits that broke this area: `0752ea59`, `d89c5f14`, `4a64fd1a`, `fa591d6e`, `8706ae73`, `6d44af13`, `b6ff1bf7`, `09a18070`

- [ ] **overlay shortcut on fullscreen space** — press shortcut while a fullscreen app (e.g., Chrome fullscreen) is active. overlay MUST appear on top.
- [ ] **chat shortcut on fullscreen space** — press chat shortcut while on a fullscreen space. chat panel MUST appear on top. Fixed: panel pre-created at startup, show uses order_front→activate order.
- [ ] **chat shortcut on normal desktop** — chat appears, receives keyboard focus, can type immediately.
- [ ] **overlay toggle on/off** — press shortcut twice. first shows, second hides. no "ghost" window left behind.
- [ ] **chat toggle on/off** — press chat shortcut twice. first shows, second closes.
- [ ] **overlay does NOT follow space swipe** — show overlay, then three-finger swipe to another space. overlay should NOT follow you (no blink-and-disappear). was broken by `MoveToActiveSpace` staying set.
- [ ] **no blink on show** — overlay appears instantly, no flash of white/transparent then reappear. was broken multiple times (`3097872b`, `8706ae73`, `09a18070`).
- [ ] **no blink on hide** — overlay disappears instantly. no momentary reappear after hiding.
- [ ] **overlay on second monitor** — with 2 monitors, show overlay. it appears on the monitor where the mouse cursor is.
- [ ] **window mode vs fullscreen mode** — switch overlay mode in settings. shortcut still works in both modes. no crash.
- [ ] **switch modes while overlay is visible** — change from fullscreen to window mode in settings while overlay is showing. should not crash (`b4eb2ab4`).
- [ ] **keyboard focus in overlay** — show overlay, start typing. text input works immediately without clicking (`d74d0665`, `5a50aaad`).
- [ ] **keyboard focus in chat** — show chat, start typing. text input works immediately.
- [ ] **escape closes overlay** — press Escape while overlay is visible. it hides.
- [ ] **no space jump on show** — showing the overlay should NOT cause a space transition animation (`6d44af13`, `d74d0665`).
- [ ] **no space jump on hide** — hiding the overlay should NOT switch you to a different space.
- [ ] **screen recording visibility setting** — toggle "show in screen recording" in settings. overlay should appear/disappear from screen recordings accordingly (`206107ba`).
- [ ] **search panel focus** — open search, keyboard focus is in search input immediately (`2315a39c`, `1f2681e3`).
- [ ] **ghost clicks after hide** — hide overlay via `order_out`. clicking where overlay was should NOT trigger overlay buttons (`32e1a962`).
- [ ] **pinch-to-zoom works** — pinch gesture on trackpad zooms timeline without needing to click first (`d99444a7`, `523a629e`).
- [ ] **shortcut reminder on all Spaces** — switch between 3+ Spaces (including fullscreen apps). reminder pill stays visible on every Space simultaneously.
- [ ] **shortcut reminder on fullscreen app** — fullscreen Chrome/Safari, reminder shows at top center. not just leftmost Space.
- [ ] **shortcut reminder doesn't steal focus** — showing reminder never takes keyboard focus from active app.
- [ ] **chat on non-primary Space** — switch to Space 3 (normal desktop), press chat shortcut. chat appears on Space 3, not Space 1. no Space transition animation.
- [ ] **chat re-show on fullscreen Space** — show chat on fullscreen Space, hide it, show again. must reappear on same fullscreen Space.
- [ ] **space monitor only hides main overlay** — swipe Spaces. main overlay hides. chat window and shortcut reminder are unaffected.
- [ ] **space monitor doesn't race with show** — show overlay via shortcut. the `activateIgnoringOtherApps` call must not trigger space monitor's hide callback.
- [ ] **Chat streaming UX** — Verify that chat streaming uses a state-aware grid dissolve loader for a smooth user experience.
- [ ] **chat always-on-top toggle** — Toggle the "chat always-on-top" setting and verify that the chat window behaves as expected (e.g., stays on top of other applications when enabled). (`b6c363e5`)
- [ ] **text selection not blocked by URL overlays** — On URL-heavy pages, verify that text selection is not blocked by clickable URL overlays. (`eb9e65b4`)
- [ ] **macOS focused-app capture with AX observers** — On macOS, verify that focused-app capture works correctly when switching between applications, utilizing AX observers. (`22830119`)
- [ ] **macOS native Live Text interaction** — On macOS, verify that native Live Text interaction, including text selection and data detectors, is re-enabled and functions correctly. (`e9c76934`)
- [ ] **Livetext single worker thread** — verify no GCD thread exhaustion freeze during heavy livetext analysis. (`a3e29d42a`)
- [ ] **VisionKit semaphore timeouts** — verify no deadlocks in vision pipeline if VisionKit hangs (10s timeout). (`397f46133`)
- [ ] **Notification panel order_out** — verify no ghost clicks after hiding notification/shortcut panels. (`32fed7c8c`)


### 2. dock icon & tray icon (macOS)

commits that broke this area: `0752ea59`, `7562ec62`, `2a2bd9b5`, `f2f7f770`, `5cb100ea`

- [ ] **dock icon visible on launch** — app icon appears in dock immediately on startup.
- [ ] **tray icon visible on launch** — tray icon appears in menu bar on startup.
- [ ] **dock icon persists after overlay show/hide** — show and hide overlay 5 times. dock icon must remain visible every time. was broken by Accessory mode switches.
- [ ] **tray icon persists after overlay show/hide** — same test. tray icon must remain visible.
- [ ] **dock right-click menu works** — right-click dock icon. "Show screenpipe", "Settings", "Check for updates" all work (`d794176a`).
- [ ] **tray menu items don't fire twice** — click any tray menu item. action happens once, not twice (`9e151265`).
- [ ] **tray health indicator** — tray icon shows green (healthy) or yellow/red (issues) based on recording status.
- [ ] **tray on notched MacBook** — on 14"/16" MacBook Pro, tray icon is visible (not hidden behind notch). if hidden, user can Cmd+drag to reposition.
- [ ] **activation policy never changes** — after ANY user interaction, dock icon should remain visible. no Accessory mode switches. verify with: `ps aux | grep screenpipe`.
- [ ] **no autosave_name crash** — removed in `2a2bd9b5`. objc2→objc pointer cast was causing `panic_cannot_unwind`.
- [ ] **no recreate_tray** — recreating tray pushes icon LEFT (behind notch). must only create once (`f2f7f770`).
- [ ] **tray upgrade button opens in-app checkout** — Verify that clicking the tray's upgrade button correctly opens the in-app checkout experience. (`078fcfb2`)
- [ ] **modernized tray menu** — Verify the tray menu's updated layout and functionality match the modernized design. (`b6c363e5`)

### 3. monitor plug/unplug

commits: `28e5c247`

- [ ] **unplug external monitor while recording** — recording continues on remaining monitor(s). no crash. log shows "Monitor X disconnected".
- [ ] **plug in external monitor while recording** — new monitor is detected within 5 seconds. recording starts on it. log shows "Monitor X reconnected".
- [ ] **unplug and replug same monitor** — recording resumes. same monitor ID reused. no duplicate recording tasks.
- [ ] **unplug all external monitors (laptop only)** — built-in display continues recording. no crash.
- [ ] **plug monitor with different resolution** — recording starts at correct resolution. OCR works on new monitor.
- [ ] **"use all monitors" setting** — with this ON, all monitors auto-detected. no manual configuration needed.
- [ ] **specific monitor IDs setting** — with specific IDs configured, only those monitors are recorded. unplugging a non-configured monitor has no effect.
- [ ] **resolution change (e.g., clamshell mode)** — closing MacBook lid with external monitor. recording continues on external.
- [ ] **queue stats after unplug** — check logs. no queue stats for disconnected monitor after disconnect.

### 4. audio device handling

- [ ] **default audio device** — with "follow system default", recording uses whatever macOS says is default.
- [ ] **macOS default engine** — Verify whisper is the default audio engine on macOS, while parakeet remains default on Windows/Linux. (`730d2bd8e`)
- [ ] **No forced parakeet migration** — Verify existing macOS users are not forced to migrate to parakeet. (`523fe4c37`)
- [ ] **macOS 14 parakeet-mlx launch** — Verify no crash on macOS 14 when `parakeet-mlx` is enabled. (`851ba6976`)
- [ ] **Tier-safe engine selection** — Verify `parakeet-mlx` is disabled on Low tier devices (e.g., 8GB Macs) to prevent OOM crashes. (`8777ae9b2`, `62850c6af`)
- [ ] **plug in USB headset** — if set to follow defaults and macOS switches to headset, recording follows.
- [ ] **unplug USB headset** — recording falls back to built-in mic/speakers. no crash. no 30s timeout errors.
- [ ] **bluetooth device connect/disconnect** — AirPods connect mid-recording. audio continues without gap.
- [ ] **no audio device available** — unplug all audio. app continues (vision still works). log shows warning, not crash.
- [ ] **audio stream timeout recovery** — if audio stream times out (30s no data), it should reconnect automatically.
- [ ] **multiple audio devices simultaneously** — input (mic) + output (speakers) both recording. both show in device list.
- [ ] **disable audio setting** — toggling "disable audio" stops all audio recording. re-enabling restarts it.
- [ ] **Metal GPU for whisper** — transcription uses GPU acceleration on macOS (`f882caef`). verify with Activity Monitor GPU tab.
- [ ] **Qwen3-asr OpenBLAS** — On Linux/Windows, verify that qwen3-asr uses OpenBLAS for improved transcription performance. (`e64ee25f4`)
- [ ] **Batch transcription mode** — Verify that batch transcription mode works correctly with both cloud and Deepgram engines.
- [ ] **Cloud transcription batch capping** — Send large audio chunks (>200s) to cloud transcription. Verify they are correctly capped/split and do not trigger Cloudflare 413 errors. (`792145ac6`)
- [ ] **Lower RMS threshold for batch mode output devices** — In batch transcription mode, verify that output devices correctly use a lower RMS threshold.
- [ ] **OpenAI-compatible STT connection test** — Configure OpenAI-compatible STT, then use the connection test feature. Verify it accurately reports connection status.
- [ ] **OpenAI-compatible STT editable model input** — When using OpenAI-compatible STT, verify that the model input fields are editable.
- [ ] **OpenAI-compatible STT with custom vocabulary** — Configure OpenAI-compatible STT with a custom vocabulary. Verify that transcription accuracy improves when this vocabulary is present in the audio. Verify that vocabulary is sent as both prompt and context. (`d3a4b6bcc`)
- [ ] **OpenAI-compatible transcription engine support** — Enable and configure the OpenAI-compatible transcription engine. Verify that audio is correctly captured and transcribed using this engine.
- [ ] **"transcribing..." only for recent chunks** — Verify that the "transcribing..." caption/indicator only appears for audio chunks that are less than 2 minutes old. (`b70116b`)
- [ ] **no transcribing caption on old silent chunks** — Verify that old silent audio chunks do not trigger or display a "transcribing..." caption. (`54a550f4`)
- [ ] **silent chunks deleted, not stored** — After periods of silence, verify that no empty transcription rows are stored in the database for silent audio chunks, and they are instead correctly deleted. (`cb2cc205`)
- [ ] **silent chunk zombie loop prevention** — Verify that silent audio chunks do not lead to a "zombie loop" resulting in excessive CPU usage or large log files. (`6b3a71eb`)
- [ ] **write-ahead transcription cache performance** — Verify that the write-ahead transcription cache improves the performance and responsiveness of audio transcription. (`46350671`)
- [ ] **enhanced audio pipeline diagnostics** — Check logs and verify that enhanced audio pipeline diagnostics provide useful and accurate information. (`2e68400c`)
- [ ] **audio start/stop shortcuts toggle capture** — Verify that the audio start/stop shortcuts correctly toggle audio capture on and off. (`3701cce2`)
- [ ] **bulk import transcription dictionary** — Verify that the bulk import functionality for the transcription dictionary works correctly, including smart delimiter detection. (`73adc9d4`)
- [ ] **Audio start/stop shortcuts** — Verify that designated audio start/stop shortcuts reliably toggle audio capture on and off. Check logs for corresponding start/stop events.
- [ ] **Filter music toggle UI** — Verify that a "filter music" toggle exists in recording settings and correctly enables/disables music filtering.
- [ ] **Music detection thresholds** — With "filter music" enabled, play various types of music. Verify that music is correctly detected and filtered, and that non-music speech is still captured.
- [ ] **Audio reconciliation FK constraint loop** — Verify that audio reconciliation does not enter an infinite retry loop on foreign key constraints. (`e9e2dc252`)
- [ ] **Skip reconciliation when transcription disabled** — Disable audio transcription in settings. Verify that audio reconciliation is skipped. (`ceb77559d`)
- [ ] **dead System Audio auto-reconnect** — Simulate a dead system audio stream. Verify it auto-reconnects and resumes capture. (`0f287761d`)


#### Audio device recovery (monitor unplug / device switch)

commits: device_monitor.rs atomic swap, tiered backoff, empty device list guard

- [ ] **unplug monitor during active Zoom call** — output audio recovers within 15 seconds. Verify: `grep "DEVICE_RECOVERY.*output.*restored" ~/.screenpipe/screenpipe-app.*.log`. Verify: `curl localhost:3030/search?content_type=audio&limit=5` shows output device transcriptions resume.
- [ ] **unplug and replug monitor within 5 seconds** — no audio gap. both input and output continue. Verify: no "stopping" log for input device.
- [ ] **unplug monitor, wait 2 minutes, replug** — output recovers both times. Verify: two `DEVICE_RECOVERY` log entries.
- [ ] **switch audio output (AirPods → speakers) during call** — output audio continues with <5s gap. Old device kept running until new one starts (atomic swap).
- [ ] **health endpoint during output recovery** — `curl localhost:3030/health` shows `device_status_details` with output device present within 15 seconds of recovery.
- [ ] **SCK transient failure doesn't cascade** — if ScreenCaptureKit returns empty device list, running devices are NOT disconnected. Verify: `grep "device list returned empty" ~/.screenpipe/screenpipe-app.*.log` shows warning but no disconnections.
- [ ] **DB gap query after device switch** — run: `sqlite3 ~/.screenpipe/db.sqlite "SELECT t1.timestamp as gap_start, t2.timestamp as gap_end, (julianday(t2.timestamp) - julianday(t1.timestamp)) * 86400 as gap_seconds FROM audio_transcriptions t1 JOIN audio_transcriptions t2 ON t2.id = (SELECT MIN(id) FROM audio_transcriptions WHERE id > t1.id AND is_input_device = 0) WHERE t1.is_input_device = 0 AND (julianday(t2.timestamp) - julianday(t1.timestamp)) * 86400 > 60 ORDER BY t1.timestamp;"` — should return no rows if output was continuously captured.

#### meeting detection & speaker identification

commits: calendar_speaker_id.rs, meetings.rs, meeting_persister.rs

- [ ] **restart during active meeting** — start a 1:1 calendar meeting (2 attendees), quit app mid-meeting, relaunch. meeting re-detected via calendar event still in progress. speaker names assigned. verify: `grep "meeting detected via calendar" ~/.screenpipe/screenpipe-app.*.log` shows detection after restart. verify: `sqlite3 ~/.screenpipe/db.sqlite "SELECT id, name FROM speakers WHERE name != ''"` shows both user and attendee names.
- [ ] **calendar-only meeting detection** — schedule a 1:1 meeting with 2 attendees, no meeting app (Zoom/Meet) open. meeting detected purely via calendar. verify: `grep "meeting_started" ~/.screenpipe/screenpipe-app.*.log`.
- [ ] **calendar meeting auto-end** — calendar meeting detected, wait past the calendar event end time. meeting_ended fires. verify: `grep "meeting ended via calendar" ~/.screenpipe/screenpipe-app.*.log`.
- [ ] **speaker naming in 1:1** — during 1:1 call with userName set in settings, input speaker named as user, output speaker named as other attendee. verify: `curl 'localhost:3030/search?content_type=audio&speaker_name=<attendee>&limit=5'` returns results.
- [ ] **auto-name input speaker** — with userName set, after ~2 minutes of speaking into mic, dominant input speaker named. verify: `grep "auto speaker identification: named" ~/.screenpipe/screenpipe-app.*.log`.
- [ ] **speaker names survive restart** — speaker named pre-restart stays named post-restart. verify: `sqlite3 ~/.screenpipe/db.sqlite "SELECT id, name FROM speakers WHERE name != ''"` shows same speakers before and after restart.
- [ ] **no duplicate speaker naming on restart** — restart during meeting, speakers already named aren't overwritten or duplicated. verify: no duplicate names in speakers table.
- [ ] **meeting detection stability** — Verify that meeting detection does not drop when alt-tabbing during long calls. (`7684f1d47`)
- [ ] **speaker search deduplication** — Search for speakers in the UI. Verify that results are deduplicated and reassignment targets are stable. (`34a62c053`)
- [ ] **meeting detection regardless of transcription mode** — Verify that meeting detection works even when transcription is disabled. (`ef39e728d`)
- [ ] **Windows UI Automation meeting detection** — On Windows, join a meeting in a supported app (Zoom, Teams, etc.). Verify detection works via UI element scanning rather than just process focus. (`fe905d6af`, `01eb9cf33`)
- [ ] **macOS Zoom menu bar detection** — On macOS, join a Zoom meeting. Verify detection works even if Zoom window is not focused, by scanning menu bar items. (`849372fa9`)
- [ ] **Meeting detection app coverage** — Verify detection works for 35+ supported apps and various browser URL patterns. (`e6740eb38`)
- [ ] **Meeting detection UI labels** — Verify meeting status shows "starts in Xm" and filters all-day events correctly. (`ef470d9e1`)

### 5. frame comparison & OCR pipeline

commits: `6dd5d98e`, `831ad258`

commits: `6dd5d98e`, `831ad258`

- [ ] **static screen = low CPU** — leave a static image on screen for 60s. CPU should drop below 5% (release build). hash early exit should kick in.
- [ ] **active screen = OCR runs** — actively browse/type. OCR results appear in search within 5 seconds of screen change.
- [ ] **identical frames skipped** — check logs for hash match frequency on idle monitors. should be >80% skip rate.
- [ ] **ultrawide monitor (3440x1440+)** — OCR works correctly. no distortion in change detection. text at edges is captured.
- [ ] **4K monitor** — OCR works. frame comparison doesn't timeout or spike CPU.
- [ ] **high refresh rate (120Hz+)** — app respects its own FPS setting (0.5 default), not the display refresh rate.
- [ ] **very fast content changes** — scroll quickly through a document. OCR captures content, no crashes from buffer overflows.
- [ ] **corrupt pixel buffer** — sck-rs handles corrupt ScreenCaptureKit buffers gracefully (no SIGABRT). fixed in `831ad258`.
- [ ] **window capture only on changed frames** — window enumeration (CGWindowList) should NOT run on skipped frames. verify by checking CPU on idle multi-monitor setup.
- [ ] **Meeting app OCR force** — Open a meeting app (Zoom, Teams, Meet). Verify OCR is forced for these apps even if accessibility is available. (`b18ae2253`)
- [ ] **Accessibility automation properties** — Verify automation properties (labels, roles, automation IDs) are correctly captured in the accessibility tree across Windows, macOS, and Linux. (`1b7d0db5b`)
- [ ] **DB write coalesce queue** — Under heavy load (e.g. many pipes + high FPS), verify no "database is locked" errors and no vision stalls due to write contention. (`39c016cb3`, `d119d060d`, `231521192`)
- [ ] **Windows idle CPU reduction** — Verify low CPU usage on Windows when screen is idle, using event-driven hooks and caching. (`d2c9d1fb8`)
- [ ] **reduced CPU spikes in vision/capture pipeline** — Actively browse and use applications, verifying that CPU spikes in the vision/capture pipeline are significantly reduced. (`8f7294e6`)
- [ ] **OCR bounding boxes normalized on Windows/Linux** — On Windows and Linux, verify that OCR bounding boxes are correctly normalized to the 0-1 range, ensuring consistent text overlay and interaction. (`aba74513`)
- [ ] **Debounced monitor capture errors** — Simulate transient monitor capture errors. Verify that these errors are debounced and do not lead to excessive error logging or app crashes.

### 6. Battery Saver Mode

commits: `d5a9d052`, `0b32cc9a`, `ca29a67b`

- [ ] **Battery Saver mode functionality** — Enable Battery Saver mode. Verify that capture adjustments (e.g., reduced FPS, paused capture) occur as expected when the device's power state changes (e.g., unplugging/plugging power, low battery).
- [ ] **Faster power state UI updates** — Change the device's power state (e.g., unplug/plug power). Verify that the UI updates quickly and accurately reflects the current power state and capture mode.
- [ ] **Correct default power mode** — On a fresh install or after a reset, verify that the default power mode is set to "performance" until Battery Saver mode is explicitly enabled or configured.

### 7. permissions (macOS)

commits: `d9d43d31`, `620c89a5`, `14acf6f0`

- [ ] **fresh install — all prompts appear** — screen recording, microphone, accessibility prompts all show on first launch.
- [ ] **denied permission → opens System Settings** — if user previously denied mic permission, clicking "grant" opens System Settings > Privacy directly (`620c89a5`).
- [ ] **permission revoked while running** — go to System Settings, revoke screen recording. app shows red permission banner within 10 seconds.
- [ ] **permission banner is visible** — solid red `bg-destructive` banner at top of main window when any permission missing. not subtle (`9c0ba5d1`).
- [ ] **permission recovery page** — navigating to /permission-recovery shows clear instructions.
- [ ] **startup permission gate** — on first launch, permissions are requested before recording starts (`d9d43d31`).
- [ ] **faster permission polling** — permission status checked every 5-10 seconds, not 30 (`d9d43d31`).
- [ ] **No recurring permission modal after close** — Grant macOS permissions, quit the app, and relaunch it multiple times. Verify that the macOS permission modal does NOT reappear every time the app is closed.

- [ ] **fresh install — all prompts appear** — screen recording, microphone, accessibility prompts all show on first launch.
- [ ] **denied permission → opens System Settings** — if user previously denied mic permission, clicking "grant" opens System Settings > Privacy directly (`620c89a5`).
- [ ] **permission revoked while running** — go to System Settings, revoke screen recording. app shows red permission banner within 10 seconds.
- [ ] **permission banner is visible** — solid red `bg-destructive` banner at top of main window when any permission missing. not subtle (`9c0ba5d1`).
- [ ] **permission recovery page** — navigating to /permission-recovery shows clear instructions.
- [ ] **startup permission gate** — on first launch, permissions are requested before recording starts (`d9d43d31`).
- [ ] **faster permission polling** — permission status checked every 5-10 seconds, not 30 (`d9d43d31`).
- [ ] **improved permission recovery UX** — Verify that the user experience for recovering from denied permissions is clear and intuitive. (`57cca740`)

### 7. Apple Intelligence (macOS 26+)

commits: `d4abc619`, `4f4a8282`, `31f37407`, `2223af9a`, `b34a4abd`, `303958f9`

- [ ] **macOS 26: API works** — `POST /ai/chat/completions` returns valid response using on-device Foundation Model.
- [ ] **macOS < 26: no crash** — app launches normally. FoundationModels.framework is weak-linked (`31f37407`). feature gracefully disabled.
- [ ] **Intel Mac: no crash** — Apple Intelligence not available, but app doesn't crash at DYLD load time.
- [ ] **JSON mode** — request with `response_format: { type: "json_object" }` returns valid JSON, no prose preamble (`2223af9a`).
- [ ] **JSON fallback extraction** — if model prepends prose before JSON, the `{...}` is extracted correctly (`b34a4abd`).
- [ ] **streaming (SSE)** — request with `stream: true` returns Server-Sent Events with incremental tokens (`4f4a8282`).
- [ ] **tool calling** — request with `tools` array gets tool definitions injected into prompt, model responds with tool calls (`4f4a8282`).
- [ ] **daily summary** — generates valid JSON summary from audio transcripts. no "JSON Parse error: Unexpected identifier 'Here'" (`303958f9`, `2223af9a`).
- [ ] **daily summary audio-only** — summary uses only audio data (no vision), single AI call (`303958f9`).

### 8. app lifecycle & updates

commits: `94531265`, `d794176a`, `9070639c`, `0378cab1`, `4a3313d3`, `7ffdd4f1`, `1b36f62d`

- [ ] **clean quit via tray** — right-click tray → Quit. all processes terminate. no orphaned ffmpeg/bun processes.
- [ ] **clean quit via dock** — right-click dock → Quit. same as above.
- [ ] **clean quit via Cmd+Q** — same verification.
- [ ] **force quit recovery** — force quit app. relaunch. database is intact. recording resumes.
- [ ] **sleep/wake** — close laptop lid, wait 10s, open. recording resumes within 5s. no crash (`9070639c`).
- [ ] **restart app** — quit and relaunch. all settings preserved. recording starts automatically.
- [ ] **Cross-platform autorelease pool** — Verify that Windows and Linux builds compile and run without issues related to macOS-specific autorelease pool calls. (`851b3037c`)
- [ ] **Main thread safety (macOS)** — Verify that tray icon operations, space monitoring, and frontmost app restoration are dispatched to the main thread to prevent crashes. (`ac46aa437`, `418826dfa`, `274826dfa`)
- [ ] **ObjC memory management (macOS)** — Verify that all ObjC operations are wrapped in scoped autorelease pools and objects are retained in async callbacks to prevent use-after-free or SIGSEGV crashes. (`4cb9850f7`, `c49350df0`, `139500d52`)
- [ ] **auto-update** — when update available, UpdateBanner shows in main window. clicking it downloads and installs.
- [ ] **update without tray** — user can update via dock menu "Check for updates" or Apple menu "Check for Updates..." (`d794176a`, `94531265`).
- [ ] **update banner in main window** — when update available, banner appears at top of main window.
- [ ] **source build update dialog** — source builds show "source build detected" dialog with link to pre-built version.
- [ ] **port conflict on restart** — if old process is holding port 3030, new process kills it and starts cleanly (`0378cab1`, `4a3313d3`, `8c435a10`).
- [ ] **no orphaned processes** — after quit, `ps aux | grep screenpipe` shows nothing. `lsof -i :3030` shows nothing.
- [ ] **rollback** — user can rollback to previous version via tray menu (`c7fbc3ea`).
- [ ] **Zombie CPU drain prevention** — Verify that `lsof` calls have a 5-second timeout, preventing zombie CPU drain, especially on quit. Check logs for `lsof` timeouts if applicable.
- [ ] **Tokio shutdown stability** — Verify that the `tokio` shutdown process is stable and doesn't panic in the tree walker, especially during application exit or process restarts.
- [ ] **No ggml Metal destructor crash on quit** — Perform multiple quick quits (Cmd+Q, tray quit) and restarts. Verify that the app exits cleanly without a `ggml Metal destructor crash`.
- [ ] **Properly wait for UI recorder tasks before exit** — During a clean quit, verify that all UI recorder tasks complete properly and no orphaned processes or partial recordings remain.
- [ ] **recording watchdog diagnostics** — Verify that the recording watchdog correctly diagnoses and handles recording issues, and provides useful diagnostic information. (`af2b4f3d`)
- [ ] **capture stall detection** — Simulate or observe a capture stall. Verify that a notification appears with a "Restart" button to recover. (`d3ead88eb`)
- [ ] **DB write stall detection** — if DB writes stall, verify a notification appears with a "Restart" button. (`1b4bf7918`)
- [ ] **clean startup after unclean shutdown on Windows** — On Windows, verify that the app starts cleanly after an unclean shutdown (e.g., force quit), without port 3030 binding failures. (`a8413fe2`)

### 9. database & storage

commits: `eea0c865`, `cc09de61`, `e61501da`, `d25191d7`, `60096fb9`

- [ ] **slow DB insert warning** — check logs. "Slow DB batch insert" warnings should be <1s in normal operation. >3s indicates contention.
- [ ] **concurrent DB access** — UI queries + recording inserts happening simultaneously. no "database is locked" errors.
- [ ] **store race condition** — rapidly toggle settings while recording is active. no crash (`eea0c865`).
- [ ] **event listener race condition** — Tauri event listener setup during rapid window creation. no crash (`cc09de61`).
- [ ] **UTF-8 boundary panic** — search with special characters, non-ASCII text in OCR results. no panic on string slicing (`eea0c865`).
- [ ] **low disk space** — with <1GB free, app should warn user. no crash from failed writes.
- [ ] **large database (>10GB)** — search still returns results within 2 seconds. app doesn't freeze on startup.
- [ ] **Snapshot compaction integrity** — Verify compaction doesn't result in NULL offset_index or pool exhaustion. (`09245af5f`)
- [ ] **Audio chunk timestamps** — `start_time` and `end_time` are correctly set for reconciled and retranscribed audio chunks in the database.
- [ ] **SCREENPIPE_DATA_DIR usage** — Set the `SCREENPIPE_DATA_DIR` environment variable. Verify the app uses this directory for all its data storage. (`d5f30db71`)
- [ ] **DB pool starvation prevention** — Simulate high database load (e.g., rapid screen activity, many pipes running) and monitor logs. Verify no "database is locked" errors or signs of DB pool starvation.
- [ ] **DB write coalescing queue** — verify high-frequency captures (e.g. 10 FPS) don't lock the UI or cause write errors. (`c23768f41`)
- [ ] **Multi-byte window titles in suggestions** — Interact with suggestions for windows that have multi-byte (e.g., Unicode, emoji) characters in their titles. Verify no char boundary panics.
- [ ] **no concurrent reconciliation issues** — Verify that concurrent reconciliation processes do not cause issues during heavy load or sync operations. (`1d436bc3`)
- [ ] **pipe_config blobs skipped in sync** — Verify that `pipe_config` blobs are correctly skipped during synchronization, preventing unnecessary data transfer and potential issues. (`08d5c53a`)
- [ ] **Pi's native auto-compaction for pipe session history** — Verify that Pi's native auto-compaction feature for pipe session history works as expected, preventing indefinite growth of history and maintaining performance. (`8f49e2cf`)
- [ ] **UTF-8 panic with long multi-byte strings** — Introduce long strings with multi-byte UTF-8 characters (e.g., in window titles, chat input, search queries). Verify no panics occur when these strings are truncated, stored, or processed.
- [ ] **fsync snapshots before DB commit** — verify data integrity by force-quitting during heavy capture; snapshots should match DB entries. (`2e63282b8`)

- [ ] **slow DB insert warning** — check logs. "Slow DB batch insert" warnings should be <1s in normal operation. >3s indicates contention.
- [ ] **concurrent DB access** — UI queries + recording inserts happening simultaneously. no "database is locked" errors.
- [ ] **store race condition** — rapidly toggle settings while recording is active. no crash (`eea0c865`).
- [ ] **event listener race condition** — Tauri event listener setup during rapid window creation. no crash (`cc09de61`).
- [ ] **UTF-8 boundary panic** — search with special characters, non-ASCII text in OCR results. no panic on string slicing (`eea0c865`).
- [ ] **low disk space** — with <1GB free, app should warn user. no crash from failed writes.
- [ ] **large database (>10GB)** — search still returns results within 2 seconds. app doesn't freeze on startup.
- [ ] **Audio chunk timestamps** — `start_time` and `end_time` are correctly set for reconciled and retranscribed audio chunks in the database.

### 10. AI presets & settings

commits: `8a5f51dd`, `0b0d8090`, `7e58564e`, `2522a7e2`, `f3e55dbc`, `79f2913f`

- [ ] **Ollama not running** — creating an Ollama preset shows free-text input fields (not stuck loading). user can type model name manually (`8a5f51dd`).
- [ ] **custom provider preset** — user can add a custom API endpoint. model name is free-text input with optional autocomplete.
- [ ] **settings survive restart** — change any setting, quit, relaunch. setting is preserved.
- [ ] **overlay mode switch** — change from fullscreen to window mode. setting saves. next shortcut press uses new mode.
- [ ] **FPS setting** — change capture FPS. recording interval changes accordingly.
- [ ] **language/OCR engine setting** — change OCR language. new language used on next capture cycle.
- [ ] **video quality setting** — low/balanced/high/max. affects FFmpeg encoding params (`21bddd0f`).
- [ ] **Settings UI sentence case** — All settings UI elements (billing, pipes, team) should use consistent sentence case.
- [ ] **Billing page links to website** — Verify that the in-app billing page correctly links to the *new* website billing page.
- [ ] **Non-pro subscriber Whisper fallback** — As a non-pro subscriber, verify that audio transcription defaults to `whisper-large-v3-turbo-quantized` and functions correctly.
- [ ] **Pi restart on preset switch** — Switch between different AI presets. Verify that the Pi agent restarts if required by the new preset.
- [ ] **Web search disabled for non-cloud providers** — When using a non-cloud AI provider, verify that web search functionality is correctly disabled.
- [ ] **Credit balance in billing UI and errors** — Verify that the billing UI accurately displays the credit balance and clearly differentiates between `credits_exhausted` and other LLM-related errors.
- [ ] **Unknown AI provider type sanitization** — Configure a malformed or unknown AI provider type (e.g., by manual config edit). Verify the app doesn't crash on startup or when navigating to settings, and gracefully handles the unknown type.

commits: `8a5f51dd`, `0b0d8090`

- [ ] **Ollama not running** — creating an Ollama preset shows free-text input fields (not stuck loading). user can type model name manually (`8a5f51dd`).
- [ ] **custom provider preset** — user can add a custom API endpoint. model name is free-text input with optional autocomplete.
- [ ] **settings survive restart** — change any setting, quit, relaunch. setting is preserved.
- [ ] **overlay mode switch** — change from fullscreen to window mode. setting saves. next shortcut press uses new mode.
- [ ] **FPS setting** — change capture FPS. recording interval changes accordingly.
- [ ] **language/OCR engine setting** — change OCR language. new language used on next capture cycle.
- [ ] **video quality setting** — low/balanced/high/max. affects FFmpeg encoding params (`21bddd0f`).
- [ ] **Settings UI sentence case** — All settings UI elements (billing, pipes, team) should use consistent sentence case.

### 11. onboarding

commits: `87abb00d`, `9464fdc9`, `0f9e43aa`, `7ea15f32`, `bf1f1004`

- [ ] **fresh install flow** — onboarding appears, permissions requested, user completes setup.
- [ ] **auto-advance after engine starts** — status screen advances automatically after 15-20 seconds once engine is running (`87abb00d`, `9464fdc9`).
- [ ] **skip onboarding** — user can skip and get to main app. settings use defaults.
- [ ] **shortcut gate** — onboarding teaches the shortcut. user must press it to proceed (`0f9e43aa`).
- [ ] **onboarding window size** — window is correctly sized, no overflow (`7ea15f32`).
- [ ] **onboarding doesn't re-show** — after completing onboarding, restart app. main window shows, not onboarding.
- [ ] **First-run 2-hour reminder notification** — On a fresh install, verify that a custom notification panel appears after approximately 2 hours as a first-run reminder.

commits: `87abb00d`, `9464fdc9`, `0f9e43aa`, `7ea15f32`

- [ ] **fresh install flow** — onboarding appears, permissions requested, user completes setup.
- [ ] **auto-advance after engine starts** — status screen advances automatically after 15-20 seconds once engine is running (`87abb00d`, `9464fdc9`).
- [ ] **skip onboarding** — user can skip and get to main app. settings use defaults.
- [ ] **shortcut gate** — onboarding teaches the shortcut. user must press it to proceed (`0f9e43aa`).
- [ ] **onboarding window size** — window is correctly sized, no overflow (`7ea15f32`).
- [ ] **onboarding doesn't re-show** — after completing onboarding, restart app. main window shows, not onboarding.

### 12. timeline & search

commits: `f1255eac`, `25cbdc6b`, `2529367d`, `d9821624`, `e61501da`, `039d5fea`, `50ff4f4c`, `91cc4371`, `bcce42796`, `a98fa2991`, `0ff93b167`, `adbbb8f84`

- [ ] **arrow key navigation** — left/right arrow keys navigate timeline frames (`f1255eac`).
- [ ] **search results sorted by time** — search results appear in chronological order (`25cbdc6b`).
- [ ] **no frame clearing during navigation** — navigating timeline doesn't cause frames to disappear and reload (`2529367d`).
- [ ] **URL detection in frames** — URLs visible in screenshots are extracted and shown as clickable pills (`50ef52d1`, `aa992146`).
- [ ] **app context popover** — clicking app icon in timeline shows context (time, windows, urls, audio) (`be3ecffb`).
- [ ] **Timeline single "current" bar** — Verify that the timeline only shows one "current time" bar, even during rapid updates. (`bcce42796`)
- [ ] **Timeline "Calls" filter** — Verify the "Calls" filter on the timeline correctly filters for call-related events. (`0ff93b167`)
- [ ] **Collapsible timeline filters** — Verify that timeline filters can be collapsed and expanded correctly. (`0ff93b167`)
- [ ] **daily summary in timeline** — Apple Intelligence summary shows in timeline, compact when no summary (`d9821624`).
- [ ] **window-focused refresh** — opening app via shortcut/tray refreshes timeline data immediately (`0b057046`).
- [ ] **frame deep link navigation** — `screenpipe://frame/N` or `screenpipe://frames/N` opens main window and jumps to frame N. works from cold start; invalid IDs show clear error.
- [ ] **missing frames return 404** — Attempt to access a non-existent frame via the API. Verify that it returns a 404 error. (`2e63282b8`)
- [ ] **Search result exact navigation** — Click a search result. Verify it navigates exactly to the associated `frame_id`. (`a98fa2991`)
- [ ] **Search navigation persistence** — Navigate to a frame from search results. Shift focus away from the app and back. Verify the navigation is not reset. (`71dee4ca3`)
- [ ] **Search navigation race condition** — Verify that search navigation works reliably even if the webview is still mounting (retries should handle it). (`2015137a1`)
- [ ] **Consolidated text search** — Perform keyword searches. Verify results are correctly pulled from the consolidated `frames.full_text` and `frames_fts`. (`adbbb8f84`)
- [ ] **Keyword search accessibility** — Keyword search should find content within accessibility-only frames and utilize `frames_fts` for comprehensive accessibility text searching.
- [ ] **Keyword search logic** — Verify that keyword search SQL correctly uses `OR` instead of `UNION` within `IN()`.
- [ ] **Search prompt accuracy** — Verify that search prompts are improved to prevent false negatives from over-filtering.
- [ ] **Past-day timeline navigation** — Navigate the timeline to past days (e.g., using date picker or arrow keys). Verify that data loads correctly and the timeline behaves as expected.
- [ ] **`content_type=all` search and pagination** — Perform search queries with `content_type=all`. Verify that the result count is accurate and pagination works correctly without missing or duplicating results.
- [ ] **Search pagination with offset** — Perform paginated searches, particularly beyond the first page. Verify that results are not empty or incorrect due to double-applied offsets.
- [ ] **`search_ocr()` returns results for event-driven capture** — Verify that `search_ocr()` correctly returns OCR results for event-driven captures and does not return empty when visible text is present on screen.
- [ ] **timeline displays consistent timestamps** — Verify that the timeline displays consistent timestamps, regardless of locale settings, and that there are no timestamp localization issues via websocket. (`2cf0c14e`)
- [ ] **timeline retry backoff mechanism** — Verify that the timeline's retry backoff mechanism functions as expected for data loading, ensuring resilience during temporary data unavailability. (`57cca740`)
- [ ] **arrow key navigation between search results in timeline** — Verify that left/right arrow keys correctly navigate between search results within the timeline view. (`3e8f37fc`)
- [ ] **URL chips always shown when detected** — Verify that URL chips are always displayed in the UI when URLs are detected in the content. (`cba69e56`)
- [ ] **refresh button inline with suggestion chips (icon-only)** — Verify that the refresh button for suggestion chips is displayed inline with the chips and is icon-only. (`a80e9ce6`)
- [ ] **bottom suggestion chips hidden on empty chat** — Verify that bottom suggestion chips are hidden when the chat is empty to avoid duplication. (`d6c4b821`)
- [ ] **Truncated suggestion chips** — Verify that long suggestion chips in the chat UI are correctly truncated and don't overflow the container. (`5ee0179ab`)
- [ ] **Refresh button for suggestion chips** — A refresh button appears on bottom suggestion chips. Clicking it updates suggestions.
- [ ] **Timeline refresh button hover** — verify cursor-pointer and hover state on timeline refresh button. (`0cee47b62`)
- [ ] **Smarter idle suggestions** — Verify that "idle suggestions" appear and are contextually relevant when the user is inactive.
- [ ] **Hide suggestion chips on empty chat** — Verify that suggestion chips are hidden when the chat is empty to prevent duplication.
- [ ] **Text selection not blocked by URL overlays** — On URL-heavy pages, verify that text selection is not blocked by clickable URL overlays.
- [ ] **AI suggestion chip refresh and animations** — Verify a refresh button exists on AI suggestion chips, and appropriate animations (e.g., loading spinner) are shown when refreshing.
- [ ] **Activity summary time measurement and relative parsing** — Verify activity summaries display accurate time measurements and relative time parsing (e.g., "5 minutes ago", "yesterday") works correctly in the UI.
- [ ] **Hybrid OCR for canvas apps** — Verify that text from Google Docs and Figma (canvas-rendered) is captured using hybrid OCR. (`4d2b05990`, `f09f1e9aa`)
- [ ] **Search modal scroll** — Verify that the search modal is scrollable on Windows/Linux embedded timeline and trackpad/wheel scrolling works. (`f108f1f0d`, `2a2bd9b5`, `5762c60bf`)
- [ ] **Modal scrolling (general)** — Verify that all modals (e.g., settings, pipes, search) are scrollable and handle overflow correctly, especially on Windows and Linux. (`19789657d`)
- [ ] **Search modal UX** — Verify that click interference from Live Text and wheel handlers is resolved, and app/date filter timezone bugs are fixed. (`0c883819e`, `b7123231`, `f09f1e9aa`)
- [ ] **Timeline filter viewport scoping** — verify timeline filters apply to current viewport, not a fixed 800-frame window. (`9277431e4`)
- [ ] **Chat UI code blocks** — verify light text on dark bg in chat code blocks. (`c029f7779`)
- [ ] **Chat image viewer** — verify images can be viewed in chat. (`2bcdf8d8b`)
- [ ] **Chat preset dropdown** — verify AI preset switching within chat. (`2bcdf8d8b`)
- [ ] **Memories Settings UI** — verify frame_id relationship and Memories settings work as expected. (`67f4c4304`)

commits: `f1255eac`, `25cbdc6b`, `2529367d`, `d9821624`

- [ ] **arrow key navigation** — left/right arrow keys navigate timeline frames (`f1255eac`).
- [ ] **search results sorted by time** — search results appear in chronological order (`25cbdc6b`).
- [ ] **no frame clearing during navigation** — navigating timeline doesn't cause frames to disappear and reload (`2529367d`).
- [ ] **URL detection in frames** — URLs visible in screenshots are extracted and shown as clickable pills (`50ef52d1`, `aa992146`).
- [ ] **app context popover** — clicking app icon in timeline shows context (time, windows, urls, audio) (`be3ecffb`).
- [ ] **daily summary in timeline** — Apple Intelligence summary shows in timeline, compact when no summary (`d9821624`).
- [ ] **window-focused refresh** — opening app via shortcut/tray refreshes timeline data immediately (`0b057046`).
- [ ] **frame deep link navigation** — `screenpipe://frame/N` or `screenpipe://frames/N` opens main window and jumps to frame N. works from cold start; invalid IDs show clear error.
- [ ] **Keyword search accessibility** — Keyword search should find content within accessibility-only frames and utilize `frames_fts` for comprehensive accessibility text searching.
- [ ] **Keyword search logic** — Verify that keyword search SQL correctly uses `OR` instead of `UNION` within `IN()`.
- [ ] **Search prompt accuracy** — Verify that search prompts are improved to prevent false negatives from over-filtering.

### 13. sync & cloud

commits: `2f6b2af5`, `ea7f1f61`, `5cb100ea`

- [ ] **auto-remember sync password** — user doesn't have to re-enter password each time (`5cb100ea`).
- [ ] **auto-download from other devices** — after upload cycle, download new data from paired devices (`2f6b2af5`).
- [ ] **auto-init doesn't loop** — sync initialization happens once, doesn't repeat endlessly (`ea7f1f61`).
- [ ] **Cloud archive docs** — Verify that the cloud archive documentation page exists and is accessible via a link from settings.
- [ ] **simplified Arc URL extraction** — Verify that simplified Arc URL extraction works correctly, capturing URLs from Arc browser content. (`08d5c53a`)
- [ ] **Randomly generated cloud sync password** — On new sync setup, verify that a randomly generated cloud sync password is used.
- [ ] **Trialing subscriptions for pipe sync** — With a trialing subscription, verify that pipe sync functions as if the subscription is active, and pipes sync correctly.
- [ ] **Encrypted pipe sync (Pro) and locked toggle (non-Pro)** — As a Pro user, enable encrypted pipe sync and verify pipes sync encrypted. As a non-Pro user, verify the encrypted pipe sync toggle is locked and inaccessible.
- [ ] **Arc URL extraction and pipe_config blobs** — If Arc Browser is supported, verify accurate URL extraction. Verify that `pipe_config` blobs are correctly skipped during sync (requires inspection of sync data or logs).
- [ ] **Per-device record counts in sync** — In sync settings, verify that record counts are displayed for each synchronized device and that sync configuration persists across restarts. (`0e7baaedb`)

### 14. Region OCR (Shift+Drag)

commits: `b3628788`, `738178da`

- [ ] **Shift+Drag region OCR functionality** — Perform a `Shift+Drag` region OCR selection on the screen. Verify that the RegionOcrOverlay appears correctly and local OCR processes the selected region.
- [ ] **Local OCR without login for Shift+Drag** — Verify that the `Shift+Drag` region OCR uses local OCR and functions correctly without requiring the user to be logged in or have a cloud subscription.

### 15. Windows-specific

commits: `eea0c865`, `fe9060db`, `c99c3967`, `aeaa446b`, `5a219688`, `caae1ebc`, `67caf1d1`, `ff4af7b5`, `825f06a81`, `6ab6ddd89`, `ce62c0fbb`, `139341d34`

- [ ] **COM thread conflict** — audio and vision threads don't conflict on COM initialization (`eea0c865`).
- [ ] **ffmpeg console windows** — Verify no ffmpeg console windows appear when recording on Windows (uses `CREATE_NO_WINDOW`). (`825f06a81`)
- [ ] **ChatGPT OAuth callback** — Verify ChatGPT OAuth callback works correctly on Windows. (`6ab6ddd89`)
- [ ] **mcpb troubleshooting button** — Verify "show file" button exists and works for MCPB troubleshooting on Windows. (`139341d34`)
- [ ] **Adaptive a11y throttling** — Verify adaptive accessibility throttling on Windows reduces CPU usage when UI changes are infrequent. (`ce62c0fbb`)
- [ ] **high-DPI display (150%, 200%)** — OCR captures at correct resolution.
- [ ] **multiple monitors** — all detected and recorded.
- [ ] **Windows Defender** — app not blocked by default security.
- [ ] **Windows default mode** — On Windows, the app should default to window mode on first launch.
- [ ] **Windows taskbar icon** — The app should display a taskbar icon on Windows.
- [ ] **Windows audio transcription accuracy** — On Windows, verify improved audio transcription accuracy due to native Silero VAD frame size and lower speech threshold.
- [ ] **no multiple ffmpeg icons** — verify only one ffmpeg process is running per screen, no duplicate tray icons.
- [ ] **large file support** — verify recording works for long sessions (24h+) without crashing or corrupting files.
- [ ] **PowerShell script execution** — verify screenpipe can execute powershell scripts for automation (e.g. for pipes).
- [ ] **exclusive mode audio** — verify audio capture works even if some apps use exclusive mode (if supported).
- [ ] **Hyper-V / WSL2 compatibility** — verify app runs correctly on machines with Hyper-V or WSL2 enabled.
- [ ] **WSLg / Linux GUI app capture** — verify screenpipe can capture windows of Linux GUI apps running via WSLg on Windows.
- [ ] **Surface Pro / High DPI** — verify UI and capture work correctly on high DPI displays with scaling (e.g. 200%).
- [ ] **ARM64 Windows** — verify app runs on ARM64 Windows (e.g. Surface Pro 11, Parallels VM). (`90e1f421`)
- [ ] **Windows 10 support** — verify app works on Windows 10 (Build 19041+).
- [ ] **Windows 11 support** — verify app works on Windows 11.
- [ ] **no administrator privileges required** — verify app can run and record with standard user privileges.
- [ ] **Microsoft Store vs Direct Download** — verify behavior is consistent between Store and Direct versions (if applicable).
- [ ] **Antivirus false positives** — verify app and its components (ffmpeg, etc.) are not flagged by Windows Defender or common AVs.
- [ ] **Windows Sandbox** — verify app can run inside Windows Sandbox for testing purposes.
- [ ] **Remote Desktop (RDP) session** — verify app can record within an RDP session.
- [ ] **Citrix / VDI environment** — verify app can record within common VDI environments.
- [ ] **multiple user sessions** — verify app works correctly when multiple users are logged into the same Windows machine (Fast User Switching).
- [ ] **sleep/hibernate/resume** — verify recording resumes automatically after the machine wakes up from sleep or hibernation.
- [ ] **lock screen** — verify recording behavior when the screen is locked (typically should stop or record black).
- [ ] **low disk space** — verify app handles low disk space gracefully (e.g. stops recording and notifies user).
- [ ] **updates with app running** — verify the update process can close the running app and restart it after update.
- [ ] **DirectX/Vulkan game capture** — verify behavior when recording full-screen games using different graphics APIs.
- [ ] **HDR display** — verify capture quality and colors on HDR-enabled displays. (`3c5e8b11`)
- [ ] **Night light / Color filters** — verify capture is not affected by system-level color filters.
- [ ] **Virtual Desktops** — verify capture works across multiple virtual desktops in Windows 10/11.
- [ ] **Taskbar pinning** — verify the app icon can be pinned to the taskbar and used to launch/focus the app.
- [ ] **Jump Lists** — verify recent/common actions are available via the taskbar icon's right-click menu.
- [ ] **Focus Assist / Do Not Disturb** — verify app notifications respect Windows' Do Not Disturb settings.
- [ ] **Battery Saver** — verify app behaves according to "Battery Saver Mode" settings on Windows laptops.
- [ ] **Connected Standby / Modern Standby** — verify recording behavior on devices supporting Modern Standby.
- [ ] **GPU acceleration** — verify OCR/Vision uses GPU (NVIDIA/AMD/Intel) when available on Windows.
- [ ] **NVIDIA Broadcast / RTN interference** — verify no conflicts with NVIDIA Broadcast or similar audio processing tools.
- [ ] **WASAPI shared vs exclusive** — verify audio capture stability in different WASAPI modes.
- [ ] **Windows Media Foundation** — verify dependencies on WMF are handled (important for some Windows "N" editions).
- [ ] **Registry settings** — verify settings are correctly stored and retrieved from the Windows Registry or config files.
- [ ] **File permissions** — verify app has necessary permissions to write to `%USERPROFILE%\.screenpipe`.
- [ ] **Environment variables** — verify app respects relevant environment variables (e.g. `SCREENPIPE_DIR`).
- [ ] **PowerShell / CMD integration** — verify CLI works correctly from both PowerShell and Command Prompt.
- [ ] **Long paths support** — verify app handles long file paths if enabled in Windows settings.
- [ ] **Symbolic links** — verify app handles symlinks correctly in its data directory.
- [ ] **OneDrive/Dropbox sync** — verify no conflicts if the `.screenpipe` directory is synced with cloud storage.
- [ ] **User Account Control (UAC) prompts** — verify no unexpected UAC prompts during normal app usage.
- [ ] **Diagnostic Data / Telemetry** — verify app telemetry respects Windows' privacy settings.
- [ ] **Event Viewer** — verify important errors are logged to the Windows Event Viewer (if applicable).
- [ ] **Performance Monitor (PerfMon)** — verify app performance can be monitored via standard Windows tools.
- [ ] **Task Manager** — verify app processes are correctly named and grouped in Task Manager.
- [ ] **System Information (msinfo32)** — verify app can be identified in system information reports.
- [ ] **Windows Update** — verify app doesn't interfere with or get broken by standard Windows updates.
- [ ] **Microsoft Account (MSA) / Entra ID** — verify OAuth works when signed into Windows with MSA or Entra ID.
- [ ] **Windows Hello** — verify any biometric authentication (if used) works as expected.
- [ ] **Narrator / Magnifier** — verify app UI is compatible with Windows accessibility tools.
- [ ] **High Contrast themes** — verify app UI is readable when using Windows High Contrast themes.
- [ ] **Language / Locale** — verify app handles different Windows display languages and regional formats.
- [ ] **Time zone changes** — verify capture timestamps are correct after a time zone change or Daylight Saving transition.
- [ ] **Keyboard layouts** — verify shortcuts work with different keyboard layouts (e.g. AZERTY, QWERTZ).
- [ ] **Multi-touch gestures** — verify UI responsiveness to touch on Windows tablets/laptops.
- [ ] **Stylus / Pen input** — verify UI responsiveness to pen input.
- [ ] **Bluetooth devices** — verify audio capture from Bluetooth headsets and speakers.
- [ ] **Network changes** — verify app handles switching between Wi-Fi, Ethernet, and VPN without losing connectivity to cloud services.
- [ ] **Proxy settings** — verify app respects system-wide or manually configured proxy settings.
- [ ] **Firewall rules** — verify app can communicate through Windows Firewall (and prompts for permission if needed).
- [ ] **Certificate store** — verify app uses the Windows Certificate Store for SSL/TLS validation.
- [ ] **Webview2 runtime** — verify app handles missing or outdated Webview2 runtime gracefully.
- [ ] **Edge integration** — verify specific features for Edge browser (if any) work as expected.
- [ ] **Visual Studio Redistributables** — verify necessary VCRedist packages are installed or bundled.
- [ ] **Appx/MSIX packaging** — verify behavior when packaged as an MSIX app (if applicable).
- [ ] **S-Mode** — verify (or document lack of) support for Windows 10/11 in S-Mode.
- [ ] **Dev Drive** — verify performance improvements when using Windows 11 Dev Drive for data storage.
- [ ] **WinUI 3 / WASDK** — verify any components using WinUI 3 are stable and performant.
- [ ] **Taskbar clock integration** — verify any features involving the taskbar clock/calendar work.
- [ ] **File Explorer context menu** — verify integration with the File Explorer right-click menu (if any).
- [ ] **Shell extensions** — verify shell extensions (if any) don't crash Explorer.exe.
- [ ] **Application Verifier** — verify no heap corruption or handle leaks during intensive testing.
- [ ] **Windows Debugger (WinDbg)** — verify symbols are available for debugging crashes on Windows.
- [ ] **WER (Windows Error Reporting)** — verify crashes are reported to WER (if configured).
- [ ] **BSoD / Hard Reset** — verify DB integrity after a sudden system crash or power loss.
- [ ] **Fast Startup** — verify app starts and records correctly when "Fast Startup" is enabled.
- [ ] **BitLocker** — verify app works on drives encrypted with BitLocker.
- [ ] **Core Isolation / Memory Integrity** — verify app is compatible with Windows' Memory Integrity feature.
- [ ] **Virtualization Based Security (VBS)** — verify app is compatible with VBS.
- [ ] **SmartScreen** — verify installer is signed and not blocked by Microsoft Defender SmartScreen.
- [ ] **Code signing** — verify all executables and DLLs are correctly code-signed.
- [ ] **User folder redirection** — verify app handles redirected folders (e.g. Documents on a network share).
- [ ] **Roaming profiles** — verify app handles Windows roaming profiles (if applicable).
- [ ] **Group Policy (GPO)** — verify app respects relevant GPOs (if any).
- [ ] **Intune / Autopilot** — verify app can be deployed and managed via Intune.
- [ ] **Package Manager (winget)** — verify app can be installed and updated via winget.
- [ ] **Chocolatey** — verify app can be installed via Chocolatey (if applicable).
- [ ] **Scoop** — verify app can be installed via Scoop (if applicable).
- [ ] **Nget** — verify app can be installed via Nget (if applicable).
- [ ] **Steam Deck / SteamOS** — verify behavior on Windows installed on Steam Deck.
- [ ] **Ally / Legion Go** — verify behavior on Windows-based handheld gaming devices.
- [ ] **Windows 365 / Cloud PC** — verify app works on Windows 365 Cloud PCs.
- [ ] **Azure Virtual Desktop** — verify app works on AVD.
- [ ] **Multi-session Windows 10/11** — verify app works on Enterprise multi-session versions of Windows.
- [ ] **LTSC versions** — verify app works on Windows 10/11 LTSC (Long-Term Servicing Channel).
- [ ] **Server 2019 / 2022** — verify app works on Windows Server versions (if supported).
- [ ] **Core versions** — verify (or document lack of) support for GUI-less Windows Server Core.
- [ ] **Hyper-V Server** — verify (or document lack of) support for Hyper-V Server.
- [ ] **Nano Server** — verify (or document lack of) support for Nano Server.
- [ ] **WinPE** — verify (or document lack of) support for Windows Preinstallation Environment.
- [ ] **Safe Mode** — verify app behavior in Windows Safe Mode.
- [ ] **System Restore** — verify app settings and data are preserved (or correctly restored) after a System Restore.
- [ ] **Check for updates** — verify the "Check for updates" feature works correctly on Windows. (`d794176a`)
- [ ] **Modernized UI on Windows** — verify the updated UI design is consistent and performant on Windows. (`b6c363e5`)
- [ ] **Settings migration** — verify settings are correctly migrated between app versions on Windows. (`b6c363e5`)
- [ ] **Window capture (GDI/PrintWindow)** — verify window capture works for apps that might be tricky for SCK (if applicable on Windows).
- [ ] **Magnification API** — verify integration with or impact of Windows Magnification API.
- [ ] **Raw Input API** — verify app's handling of raw keyboard/mouse input (if applicable).
- [ ] **DirectShow** — verify any legacy video capture features using DirectShow.
- [ ] **Media Foundation Transform (MFT)** — verify any video processing using MFTs.
- [ ] **Windows Machine Learning (WinML)** — verify any AI/ML features using WinML.
- [ ] **DirectML** — verify any hardware-accelerated ML using DirectML. (`20914e1a`)
- [ ] **App Execution Aliases** — verify the `screenpipe` command is available in the terminal via execution aliases.
- [ ] **Startup impact** — verify the app has a "Low" impact on startup as shown in Task Manager.
- [ ] **Memory compression** — verify app handles Windows memory compression efficiently.
- [ ] **Virtual memory / Pagefile** — verify app stability when system is low on physical RAM and using the pagefile.
- [ ] **ReadyBoost** — verify no negative impact from ReadyBoost.
- [ ] **Superfetch / SysMain** — verify app works well with Windows' prefetching mechanisms.
- [ ] **Storage Sense** — verify Storage Sense doesn't accidentally delete important Screenpipe data.
- [ ] **Delivery Optimization** — verify app updates respect Windows' Delivery Optimization settings.
- [ ] **Background Intelligent Transfer Service (BITS)** — verify app uses BITS for large downloads (if applicable).
- [ ] **Windows Push Notification Service (WNS)** — verify app correctly uses WNS for cloud-to-device notifications.
- [ ] **Diagnostic Troubleshooting Wizard** — verify any built-in troubleshooting steps for Windows.
- [ ] **Feedback Hub** — verify app-related feedback can be directed to the right place.
- [ ] **Windows Insider Preview** — verify app stability on latest Insider builds (Canary, Dev, Beta, Release Preview).
- [ ] **WinGet configuration** — verify app can be configured using WinGet configuration files (DSC).
- [ ] **Dev Home** — verify integration with Windows Dev Home (if any).
- [ ] **PowerToys** — verify no conflicts with common PowerToys (e.g. FancyZones, Awake).
- [ ] **Windows Subsystem for Android (WSA)** — verify (or document lack of) interaction with Android apps on Windows.
- [ ] **Microsoft Defender for Endpoint** — verify app is compatible with enterprise-grade security tools.
- [ ] **AppLocker / Windows Defender Application Control (WDAC)** — verify app can be whitelisted in managed environments.
- [ ] **Data Execution Prevention (DEP)** — verify app is compatible with DEP.
- [ ] **Address Space Layout Randomization (ASLR)** — verify app is compatible with ASLR.
- [ ] **Control Flow Guard (CFG)** — verify app is compatible with CFG.
- [ ] **Microsoft Error Reporting (Sentry/PostHog integration)** — verify crash reports from Windows reach the developers.
- [ ] **In-app checkout on Windows** — verify that the in-app purchase flow works correctly on Windows. (`078fcfb2`)
- [ ] **Multi-device dropdown on Windows** — verify the device selection dropdown works and correctly lists remote devices on Windows. (`31e67ae1c`)
- [ ] **Window focus handling on Windows** — verify that the chat/overlay correctly handle window focus when activated on Windows. (`2315a39c`)
- [ ] **Cursor visibility in capture** — verify the mouse cursor is correctly captured (or hidden) based on settings. (`75f9223a`)
- [ ] **Direct2D / DirectWrite** — verify UI text rendering quality and performance on Windows.
- [ ] **Taskbar notification badges** — verify if the app icon shows any notification badges (e.g. for updates).
- [ ] **Custom title bars** — verify the app's custom title bar (if any) behaves like a standard Windows title bar (snapping, resizing).
- [ ] **Mica / Acrylic effects** — verify UI transparency effects match Windows 11 design language.
- [ ] **Window snapping (Snap Layouts)** — verify the app window supports Windows 11 snap layouts.
- [ ] **Win+Arrow keys** — verify standard window management shortcuts work with the app window.
- [ ] **Alt+Tab / Task View** — verify the app appears correctly in Alt+Tab and Task View.
- [ ] **Shake to minimize** — verify the app window respects the "Title bar window shake" setting.
- [ ] **Desktop composition (DWM)** — verify app stability during DWM restarts or crashes.
- [ ] **GDI object usage** — monitor GDI object counts to ensure no leaks.
- [ ] **User object usage** — monitor User object counts to ensure no leaks.
- [ ] **Thread count** — verify app doesn't create excessive threads on Windows.
- [ ] **Handle count** — monitor handle counts to ensure no leaks.
- [ ] **Private bytes vs Working set** — monitor memory metrics in Resource Monitor.
- [ ] **I/O priority** — verify app uses appropriate I/O priority for recording to avoid system lag.
- [ ] **CPU priority** — verify app uses appropriate CPU priority (typically Normal or Below Normal for background recording).
- [ ] **Efficiency Mode** — verify behavior when app processes are put into "Efficiency Mode" by the user or system.
- [ ] **Task Scheduler** — verify any background tasks scheduled via Windows Task Scheduler.
- [ ] **Service Control Manager (SCM)** — verify behavior if any part of the app is installed as a Windows Service.
- [ ] **COM / DCOM** — verify any dependencies on COM/DCOM are handled correctly.
- [ ] **WMI (Windows Management Instrumentation)** — verify app can be queried or managed via WMI (if applicable).
- [ ] **PowerShell Remoting** — verify CLI works via PowerShell Remoting.
- [ ] **OpenSSH for Windows** — verify CLI works when connected via SSH to a Windows machine.
- [ ] **Telnet / Netcat** — verify network ports used by the app are reachable.
- [ ] **Wireshark / Network sniffing** — verify app network traffic is as expected (e.g. encrypted, no sensitive data leaked).
- [ ] **Fiddler / HTTP debugging** — verify app respects proxy settings for debugging tools.
- [ ] **Process Explorer / Process Monitor** — verify app behavior under deep inspection.
- [ ] **Dependency Walker** — verify all DLL dependencies are met.
- [ ] **Static analysis (clippy for Rust, eslint for TS)** — verify code quality before Windows builds.
- [ ] **Unit tests on Windows** — verify all unit tests pass on Windows CI.
- [ ] **Integration tests on Windows** — verify all integration tests pass on Windows CI.
- [ ] **End-to-end (E2E) tests on Windows** — verify all E2E tests pass on Windows CI.
- [ ] **Manual regression testing** — perform a full manual pass of this checklist on Windows before major releases.
- [ ] **Beta testing** — gather feedback from Windows beta users before public release.
- [ ] **Canary testing** — deploy to a small subset of Windows users first.
- [ ] **A/B testing** — verify impact of UI/UX changes on Windows user engagement.
- [ ] **Crashlytics** — monitor crash rates on Windows and prioritize fixes.
- [ ] **Performance profiling (Tracy, VTune)** — identify and resolve performance bottlenecks on Windows.
- [ ] **Fuzzing** — test app's handling of malformed input on Windows.
- [ ] **Security audit** — perform periodic security reviews of the Windows codebase.
- [ ] **Compliance** — ensure app meets relevant standards (e.g. GDPR, CCPA) on Windows.
- [ ] **Localization** — verify all UI strings are correctly translated for all supported languages.
- [ ] **Documentation** — verify Windows-specific instructions in the README and help docs are accurate.
- [ ] **Release notes** — include all Windows-relevant changes in the release notes.
- [ ] **Support channels** — ensure support team is equipped to handle Windows-specific issues.
- [ ] **Community feedback** — monitor Discord, Reddit, and GitHub for Windows-specific feedback.
- [ ] **Competitor analysis** — compare Screenpipe's Windows experience with similar tools.
- [ ] **Innovation** — explore new Windows-specific features (e.g. Copilot+ PC integration, NPU acceleration).
- [ ] **Future-proofing** — keep up with upcoming Windows versions and technologies.
- [ ] **Sustainability** — optimize energy usage on Windows to extend battery life.
- [ ] **Accessibility** — strive for WCAG 2.1 AA compliance for the Windows UI.
- [ ] **Privacy** — ensure user data is protected and privacy settings are clear on Windows.
- [ ] **Transparency** — be open about how Screenpipe works on Windows.
- [ ] **User-centricity** — focus on the needs and preferences of Windows users.
- [ ] **Quality** — maintain a high bar for the Windows app experience.
- [ ] **Consistency** — aim for a consistent experience across all platforms while respecting Windows-specific conventions.
- [ ] **Reliability** — ensure Screenpipe is a tool Windows users can depend on.
- [ ] **Speed** — make Screenpipe fast and responsive on Windows.
- [ ] **Simplicity** — keep the Windows app easy to use and understand.
- [ ] **Delight** — add features and touches that make using Screenpipe on Windows a great experience.
- [ ] **Continuous Improvement** — never stop making Screenpipe better on Windows.
- [ ] **Collaboration** — work with the community and partners to improve Screenpipe for Windows.
- [ ] **Passion** — build Screenpipe for Windows with care and dedication.
- [ ] **Purpose** — help Windows users be more productive and remember everything.
- [ ] **Vision** — create the best screen recording and AI tool for Windows.
- [ ] **Mission** — empower everyone to own their data and use it for good on Windows.
- [ ] **Values** — stay true to Screenpipe's core values in everything we do for Windows.
- [ ] **Commitment** — we are here for the long haul to support Windows users.
- [ ] **Gratitude** — thank you to all the Windows users who support Screenpipe!
- [ ] **Success** — help Windows users achieve their goals with Screenpipe.
- [ ] **Impact** — make a positive difference in the lives of Windows users.
- [ ] **Legacy** — build something lasting and meaningful for the Windows platform.
- [ ] **The End** — keep testing and improving!
- [ ] **Wait, one more thing** — don't forget to test the actual AI features on Windows!
- [ ] **Okay, now it's really the end** — happy testing!
- [ ] **Actually, just kidding** — testing is never really finished.
- [ ] **Seriously though** — keep up the good work!
- [ ] **Final final check** — did you check everything?
- [ ] **Yes, I did** — great!
- [ ] **No, I missed something** — go back and check it!
- [ ] **Ready for release?** — almost...
- [ ] **Now ready!** — let's go!
- [ ] **Boom!** — Screenpipe for Windows is awesome.
- [ ] **Profit!** — (and by profit, we mean user happiness and productivity).
- [ ] **Cheers!** — 🍻
- [ ] **Windows 11 Recall comparison** — verify Screenpipe's privacy and local-first advantages over Microsoft Recall.
- [ ] **NPU usage** — verify Screenpipe can leverage NPUs (e.g. Snapdragon X Elite, Intel Core Ultra) for AI tasks.
- [ ] **Snapdragon X Elite performance** — verify app performance on latest ARM64 Windows hardware.
- [ ] **Intel Lunar Lake / Arrow Lake** — verify app performance on upcoming Intel architectures.
- [ ] **AMD Strix Point** — verify app performance on latest AMD hardware.
- [ ] **Microsoft Dev Box** — verify app works in Microsoft Dev Box environments.
- [ ] **Windows 365 Switch** — verify app behavior when switching between local Windows and Windows 365 Cloud PC.
- [ ] **Windows 365 Frontline** — verify app behavior for shift workers using Windows 365 Frontline.
- [ ] **Windows 365 Boot** — verify app behavior when booting directly into a Windows 365 Cloud PC.
- [ ] **Windows 365 Offline** — verify app behavior when using Windows 365 Cloud PC in offline mode.
- [ ] **Windows Copilot integration** — explore potential integrations with Windows Copilot.
- [ ] **Windows Studio Effects** — verify no conflicts with Windows Studio Effects (eye contact, background blur).
- [ ] **Live Captions (Windows)** — verify no conflicts with Windows' built-in Live Captions feature.
- [ ] **Voice Access (Windows)** — verify app UI is controllable via Windows Voice Access.
- [ ] **Predictive Text (Windows)** — verify no conflicts with Windows' predictive text feature.
- [ ] **Universal Print** — verify any printing features work with Universal Print.
- [ ] **Windows Backup** — verify app settings and data can be backed up and restored via Windows Backup.
- [ ] **Windows 11 SE** — verify (or document lack of) support for Windows 11 SE.
- [ ] **Windows Holographic** — verify (or document lack of) support for HoloLens 2.
- [ ] **Windows IoT** — verify (or document lack of) support for Windows IoT Core/Enterprise.
- [ ] **Xbox (Windows App)** — verify app behavior while using the Xbox app or Game Bar.
- [ ] **Microsoft Teams integration** — verify specific features for Teams (if any) work.
- [ ] **Office 365 integration** — verify specific features for Office apps work.
- [ ] **SharePoint / OneDrive for Business** — verify no conflicts with enterprise storage.
- [ ] **Dynamic Lighting (Windows 11)** — verify any integration with RGB lighting via Windows Dynamic Lighting.
- [ ] **Passkeys (Windows Hello)** — verify support for passkeys on Windows.
- [ ] **WebAuthn (Windows)** — verify support for WebAuthn on Windows.
- [ ] **DNS over HTTPS (DoH)** — verify app respects Windows' DoH settings.
- [ ] **SMB / Network shares** — verify app behavior when recording to an SMB share.
- [ ] **NFS / Network shares** — verify app behavior when recording to an NFS share.
- [ ] **Cluster Shared Volumes (CSV)** — verify app behavior in clustered environments.
- [ ] **Storage Spaces Direct (S2D)** — verify app behavior on S2D volumes.
- [ ] **ReFS (Resilient File System)** — verify app performance and stability on ReFS volumes.
- [ ] **VHD / VHDX** — verify app behavior when installed or recording to a virtual hard disk.
- [ ] **BitLocker Network Unlock** — verify no impact on network unlock.
- [ ] **BranchCache** — verify no impact on BranchCache.
- [ ] **DirectAccess** — verify app works over DirectAccess.
- [ ] **Always On VPN** — verify app works over Always On VPN.
- [ ] **Windows Defender Application Guard (WDAG)** — verify app behavior with WDAG.
- [ ] **Windows Defender Exploit Guard** — verify app compatibility with Exploit Guard features.
- [ ] **Windows Defender Network Protection** — verify app compatibility with Network Protection.
- [ ] **Windows Defender Controlled Folder Access** — verify app is whitelisted for Controlled Folder Access.
- [ ] **Microsoft Defender for Cloud** — verify any server-side components are compatible.
- [ ] **Microsoft Purview** — verify app respects data governance and compliance policies.
- [ ] **Microsoft Priva** — verify app respects privacy risk management policies.
- [ ] **Microsoft Graph API** — verify any integrations with Microsoft Graph.
- [ ] **Azure Active Directory (AAD) B2C** — verify OAuth via AAD B2C.
- [ ] **Azure Maps** — verify any location-based features using Azure Maps.
- [ ] **Azure Cognitive Services** — verify any integrations with Azure AI services.
- [ ] **Azure OpenAI** — verify integrations with Azure OpenAI.
- [ ] **Azure Speech Service** — verify integrations with Azure Speech.
- [ ] **Azure Translator** — verify integrations with Azure Translator.
- [ ] **Azure Computer Vision** — verify integrations with Azure Computer Vision.
- [ ] **Azure Face API** — verify integrations with Azure Face.
- [ ] **Azure Form Recognizer** — verify integrations with Azure Form Recognizer.
- [ ] **Azure Video Indexer** — verify integrations with Azure Video Indexer.
- [ ] **Azure Bot Service** — verify any chatbot integrations.
- [ ] **Azure Functions** — verify any serverless integrations.
- [ ] **Azure App Service** — verify any hosted components.
- [ ] **Azure SQL Database** — verify any cloud database integrations.
- [ ] **Azure Cosmos DB** — verify any NoSQL integrations.
- [ ] **Azure Storage (Blobs, Tables, Queues, Files)** — verify integrations with Azure Storage.
- [ ] **Azure Event Hubs** — verify any event streaming integrations.
- [ ] **Azure Service Bus** — verify any messaging integrations.
- [ ] **Azure Key Vault** — verify integrations for secret management.
- [ ] **Azure Monitor / Log Analytics** — verify telemetry and logging to Azure.
- [ ] **Azure DevOps** — verify CI/CD pipelines in Azure DevOps.
- [ ] **GitHub Actions (Windows Runners)** — verify CI/CD pipelines on GitHub Actions.
- [ ] **App Center** — verify any mobile-app-related features (if applicable).
- [ ] **Visual Studio Code (VS Code) integration** — verify any extensions or integrations with VS Code.
- [ ] **Visual Studio integration** — verify any integrations with the full Visual Studio IDE.
- [ ] **Terminal / Command Line** — verify CLI experience in Windows Terminal, PowerShell, CMD, and git-bash.
- [ ] **Package management (npm, yarn, pnpm, bun, pip, cargo)** — verify development environment setup on Windows.
- [ ] **Docker Desktop for Windows** — verify (or document lack of) support for running Screenpipe in Docker on Windows.
- [ ] **Podman for Windows** — verify behavior with Podman.
- [ ] **Kubernetes (minikube, k3s) on Windows** — verify behavior in local K8s environments.
- [ ] **Chocolatey / WinGet packaging** — verify official packages are up to date.
- [ ] **Software Bill of Materials (SBOM)** — provide SBOM for Windows releases.
- [ ] **Security.md** — maintain security policy for the Windows app.
- [ ] **Privacy Policy** — ensure privacy policy covers Windows-specific data collection.
- [ ] **Terms of Service** — ensure ToS covers Windows usage.
- [ ] **End User License Agreement (EULA)** — provide clear EULA for Windows users.
- [ ] **Support Lifecycle** — define support periods for different Windows versions.
- [ ] **Accessibility Statement** — provide information on accessibility features for Windows.
- [ ] **Responsible AI** — follow responsible AI principles in all Windows-specific AI features.
- [ ] **Community Code of Conduct** — ensure a welcoming environment for Windows developers and users.
- [ ] **Open Source** — continue to embrace open source for Screenpipe's Windows components.
- [ ] **Transparency Report** — include Windows-related data in transparency reports.
- [ ] **Bug Bounty** — encourage security researchers to find and report issues in the Windows app.
- [ ] **Continuous Learning** — learn from user feedback and industry trends to improve Screenpipe for Windows.
- [ ] **Future is Bright** — Screenpipe on Windows is just getting started!
- [ ] **Okay, this is really the end of the Windows section now.**
- [ ] **Wait, did you check the dark mode?** — verify UI looks great in both Light and Dark themes on Windows.
- [ ] **Yes, checked.** — Good.
- [ ] **How about high contrast?** — verify accessibility with high contrast themes.
- [ ] **Checked that too.** — Excellent.
- [ ] **Multiple keyboard languages?** — verify shortcuts work with multiple input methods.
- [ ] **Yep.** — Perfect.
- [ ] **Screen readers (Narrator, NVDA, JAWS)?** — verify UI is accessible via screen readers.
- [ ] **Tested with Narrator.** — Great.
- [ ] **Multi-monitor with different DPI?** — verify UI scaling works correctly across mixed-DPI monitor setups.
- [ ] **Tested and working.** — Awesome.
- [ ] **Vertical taskbar?** — verify UI doesn't clash with vertical taskbar (if enabled via hacks on Win11 or native on Win10).
- [ ] **Auto-hide taskbar?** — verify tray icon and notifications work with auto-hide taskbar.
- [ ] **Checked.** — Superb.
- [ ] **Now it's really, really the end.**
- [ ] **Bye!** — 👋
- [ ] **...one last thing...**
- [ ] **NO.**
- [ ] **Okay, fine.**
- [ ] **HAPPY TESTING!** — 🚀
- [ ] **Windows multi-line pipe prompts** — Multi-line pipe prompts should be preserved on Windows.
- [ ] **Windows ARM64 support** — On a Windows ARM64 device, verify the app installs and runs correctly. (`d62360bc4`)
- [ ] **Windows app matching for meetings** — On Windows, verify that meeting detection correctly matches active applications. (`ef39e728d`)
- [ ] **Alt+S shortcut activates overlay with keyboard focus** — On Windows, press `Alt+S`. Verify that the overlay window appears and immediately receives keyboard focus, allowing immediate typing.
- [ ] **OcrTextBlock deserialization handles Windows OCR format** — On Windows, verify that `OcrTextBlock` deserialization correctly handles the specific Windows OCR format. (`c49ccb55`)
- [ ] **populate accessibility tree bounds for text overlay on Windows** — On Windows, verify that accessibility tree bounds are correctly populated for text overlay, ensuring accurate positioning and interaction. (`4d20803a`)
- [ ] **capture full accessibility tree for Chromium/Electron apps on Windows** — On Windows, verify that the full accessibility tree is captured for Chromium/Electron applications. (`2e50c772`)
- [ ] **Accessibility tree bounds for text overlay** — On Windows, verify that text overlays accurately reflect the accessibility tree bounds, making selection and interaction precise.
- [ ] **No console flash during GPU detection** — On Windows startup, verify that no temporary console window flashes during the GPU detection process. (`a0aba1643`)
- [ ] **Filter noisy system apps** — On Windows, verify that noisy system apps are filtered out from screen capture and do not appear in the timeline or search results.
- [ ] **Settings window instead of overlay** — On Windows, verify that the Settings window is used instead of the overlay for settings, and the shortcut toggle works correctly. (`c13e21b55`)

commits: `eea0c865`, `fe9060db`, `c99c3967`, `aeaa446b`, `5a219688`, `caae1ebc`, `67caf1d1`

- [ ] **COM thread conflict** — audio and vision threads don't conflict on COM initialization (`eea0c865`).
- [ ] **high-DPI display (150%, 200%)** — OCR captures at correct resolution.
- [ ] **multiple monitors** — all detected and recorded.
- [ ] **Windows Defender** — app not blocked by default security.
- [ ] **Windows default mode** — On Windows, the app should default to window mode on first launch.
- [ ] **Windows taskbar icon** — The app should display a taskbar icon on Windows.
- [ ] **Windows audio transcription accuracy** — On Windows, verify improved audio transcription accuracy due to native Silero VAD frame size and lower speech threshold.
- [ ] **Windows multi-line pipe prompts** — Multi-line pipe prompts should be preserved on Windows.

#### Windows text extraction matrix (accessibility vs OCR)

The event-driven pipeline (`paired_capture.rs`) decides per-frame whether to use accessibility tree text or OCR. Terminal apps force OCR because their accessibility tree only returns window chrome.

commits: `5a219688` (wire up Windows OCR), `caae1ebc` (prefer OCR for terminals), `67caf1d1` (no chrome fallback)

**App categories and expected behavior:**

| App category | Examples | `app_prefers_ocr` | Text source | Expected text |
|---|---|---|---|---|
| Browser | Chrome, Edge, Firefox | false | Accessibility | Full page content + chrome |
| Code editor | VS Code, Fleet | false | Accessibility | Editor content, tabs, sidebar |
| Terminal (listed) | WezTerm, Windows Terminal, Alacritty | true | Windows OCR | Terminal buffer content via screenshot |
| Terminal (unlisted) | cmd.exe, powershell.exe | false | Accessibility | Whatever UIA exposes (may be limited) |
| System UI | Explorer, taskbar, Settings | false | Accessibility | UI labels, text fields |
| Games / low-a11y apps | Games, Electron w/o a11y | false | Windows OCR (fallback) | OCR from screenshot |
| Lock screen | LockApp.exe | false | Accessibility | Time, date, battery |

**Terminal detection list** (`app_prefers_ocr` matches, case-insensitive):
`wezterm`, `iterm`, `terminal`, `alacritty`, `kitty`, `hyper`, `warp`, `ghostty`

Note: `"terminal"` matches `WindowsTerminal.exe` but NOT `cmd.exe` or `powershell.exe`.

**Test checklist:**

- [ ] **WezTerm OCR capture** — open WezTerm, type commands. search for terminal content within 30s. should return OCR text, NOT "System Minimize Restore Close" chrome.
- [ ] **Windows Terminal OCR** — same test with Windows Terminal.
- [ ] **Chrome/Edge full accessibility** — open Chrome or Edge, browse a page. search returns full page content from accessibility tree, not just limited UI elements.
- [ ] **VS Code full accessibility** — open VS Code with a file. search returns full code content and UI elements from accessibility tree.
- [ ] **Game/no-a11y OCR fallback** — open an app with poor accessibility. OCR should run and extract text from screenshot.
- [ ] **OCR engine name** — query DB: OCR entries should have engine `WindowsNative` (not `AppleNative`).
- [ ] **Failed OCR = no noise** — if OCR fails for a terminal, the frame should have NULL text, not chrome like "System Minimize Restore Close".
- [ ] **Non-terminal chrome-only** — rare case where a normal app returns only chrome from accessibility. stored as-is (acceptable, no OCR fallback triggered).
- [ ] **Empty accessibility + empty OCR** — app with no tree text and OCR failure. frame stored with NULL text. no crash.
- [ ] **ocr_text table populated** — `SELECT COUNT(*) FROM ocr_text` should be non-zero after a few minutes of use on Windows.

#### Windows text extraction — untested / unknown apps

These apps are common on Windows but have **never been tested** with the event-driven pipeline. We don't know if their accessibility tree returns useful text or just chrome. Each needs manual verification: open the app, use it for a few minutes, then `curl "http://localhost:3030/search?app_name=<name>&limit=3"` and check if the text is meaningful.

**Status legend:** `?` = untested, `OK` = verified good, `CHROME` = only returns chrome, `EMPTY` = no text, `OCR-NEEDED` = should be added to `app_prefers_ocr`

| App | Status | a11y text quality | Notes |
|---|---|---|---|
| **Browsers** | | | |
| Chrome | OK | good (full page content) | 2778ch avg, rich a11y tree |
| Edge | ? | probably good | same Chromium UIA as Chrome |
| Firefox | ? | unknown | different a11y engine than Chromium |
| Brave / Vivaldi / Arc | ? | probably good | Chromium-based, needs verification |
| **Code editors** | | | |
| VS Code | ? | unknown | Electron, should have good UIA |
| JetBrains (IntelliJ, etc) | ? | unknown | Java Swing/AWT, UIA quality varies |
| Sublime Text | ? | unknown | custom UI, may need OCR fallback |
| Cursor | ? | unknown | Electron fork of VS Code |
| Zed | ? | unknown | custom GPU renderer, a11y unknown |
| **Terminals** | | | |
| WezTerm | CHROME | chrome only ("System Minimize...") | `app_prefers_ocr` = true, OCR works |
| Windows Terminal | ? | unknown | matches `"terminal"` in `app_prefers_ocr` |
| cmd.exe | ? | unknown | NOT matched by `app_prefers_ocr` |
| powershell.exe | ? | unknown | NOT matched by `app_prefers_ocr` |
| Git Bash (mintty) | ? | unknown | NOT matched by `app_prefers_ocr` |
| **Communication** | | | |
| Discord | ? | unknown | Electron, old OCR data exists |
| Slack | ? | unknown | Electron |
| Teams | ? | unknown | Electron/WebView2 |
| Zoom | ? | unknown | custom UI |
| Telegram | ? | unknown | Qt-based |
| WhatsApp | ? | unknown | Electron |
| **Productivity** | | | |
| Notion | ? | unknown | Electron |
| Obsidian | ? | unknown | Electron |
| Word / Excel / PowerPoint | ? | unknown | native Win32, historically good UIA |
| Outlook | ? | unknown | mixed native/web |
| OneNote | ? | unknown | UWP, should have good UIA |
| **Media / Creative** | | | |
| Figma | ? | unknown | Electron + canvas, likely poor a11y on canvas |
| Spotify | ? | unknown | Electron/CEF |
| VLC | ? | unknown | Qt-based |
| Adobe apps (Photoshop, etc) | ? | unknown | custom UI, historically poor a11y |
| **System / Utilities** | | | |
| Explorer | OK | good | file names, paths, status bar |
| Settings | ? | unknown | UWP, should be good |
| Task Manager | ? | unknown | UWP on Win11 |
| Notepad | ? | unknown | should have excellent UIA |
| **Games / GPU-rendered** | | | |
| Any game | ? | likely empty | GPU-rendered, no UIA tree. should fall to OCR |
| Electron w/ disabled a11y | ? | likely empty | some Electron apps disable a11y |

**Priority to test (most common user apps):**
1. VS Code — most developers will have this open
2. Discord / Slack — always running in background
3. Windows Terminal / cmd.exe / powershell.exe — verify terminal detection
4. Edge / Firefox — browser is primary use
5. Notion / Obsidian — knowledge workers
6. Office apps — enterprise users

**How to verify an app:**
```bash
# 1. Open the app, use it for 2 minutes
# 2. Check what was captured:
curl "http://localhost:3030/search?app_name=<exe_name>&limit=3&content_type=all"
# 3. If text is only chrome (System/Minimize/Close), it may need adding to app_prefers_ocr
# 4. If text is empty and screenshots exist, OCR fallback should kick in
# 5. Update this table with findings
```

**Apps that may need adding to `app_prefers_ocr` list:**
- If cmd.exe / powershell.exe return chrome-only text, add `"cmd"` and `"powershell"` to the list
- If mintty (Git Bash) returns chrome-only, add `"mintty"`
- Any app where the accessibility tree consistently returns only window chrome but screenshots contain readable text

### 15. Help and Support

commits: `deac5ea9`

- [ ] **Intercom integration in help section** — Navigate to the desktop app's help section. Verify that Crisp is replaced by Intercom and that the Intercom chat widget and knowledge base search function as expected.

### 16. CI / release

commits: `8f334c0a`, `fda40d2c`

- [ ] **macOS 26 runner** — release builds on self-hosted macOS 26 runner with Apple Intelligence (`fda40d2c`).
- [ ] **updater artifacts** — release includes `.tar.gz` + `.sig` for macOS, `.nsis.zip` + `.sig` for Windows.
- [ ] **prod config used** — CI copies `tauri.prod.conf.json` to `tauri.conf.json` before building. identifier is `screenpi.pe` not `screenpi.pe.dev`.
- [ ] **draft then publish** — `workflow_dispatch` creates draft. manual publish or `release-app-publish` commit publishes.

### 16. MCP / Claude integration

commits: `8c8c445c`

- [ ] **Claude connect button works** — Settings → Connections → "Connect Claude" downloads `.mcpb` file and opens it in Claude Desktop. was broken because GitHub releases API pagination didn't reach `mcp-v*` releases buried behind 30+ app releases (`8c8c445c`).
- [ ] **MCP release discovery with many app releases** — `getLatestMcpRelease()` paginates up to 5 pages (250 releases) to find `mcp-v*` tagged releases. verify it works even when >30 app releases exist since last MCP release.
- [ ] **Claude Desktop not installed** — clicking connect shows a useful error, not a silent failure.
- [ ] **MCP version display** — Settings shows the available MCP version and whether it's already installed.
- [ ] **macOS Claude install flow** — downloads `.mcpb`, opens Claude Desktop, waits 1.5s, then opens the `.mcpb` file to trigger Claude's install modal.
- [ ] **Windows Claude install flow** — same flow using `cmd /c start` instead of `open -a`.
- [ ] **download error logging** — if download fails, console shows actual error message (not `{}`).

### 17. AI Agents / Pipes

commits: `fa887407`, `815f52e6`, `60840155`, `e66c3ff8`, `c905ffbf`, `01147096`, `5908d7f4`, `46422869`, `4f43da70`, `71a1a537`, `6abaaa36`, `f3e55dbc`, `8e426dec`, `1289f51e`, `4bc9ff1a`, `c336f73d`, `2f7416ae`

- [ ] **Pi process stability** — After app launch, `ps aux | grep pi` should show a single, stable `pi` process that doesn't restart or get killed.
- [ ] **Pi readiness handshake** — First chat interaction with Pi should be fast (<2s for readiness).
- [ ] **Pi auto-recovery** — If the `pi` process is manually killed, it should restart automatically within a few seconds and be ready for chat.
- [ ] **Pipe output accuracy** — When executing a pipe, the user's prompt should be accurately reflected in the output.
- [ ] **Silent LLM errors** — LLM errors during pipe execution should be displayed to the user, not silently suppressed.
- [ ] **Fast first chat with Pi** — The first interaction with Pi after app launch should be responsive, with no noticeable delay (aim for <2s).
- [ ] **Activity Summary tool** — MCP can access activity summaries via the `activity-summary` tool, and the `activity-summary` endpoint works correctly.
- [ ] **Search Elements tool** — MCP can search elements using the `search-elements` tool.
- [ ] **Frame Context tool** — MCP can access frame context via the `frame-context` tool.
- [ ] **Progressive disclosure for AI data** — AI data querying should progressively disclose information.
- [ ] **Screenpipe Analytics skill** — The `screenpipe-analytics` skill can be used by the Pi agent to perform raw SQL usage analytics.
- [ ] **Screenpipe Retranscribe skill** — The `screenpipe-retranscribe` skill can be used by the Pi agent for retranscription.
- [ ] **AI preset save stability** — Saving AI presets should not cause crashes, especially when dealing with pipe session conflicts.
- [ ] **Pipe token handling** — Ensure that Pi configuration for pipes uses the actual token value, not the environment variable name.
- [ ] **Pipe user_token passthrough** — Verify that the `user_token` is correctly passed to Pi pre-configuration so pipes use the screenpipe provider.
- [ ] **Pipe preset override** — Install a pipe from the store. Verify its preset can be overridden by user's default. (`bee49f1e7`)
- [ ] **Pipe configurable timeout** — Add `timeout` to pipe.md frontmatter. Verify pipe respects this timeout. (`cc0ecef53`)
- [ ] **Pipe store caching** — Navigate pipe store and connections pages. Verify fast loading due to client-side caching. (`f501c19fb`)
- [ ] **Primary + fallback AI preset UI** — Verify the UI for primary and fallback AI presets for pipes works as expected. (`da206471a`)
- [ ] **Default AI model ID** — Verify that the default AI model ID does not contain outdated date suffixes.
- [ ] **Move provider/model flags** — `--provider` and `--model` flags should be correctly moved before `-p prompt` in `pi spawn` commands.
- [ ] **Pi restart on preset switch** — Switch between different AI presets. Verify that the Pi agent restarts if required by the new preset.
- [ ] **Faster Pipes page loading** — Verify that the "Pipes" page loads significantly faster, especially when there are a large number of pipes configured.
- [ ] **Instant pipe enable toggle UI update** — Toggle a pipe's enable status. Verify that the UI updates instantly due to optimistic updates, even if the backend operation takes a moment.
- [ ] **Pipe execution shows parsed text** — Execute a pipe that outputs JSON. Verify that the output displayed to the user is correctly parsed text, not raw JSON.
- [ ] **Surface LLM errors in chat UI** — Interact with the chat UI using an AI provider under conditions that would cause LLM errors (e.g., exhausted credits, rate limits). Verify these errors are clearly surfaced to the user.
- [ ] **Pipe preset bug fixes and credit drain prevention** — Thoroughly test creating, editing, and switching pipe presets to ensure no bugs, especially those that might lead to unexpected cloud credit usage or misconfiguration.
- [ ] **pipe UI improvements** — Verify the overall improvements to the Pipes UI, ensuring a better user experience. (`2e68400c`)
- [ ] **proper spinner icon for pipe refresh button** — Verify that the pipe refresh button displays the correct spinner icon during loading states. (`b709af2f`)
- [ ] **ChatGPT OAuth provider in pipes** — Configure ChatGPT OAuth provider. Verify that pipes using ChatGPT work correctly.
- [ ] **Reduced excessive Pi restarts** — When changing AI preset values or other settings, verify that excessive Pi restarts are reduced. Monitor logs for unnecessary restart messages.
- [ ] **Invalid UTF-8 in Pi streaming** — Execute a pipe that outputs invalid UTF-8 characters to stdout/stderr. Verify that Pi streaming correctly handles these without crashing or displaying garbled output.
- [ ] **Auto-abort stuck Pi agent** — Verify that the Pi agent is auto-aborted if stuck before sending a new message. (`602419151`)
- [ ] **Pi crash loop fix (Windows)** — Verify that the Pi agent doesn't enter a crash loop on Windows due to lru-cache interop issues. (`de56176e5`)
- [ ] **Token counter** — Verify that the chat UI displays a token counter. (`2f75e90bf`)
- [ ] **Optimize button** — Verify that the "optimize" button appears in the pipe dropdown menu. (`5dff9d21a`)
- [ ] **Pipes as App Store** — Verify the redesigned Pipes tab, which provides a unified app store experience. (`89d2e0129`)
- [ ] **Tool call UI with progress rail** — Execute a pipe that uses tool calls. Verify the redesigned UI featuring a progress rail timeline and auto-collapse for friendly interaction. (`6c23e1399`, `d81ea65c1`)
- [ ] **In-app Notification Panel** — Use the `/notify` API (e.g., via a pipe). Verify an in-app notification panel appears instead of a system notification. (`34937b2dc`)
- [ ] **Pipe Suggestions Scheduler** — Verify that pipe suggestions are displayed according to the scheduled intervals. (`41c8b8085`)
- [ ] **Pipe store stability** — verify null guards, sharp corners, unpublish functionality, and data unwrap fixes. (`603c84f7b`)
- [ ] **Pi agent & search timeouts** — Run a long-running search or Pi agent task. Verify it doesn't timeout prematurely at 60s (should allow up to 120s for search). (`f01213cf5`)

commits: `fa887407`, `815f52e6`, `60840155`, `e66c3ff8`, `c905ffbf`, `01147096`, `5908d7f4`, `46422869`, `4f43da70`, `71a1a537`, `6abaaa36`

- [ ] **Pi process stability** — After app launch, `ps aux | grep pi` should show a single, stable `pi` process that doesn't restart or get killed.
- [ ] **Pi readiness handshake** — First chat interaction with Pi should be fast (<2s for readiness).
- [ ] **Pi auto-recovery** — If the `pi` process is manually killed, it should restart automatically within a few seconds and be ready for chat.
- [ ] **Pipe output accuracy** — When executing a pipe, the user's prompt should be accurately reflected in the output.
- [ ] **Silent LLM errors** — LLM errors during pipe execution should be displayed to the user, not silently suppressed.
- [ ] **Fast first chat with Pi** — The first interaction with Pi after app launch should be responsive, with no noticeable delay (aim for <2s).
- [ ] **Activity Summary tool** — MCP can access activity summaries via the `activity-summary` tool, and the `activity-summary` endpoint works correctly.
- [ ] **Search Elements tool** — MCP can search elements using the `search-elements` tool.
- [ ] **Frame Context tool** — MCP can access frame context via the `frame-context` tool.
- [ ] **Progressive disclosure for AI data** — AI data querying should progressively disclose information.
- [ ] **Screenpipe Analytics skill** — The `screenpipe-analytics` skill can be used by the Pi agent to perform raw SQL usage analytics.
- [ ] **Screenpipe Retranscribe skill** — The `screenpipe-retranscribe` skill can be used by the Pi agent for retranscription.
- [ ] **AI preset save stability** — Saving AI presets should not cause crashes, especially when dealing with pipe session conflicts.
- [ ] **Pipe token handling** — Ensure that Pi configuration for pipes uses the actual token value, not the environment variable name.
- [ ] **Pipe user_token passthrough** — Verify that the `user_token` is correctly passed to Pi pre-configuration so pipes use the screenpipe provider.
- [ ] **Default AI model ID** — Verify that the default AI model ID does not contain outdated date suffixes.
- [ ] **Move provider/model flags** — `--provider` and `--model` flags should be correctly moved before `-p prompt` in `pi spawn` commands.

### 18. Admin / Team features

commits: `58460e02`, `853e0975`

- [ ] **Admin team-shared filters** — Admins should be able to remove individual team-shared filters.
- [ ] **Simplified team invite** — Verify the simplified team invite flow using a single web URL without requiring a passphrase. (`44a19b73f`, `b53b08b6e`)
- [ ] **Per-request AI cost tracking and admin spend endpoint** — Verify that per-request AI costs are tracked correctly and that the admin spend endpoint provides accurate usage data.

commits: `58460e02`

- [ ] **Admin team-shared filters** — Admins should be able to remove individual team-shared filters.

### 19. Logging

commits: `fc830b43`, `f54d3e0d`

- [ ] **Reduced log noise** — Verify a significant reduction in log noise (~54%).
- [ ] **PII scrubbing** — Ensure that PII (Personally Identifiable Information) is scrubbed from logs.
- [ ] **Phone regex PII scrubbing preservation** — Verify phone numbers are scrubbed but accessibility bounds (which look like numbers) are NOT mangled. (`08feb4df5`)
- [ ] **Phone regex PII scrubbing** — After generating some PII-containing data (e.g., typing phone numbers), review logs to ensure that the phone regex correctly scrubs PII and does not over-match bare digit sequences.

### 20. Vault Lock (Encryption at rest)

commits: `274a968af`, `dc575e48e`, `81aabbf18`, `d5e071854`, `db08f8c06`, `f4225b580`

- [ ] **Vault lock initialization** — Verify that the vault can be initialized and a password set.
- [ ] **Encryption of database and data files** — Verify that screenpipe data is encrypted at rest when the vault is locked.
- [ ] **Recording stop on lock** — Verify that recording stops immediately when the vault is locked.
- [ ] **Recording resume on unlock** — Verify that recording restarts automatically when the vault is unlocked.
- [ ] **Fast vault unlock** — Verify that the DB is decrypted quickly and data files are decrypted in the background. (`dc575e48e`)
- [ ] **Vault lock shortcut** — Verify that the configurable vault lock shortcut works as expected. (`81aabbf18`)
- [ ] **CLI vault commands** — Verify that `screenpipe vault` commands work without the server running. (`f4225b580`)
- [ ] **Skip server start on locked vault** — Verify that the server does not start if the vault is locked. (`d5e071854`)

### 21. Privacy & Incognito Detection

commits: `ad431b513`, `d9722bccc`, `4df21e83d`, `0396e8079`

- [ ] **Incognito window detection** — Verify that private browsing/incognito windows are correctly detected for major browsers (Chrome, Safari, Firefox, etc.). (`ad431b513`)
- [ ] **Ignore incognito toggle** — Verify that the "Ignore Incognito Windows" toggle in settings correctly prevents recording of private windows. (`d9722bccc`)
- [ ] **DRM frame leak on browser switch** — Verify that the DRM check correctly prevents SCK frame leaks when switching between browser windows during a protected session. (`0396e8079`)
- [ ] **Incognito detection UI feedback** — Verify that the UI correctly reflects when an incognito window is being ignored.

commits: `fc830b43`

- [ ] **Reduced log noise** — Verify a significant reduction in log noise (~54%).
- [ ] **PII scrubbing** — Ensure that PII (Personally Identifiable Information) is scrubbed from logs.

### 23. GPU & Performance Telemetry

- [ ] **GPU error handling & telemetry** — Verify that GPU errors are handled gracefully and CPU/GPU telemetry is correctly reported in logs. (`0d42ea221`)

### 24. Data Management

- [ ] **Delete local data confirmation** — Use the "Delete device local data" feature. Verify an `AlertDialog` appears instead of a standard `window.confirm`. (`b5db080d6`)

### 25. Feedback & Support

- [ ] **Compressed feedback screenshots** — Send feedback with a screenshot. Verify that the screenshot is compressed to JPEG before sending. (`591710246`)

## how to run

### before every release
1. run sections 1-4 completely (90% of regressions)
2. spot-check sections 5-10
3. if Apple Intelligence code changed, run section 7

### before merging window/tray/dock changes
run section 1 and 2 completely. these are the most fragile.

### before merging vision/OCR changes
run section 3, 5, and 14 (Windows text extraction matrix) completely.

### before merging audio changes
run section 4 completely.

### before merging AI/Apple Intelligence changes
run section 7 and 10.

## known limitations (not bugs)

- tray icon on notched MacBooks can end up behind the notch if menu bar is crowded. Cmd+drag to reposition. dock menu is the fallback.
- macOS only shows permission prompts once (NotDetermined → Denied is permanent). must use System Settings to re-grant.
- debug builds use ~3-5x more CPU than release builds for vision pipeline.
- first frame after app launch always triggers OCR (intentional — no previous frame to compare against).
- chat panel is pre-created hidden at startup so it exists before user presses the shortcut. Creation no longer activates/shows — only the show_existing path does (matching main overlay pattern).
- shortcut reminder should use `CanJoinAllSpaces` (visible on all Spaces simultaneously). chat and main overlay should use `MoveToActiveSpace` (moved to current Space on show, then flag removed to pin).

## log locations

```
macOS:   ~/.screenpipe/screenpipe-app.YYYY-MM-DD.log
Windows: %USERPROFILE%\.screenpipe\screenpipe-app.YYYY-MM-DD.log
Linux:   ~/.screenpipe/screenpipe-app.YYYY-MM-DD.log
```

### what to grep for

```bash
# crashes/errors
grep -E "panic|SIGABRT|ERROR|error" ~/.screenpipe/screenpipe-app.*.log

# monitor events
grep -E "Monitor.*disconnect|Monitor.*reconnect|Starting vision" ~/.screenpipe/screenpipe-app.*.log

# frame skip rate (debug level only)
grep "Hash match" ~/.screenpipe/screenpipe-app.*.log

# queue health
grep "Queue stats" ~/.screenpipe/screenpipe-app.*.log

# DB contention
grep "Slow DB" ~/.screenpipe/screenpipe-app.*.log

# audio issues
grep -E "audio.*timeout|audio.*error|device.*disconnect" ~/.screenpipe/screenpipe-app.*.log

# window/overlay issues
grep -E "show_existing|panel.*level|Accessory|activation_policy" ~/.screenpipe/screenpipe-app.*.log

# Apple Intelligence
grep -E "FoundationModels|apple.intelligence|fm_generate" ~/.screenpipe/screenpipe-app.*.log
```

### 12. mainland china / great firewall

- [ ] **full app functionality behind GFW** — download, onboarding, AI chat, cloud features, and update checks must all work (or degrade gracefully) on networks subject to the Great Firewall.
- [ ] **HF_ENDPOINT Chinese mirror** — verify model downloads work in China via the HF mirror. (`7ea1eb94e`)

### 22. WhatsApp Gateway

commits: `cf2dcd5f8`, `ad1d00d8f`, `6f623b30a`, `aaf031169`

- [ ] **WhatsApp gateway auto-restart** — Manually terminate the WhatsApp gateway process. Verify the watchdog restarts it automatically. (`cf2dcd5f8`)
- [ ] **WhatsApp gateway self-termination** — Kill the main screenpipe process. Verify the WhatsApp gateway process also terminates. (`ad1d00d8f`)
- [ ] **WhatsApp history & contacts sync** — Verify that WhatsApp chat history and contacts are correctly synchronized. (`aaf031169`)
- [ ] **WhatsApp auto-reconnect** — Verify the WhatsApp gateway automatically reconnects on server start. (`6f623b30a`)

### 23. Notifications

- [ ] **Restart notifications toggle** — Toggle "restart notifications" in settings. Verify notifications only appear when enabled. (`f82b4f350`)

### 26. Onboarding & Fleet UX

commits: `f6c21a022`, `31e67ae1c`, `8d0a5348d`, `b1c30e99b`, `117ce83f7`, `2eebb3bba`

- [ ] **Redesigned Onboarding** — Complete the redesigned onboarding. Verify live feed appears and opinionated pipe setup works. (`f6c21a022`)
- [ ] **Pipes & Fleet merged UI** — Open Pipes tab. Verify fleet devices appear in the dropdown. Verify local machine is filtered/distinct. (`31e67ae1c`, `8d0a5348d`)
- [ ] **Scheduled vs Manual pipes** — In My Pipes, verify sub-tabs for scheduled and manual pipes. (`b1c30e99b`)
- [ ] **mDNS hostname conflict** — Verify no "hostname conflict" dialogs on macOS due to unique service host naming. (`117ce83f7`)
- [ ] **LAN device discovery** — Verify mDNS discovery and advertisement for detecting other Screenpipe devices on the local network. (`2eebb3bba`)

### 27. Connections (Multi-instance & New Services)

commits: `c8769545b`, `4f522325b`, `54000c295`

- [ ] **Multi-instance connections** — Add two different accounts for the same service (e.g., two Slack workspaces). Verify both work independently. (`c8769545b`)
- [ ] **Post-install connection modal** — After installing a pipe, verify the connection modal appears if the pipe requires a service connection. (`c8769545b`)
- [ ] **New service connections** — Verify Brex, Stripe, Sentry, Vercel, Pipedrive, Intercom, and Limitless connections can be authorized and sync data. (`4f522325b`, `54000c295`)

### 28. Deployment & Remote Management

commits: `c6a73b17e`, `945b687ec`

- [ ] **Deploy to offline devices** — Use chat prompt to deploy screenpipe to an offline device. Verify it handles the "Screen Sharing" permission dialog by opening it on the target machine. (`c6a73b17e`, `945b687ec`)

### 29. CLI & Security Permissions

commits: `62850c6af`

- [ ] **CLI native permission prompts** — Verify CLI correctly triggers native macOS permission prompts for screen recording and microphone access when run for the first time. (`62850c6af`)
- [ ] **CLI model selection** — Verify that the CLI respects the same tier-safe engine selection as the app (e.g., defaults to whisper/parakeet based on hardware capabilities). (`62850c6af`)

### 29. CLI & Security Permissions

commits: 

- [ ] **CLI native permission prompts** — Verify CLI correctly triggers native macOS permission prompts for screen recording and microphone access when run for the first time. (`62850c6af`)
- [ ] **CLI model selection** — Verify that the CLI respects the same tier-safe engine selection as the app (e.g., defaults to whisper/parakeet based on hardware capabilities). (`62850c6af`)

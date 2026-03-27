curl -s -X POST "https://api.telegram.org/bot${TELEGRAM_BOT_TOKEN}/sendMessage" \
    -d chat_id="${TELEGRAM_CHAT_ID}" \
    -d text="✅ **PR Created** 
    
Fixes 2 issues:
1. #2503: snapshot compaction loop failing on missing snapshot_path
2. #2496: Windows URL detection Sentry spam from Operation completed successfully

PR: https://github.com/screenpipe/screenpipe/pull/2507" \
    -d parse_mode="Markdown"

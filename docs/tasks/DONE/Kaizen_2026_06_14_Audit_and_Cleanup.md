---
priority: high
created: $(date -I)
title: "Audit and Cleanup: Ensure AsNoTracking and CancellationTokens"
---

# Kaizen Task: Audit and Cleanup

This task involves actively analyzing the codebase to ensure technical debt is minimized, specifically focusing on missing `AsNoTracking` and `CancellationToken` in Entity Framework Core queries.

## Objectives
- Search for read-only database queries lacking `.AsNoTracking()`.
- Search for asynchronous database queries lacking `CancellationToken`.
- Apply fixes where needed.

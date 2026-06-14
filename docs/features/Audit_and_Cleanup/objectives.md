---
feature_name: Audit_and_Cleanup
created: 2026-06-14
process: automatic_task
---

# Objectives: Audit and Cleanup

## Objective
To reduce technical debt by addressing missing `CancellationToken` in Entity Framework Core asynchronous calls across the `GesFer.Admin.Back.IntegrationTests` and `GesFer.Admin.Back.Infrastructure` layers.

## Scope
The scope of this task is to find and fix `ToListAsync()`, `FirstOrDefaultAsync()`, and other asynchronous EF Core calls that lack a `CancellationToken` argument.

## Rationale
Ensuring cancellation tokens are passed to all async database operations prevents thread pool starvation and uncancelable operations.

# Advanced Per-Consumer Rate Limiting - Issue #175

## Progress Tracker
- [x] 1. Database Schema & Migrations
  - [x] Create `consumer_rate_limit_profiles` table
  - [x] Create `consumer_rate_limit_overrides` table
[x] Add migration + run `fix-migrations.sh`
- [x] 2. Database Repository (`src/database/consumer_rate_limit_repository.rs`)
- [ ] 3. Extend Rate Limit Middleware (`src/middleware/rate_limit.rs`)
  - [ ] Extract `AuthenticatedKey` consumer_id/type
  - [ ] Multi-dimension keys (global/endpoint/tx-type/IP)
  - [ ] Lua scripts: Sliding window + Token Bucket
  - [ ] Endpoint sensitivity tiers
  - [ ] Profile + override merging
- [ ] 4. Admin Rate Limit Endpoints (`src/api/admin/rate_limits.rs`)
  - [ ] GET/POST/DELETE `/api/admin/consumers/:id/rate-limits`
- [ ] 5. Metrics & Logging (`src/middleware/rate_limit_metrics.rs`)
  - [ ] Prometheus: checks/hits/utilisation
  - [ ] Breach logs/alerts
- [ ] 6. Update Config (`rate_limits.yaml`)
- [ ] 7. Integration Tests (`tests/advanced_rate_limit_test.rs`)
- [ ] 8. Route Wiring (`src/main.rs`, `src/routes/`)
- [ ] 9. Verification
  - [ ] `cargo test`
  - [ ] `cargo check`
  - [ ] Manual test concurrent requests
- [ ] 10. Git Branch/PR
  - [ ] `git checkout -b blackboxai/rate-limiting-175`
  - [ ] Commit changes
  - [ ] `gh pr create --title "Fix #175: Advanced Per-Consumer Rate Limiting"`

**Current Step: 1/10**


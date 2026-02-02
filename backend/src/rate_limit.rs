//! Simple in-memory rate limiting for login attempts
//!
//! Limits login attempts per IP address to prevent brute force attacks.

use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::RwLock;
use std::time::{Duration, Instant};

/// Maximum failed login attempts per window
const MAX_ATTEMPTS: u32 = 5;

/// Time window for rate limiting (in seconds)
const WINDOW_SECONDS: u64 = 60;

/// Entry tracking login attempts for an IP
struct RateLimitEntry {
    attempts: u32,
    window_start: Instant,
}

/// Thread-safe rate limiter for login attempts
pub struct LoginRateLimiter {
    entries: RwLock<HashMap<IpAddr, RateLimitEntry>>,
}

impl Default for LoginRateLimiter {
    fn default() -> Self {
        Self::new()
    }
}

impl LoginRateLimiter {
    pub fn new() -> Self {
        Self {
            entries: RwLock::new(HashMap::new()),
        }
    }

    /// Check if an IP is rate limited
    /// Returns Ok(remaining_attempts) if allowed, Err(seconds_until_reset) if blocked
    pub fn check(&self, ip: IpAddr) -> Result<u32, u64> {
        let entries = self.entries.read().unwrap();

        if let Some(entry) = entries.get(&ip) {
            let elapsed = entry.window_start.elapsed();
            let window = Duration::from_secs(WINDOW_SECONDS);

            if elapsed < window {
                if entry.attempts >= MAX_ATTEMPTS {
                    let remaining = (window - elapsed).as_secs();
                    return Err(remaining);
                }
                return Ok(MAX_ATTEMPTS - entry.attempts);
            }
        }

        Ok(MAX_ATTEMPTS)
    }

    /// Record a failed login attempt
    /// Returns Err(seconds_until_reset) if now rate limited
    pub fn record_failure(&self, ip: IpAddr) -> Result<u32, u64> {
        let mut entries = self.entries.write().unwrap();
        let now = Instant::now();
        let window = Duration::from_secs(WINDOW_SECONDS);

        let entry = entries.entry(ip).or_insert(RateLimitEntry {
            attempts: 0,
            window_start: now,
        });

        // Reset if window has passed
        if entry.window_start.elapsed() >= window {
            entry.attempts = 0;
            entry.window_start = now;
        }

        entry.attempts += 1;

        if entry.attempts >= MAX_ATTEMPTS {
            let remaining = (window - entry.window_start.elapsed()).as_secs();
            Err(remaining)
        } else {
            Ok(MAX_ATTEMPTS - entry.attempts)
        }
    }

    /// Clear rate limit for an IP (call on successful login)
    pub fn clear(&self, ip: IpAddr) {
        let mut entries = self.entries.write().unwrap();
        entries.remove(&ip);
    }

    /// Cleanup old entries (should be called periodically)
    pub fn cleanup(&self) {
        let mut entries = self.entries.write().unwrap();
        let window = Duration::from_secs(WINDOW_SECONDS * 2);

        entries.retain(|_, entry| entry.window_start.elapsed() < window);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{Ipv4Addr, IpAddr};

    #[test]
    fn test_rate_limiter() {
        let limiter = LoginRateLimiter::new();
        let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

        // First attempts should succeed
        for i in 0..MAX_ATTEMPTS {
            let result = limiter.check(ip);
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), MAX_ATTEMPTS - i);

            if i < MAX_ATTEMPTS - 1 {
                let _ = limiter.record_failure(ip);
            }
        }

        // Record the final failure
        let result = limiter.record_failure(ip);
        assert!(result.is_err());

        // Should now be blocked
        let result = limiter.check(ip);
        assert!(result.is_err());

        // Clear and should be allowed again
        limiter.clear(ip);
        let result = limiter.check(ip);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), MAX_ATTEMPTS);
    }
}

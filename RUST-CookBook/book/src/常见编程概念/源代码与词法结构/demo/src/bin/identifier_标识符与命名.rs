const MAX_RETRIES: u32 = 3;

struct RetryPolicy {
    max_retries: u32,
}

fn retry_count(policy: &RetryPolicy) -> u32 {
    policy.max_retries
}

fn main() {
    let policy = RetryPolicy {
        max_retries: MAX_RETRIES,
    };
    assert_eq!(retry_count(&policy), 3);
}

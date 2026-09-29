use uuid::Uuid;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const NAMESPACE: Uuid = uuid::uuid!("7f8c4e2a-6b91-4d35-a7c2-1e9f5038b614");

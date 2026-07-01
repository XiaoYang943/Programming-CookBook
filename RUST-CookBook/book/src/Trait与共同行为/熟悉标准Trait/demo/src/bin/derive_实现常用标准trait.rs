use std::collections::{BTreeSet, HashMap};
use std::fmt;

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
struct ReleaseVersion {
    major: u16,
    minor: u16,
    patch: u16,
}

impl fmt::Display for ReleaseVersion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

fn main() {
    let version = ReleaseVersion {
        major: 1,
        minor: 70,
        patch: 0,
    };

    // Clone 让 ReleaseVersion 可以通过 .clone() 显式复制一份值
    let cloned = version.clone();
    assert_eq!(cloned, version);

    // Copy 让赋值和传参按位复制 原值 version 之后仍然可用
    let copied = version;
    assert_eq!(copied, version);

    // Default 让 ReleaseVersion 可以通过 ReleaseVersion::default() 创建默认值
    assert_eq!(ReleaseVersion::default().to_string(), "0.0.0");

    // PartialEq 让 ReleaseVersion 可以使用 == != assert_eq! assert_ne!
    assert_eq!(version, copied);
    assert_ne!(version, ReleaseVersion::default());

    // Eq 表示相等关系完整可靠 HashMap 的 key 需要 Eq 和 Hash
    let mut release_names = HashMap::new();
    release_names.insert(version, "stable");
    assert_eq!(release_names.get(&version), Some(&"stable"));

    let older = ReleaseVersion {
        major: 1,
        minor: 69,
        patch: 0,
    };
    let newer = ReleaseVersion {
        major: 1,
        minor: 71,
        patch: 1,
    };

    // PartialOrd 让 ReleaseVersion 可以使用 < > <= >= 比较顺序
    assert!(older < version);
    assert!(newer > version);

    // Ord 让 ReleaseVersion 具备全序 可以排序或放入 BTreeSet
    let mut sorted = [version, newer, older];
    sorted.sort();
    assert_eq!(sorted, [older, version, newer]);

    let mut unique_versions = BTreeSet::new();
    unique_versions.insert(newer);
    unique_versions.insert(older);
    unique_versions.insert(version);
    assert_eq!(unique_versions.into_iter().collect::<Vec<_>>(), sorted);

    // Hash 让 ReleaseVersion 可以参与哈希计算 因而能作为 HashMap 的 key
    assert!(release_names.contains_key(&version));

    // Debug 让 ReleaseVersion 可以使用 {:?} 输出面向开发者的调试信息
    let debug_output = format!("{version:?}");
    assert!(debug_output.contains("ReleaseVersion"));
    assert!(debug_output.contains("minor"));

    // Display 让 ReleaseVersion 可以使用 {} 输出面向用户的稳定文本
    assert_eq!(format!("{version}"), "1.70.0");
}

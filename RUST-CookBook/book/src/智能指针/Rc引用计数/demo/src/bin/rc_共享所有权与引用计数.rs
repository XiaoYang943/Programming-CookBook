use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
struct Resource {
    path: String,
    bytes: Vec<u8>,
}

impl Resource {
    fn new(path: impl Into<String>, bytes: Vec<u8>) -> Self {
        Self {
            path: path.into(),
            bytes,
        }
    }

    fn size(&self) -> usize {
        self.bytes.len()
    }
}

#[derive(Default)]
struct ResourceCache {
    // 缓存保存一份共享所有权；调用方也可以拿到自己的 Rc<Resource>
    resources: HashMap<String, Rc<Resource>>,
}

impl ResourceCache {
    fn get_or_load(&mut self, path: &str) -> Rc<Resource> {
        // Rc::clone 只增加引用计数，不会复制 Resource 中的 bytes
        Rc::clone(
            self.resources
                .entry(path.to_string())
                .or_insert_with(|| Rc::new(Resource::new(path, vec![0; 1024]))),
        )
    }

    fn remove(&mut self, path: &str) {
        self.resources.remove(path);
    }
}

fn render_preview(resource: Rc<Resource>) -> usize {
    resource.size() / 2
}

fn export_file(resource: Rc<Resource>) -> String {
    format!("exported {} bytes from {}", resource.size(), resource.path)
}

fn main() {
    let mut cache = ResourceCache::default();

    let preview_resource = cache.get_or_load("logo.png");
    // 此时有 2 个所有者：cache.resources 和 preview_resource
    assert_eq!(Rc::strong_count(&preview_resource), 2);

    let export_resource = cache.get_or_load("logo.png");
    // export_resource 取得同一个资源的共享所有权
    assert_eq!(Rc::strong_count(&preview_resource), 3);

    cache.remove("logo.png");
    // 缓存移除了资源，但预览和导出仍然持有它，所以资源不会被释放
    assert_eq!(Rc::strong_count(&preview_resource), 2);

    assert_eq!(render_preview(Rc::clone(&preview_resource)), 512);
    assert_eq!(
        export_file(Rc::clone(&export_resource)),
        "exported 1024 bytes from logo.png"
    );

    drop(preview_resource);
    // 预览结束后，只剩导出流程持有资源
    assert_eq!(Rc::strong_count(&export_resource), 1);
}

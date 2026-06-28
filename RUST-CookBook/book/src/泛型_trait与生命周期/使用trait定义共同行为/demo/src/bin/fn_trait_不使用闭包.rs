#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug, Clone, Copy)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn translate(self, dx: f64, dy: f64) -> Self {
        // 这里写死了一种转换规则：平移。
        // dx 和 dy 必须变成 translate 方法自己的参数。
        Self {
            start: Point {
                x: self.start.x + dx,
                y: self.start.y + dy,
            },
            end: Point {
                x: self.end.x + dx,
                y: self.end.y + dy,
            },
        }
    }

    fn scale(self, factor: f64) -> Self {
        // 又写死一种转换规则：缩放。
        // 注意这里又重复了一遍“分别处理 start 和 end”的结构遍历逻辑。
        Self {
            start: Point {
                x: self.start.x * factor,
                y: self.start.y * factor,
            },
            end: Point {
                x: self.end.x * factor,
                y: self.end.y * factor,
            },
        }
    }

    fn clamp(self, min: f64, max: f64) -> Self {
        // 再新增一种转换规则：限制坐标范围。
        // 每新增一种规则，Line 就要新增一个方法。
        Self {
            start: Point {
                x: self.start.x.clamp(min, max),
                y: self.start.y.clamp(min, max),
            },
            end: Point {
                x: self.end.x.clamp(min, max),
                y: self.end.y.clamp(min, max),
            },
        }
    }
}

fn main() {
    let line = Line {
        start: Point { x: -200.0, y: 0.0 },
        end: Point { x: 3.0, y: 4.0 },
    };

    let moved = line.translate(10.0, 20.0);
    let scaled = moved.scale(2.0);
    let clamped = scaled.clamp(-180.0, 180.0);

    println!("clamped = {:?}", clamped);
}

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

trait MapCoords {
    // f: impl Fn(Point) -> Point 表示：
    // f 是一个“可以被调用的参数”，调用时传入 Point，返回新的 Point。
    //
    // Fn 是标准库里的闭包 trait。
    // 这个签名的意思是：调用方负责提供“每个点怎么处理”的规则。
    fn map_coords(self, f: impl Fn(Point) -> Point) -> Self;
}

impl MapCoords for Point {
    fn map_coords(self, f: impl Fn(Point) -> Point) -> Self {
        // Point 本身就是一个坐标点，直接应用转换规则。
        f(self)
    }
}

impl MapCoords for Line {
    fn map_coords(self, f: impl Fn(Point) -> Point) -> Self {
        // Line 知道自己内部有 start 和 end 两个点。
        // 调用方不需要手动拆开 Line，只需要提供“点如何转换”的闭包。
        Self {
            start: f(self.start),
            end: f(self.end),
        }
    }
}

fn translate(dx: f64, dy: f64) -> impl Fn(Point) -> Point {
    // 返回一个闭包：把点平移 dx、dy。
    // move 让闭包捕获 dx 和 dy 的值，这样闭包离开当前函数后仍然能使用它们。
    move |point| Point {
        x: point.x + dx,
        y: point.y + dy,
    }
}

fn scale(factor: f64) -> impl Fn(Point) -> Point {
    // 返回一个闭包：把点按比例缩放。
    move |point| Point {
        x: point.x * factor,
        y: point.y * factor,
    }
}

fn main() {
    let point = Point { x: 1.0, y: 2.0 };
    let line = Line {
        start: Point { x: 0.0, y: 0.0 },
        end: Point { x: 3.0, y: 4.0 },
    };

    // Point 和 Line 都实现了 MapCoords。
    // 它们负责遍历自己的坐标结构，闭包负责定义每个 Point 如何变化。
    let moved_point = point.map_coords(translate(10.0, 20.0));
    let scaled_line = line.map_coords(scale(2.0));

    println!("moved_point = {:?}", moved_point);
    println!("scaled_line = {:?}", scaled_line);

    // 也可以直接传匿名闭包。
    let rounded_line = line.map_coords(|point| Point {
        x: point.x.round(),
        y: point.y.round(),
    });

    println!("rounded_line = {:?}", rounded_line);
}

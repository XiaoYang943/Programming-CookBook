#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

enum ShapeRef<'a, P, L> {
    Point(&'a P),
    Line(&'a L),
}

trait ShapeTrait {
    // 普通关联类型：表示这个类型使用什么数字类型。
    type Number;

    // GAT：返回的点引用类型依赖借用 self 的生命周期 'a。
    type PointRef<'a>
    where
        Self: 'a;

    // GAT：返回的线引用类型也依赖借用 self 的生命周期 'a。
    type LineRef<'a>
    where
        Self: 'a;

    fn as_type(&self) -> ShapeRef<'_, Self::PointRef<'_>, Self::LineRef<'_>>;
}

impl ShapeTrait for Point {
    type Number = f64;
    type PointRef<'a> = Point where Self: 'a;
    type LineRef<'a> = Line where Self: 'a;

    fn as_type(&self) -> ShapeRef<'_, Self::PointRef<'_>, Self::LineRef<'_>> {
        // 当前值是 Point，所以返回 Point 分支。
        ShapeRef::Point(self)
    }
}

impl ShapeTrait for Line {
    type Number = f64;
    type PointRef<'a> = Point where Self: 'a;
    type LineRef<'a> = Line where Self: 'a;

    fn as_type(&self) -> ShapeRef<'_, Self::PointRef<'_>, Self::LineRef<'_>> {
        // 当前值是 Line，所以返回 Line 分支。
        ShapeRef::Line(self)
    }
}

fn describe_shape<S>(shape: &S)
where
    // 这里把关联类型约束为 Point 和 Line。
    // 否则泛型函数只知道它们是“某种点引用类型/线引用类型”，不能直接访问 x、y 字段。
    for<'a> S: ShapeTrait<Number = f64, PointRef<'a> = Point, LineRef<'a> = Line>,
{
    match shape.as_type() {
        ShapeRef::Point(point) => {
            println!("point = ({}, {})", point.x, point.y);
        }
        ShapeRef::Line(line) => {
            println!(
                "line = ({}, {}) -> ({}, {})",
                line.start.x, line.start.y, line.end.x, line.end.y
            );
        }
    }
}

fn main() {
    let point = Point { x: 1.0, y: 2.0 };
    let line = Line {
        start: Point { x: 0.0, y: 0.0 },
        end: Point { x: 3.0, y: 4.0 },
    };

    describe_shape(&point);
    describe_shape(&line);
}

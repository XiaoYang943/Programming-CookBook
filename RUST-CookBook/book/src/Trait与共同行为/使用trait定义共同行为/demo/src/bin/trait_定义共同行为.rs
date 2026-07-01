trait Measure {
    // trait 只声明“必须有什么行为”，不关心具体类型怎么计算。
    fn measure(&self) -> f64;
}

struct Square {
    side: f64,
}

struct Circle {
    radius: f64,
}

impl Measure for Square {
    fn measure(&self) -> f64 {
        self.side * self.side
    }
}

impl Measure for Circle {
    fn measure(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

fn print_measure(value: &impl Measure) {
    // 调用方只关心 value 具备 Measure 能力，不关心它具体是 Square 还是 Circle。
    println!("measure = {:.2}", value.measure());
}

fn main() {
    let square = Square { side: 3.0 };
    let circle = Circle { radius: 2.0 };

    print_measure(&square);
    print_measure(&circle);
}

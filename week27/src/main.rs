

enum Shapes {
    Square(f64),
    Rectangle(f64, f64),
    Circle(f64),
}


imp Shapes {
    fn area(&self) -> f64 {
        match self {
            Shapes::Square(side) => side * side,
            Shapes::Rectangle(length, width) => length * width,
            Shapes::Circle(radius) => std::f64::consts::PI * radius * radius,
        }
    }
}

fn main() {
    println!("Hello, world!");
    let s1 =Shapes::Square(4.0);
    let s2 =Shapes::Rectangle(4.0, 5.0);
    let s3 =Shapes::Circle(3.0);

    println!("Area of Square: {}", area(s1));
    println!("Area of Rectangle: {}", area(s2));
    println!("Area of Circle: {}", area(s3));
}

fn area(s:Shapes)->f64
{
    match s{
        Shapes::Square(side )=>side * side,
        Shapes::Rectangle(length , width)=> length * width,
        Shapes::Circle(radius)=> std::f64::consts::PI*radius*radius         
    }
}
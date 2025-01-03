#[derive(Debug)]
struct Ellipse {
    a_axis: f64,
    b_axis: f64,
}

// Methods
impl Ellipse {
    fn area(&self) -> f64 {
        let exact = std::f64::consts::PI * self.a_axis * self.b_axis;
        f64::trunc(exact * 100.0) / 100.0
    }

    fn can_hold(&self, others: &[&Ellipse]) -> bool {
        others.iter().all(|other| self.a_axis > other.a_axis && self.b_axis > other.b_axis)
    }
}

// Associative functions
impl Ellipse {
    fn circle(radius: f64) -> Ellipse {
        Ellipse {
            a_axis: radius,
            b_axis: radius,
        }
    }
}

fn main() {
    let ell1 = Ellipse {
        a_axis: 2.0,
        b_axis: 3.0,
    };

    let ell2 = Ellipse {
        a_axis: 1.0,
        b_axis: 2.0,
    };

    let ell3 = Ellipse::circle(5.0);
    // Area implementation
    println!(
        "The area of ell1 is {} square pixels.", 
        ell1.area()
    );

    println!(
        "The area of ell2 is {} square pixels.", 
        ell2.area()
    );

    println!(
        "The area of ell3 is {} square pixels.", 
        ell3.area()
    );

    // If can hold
    println!(
        "Can ell1 hold ell2? {}",
        ell1.can_hold(&[&ell2])
    );

    println!(
        "Can ell2 hold ell1? {}",
        ell2.can_hold(&[&ell1])
    );

    println!(
        "Can ell3 hold ell1? {}",
        ell3.can_hold(&[&ell1])
    );

    // Print the ellipse
    println!("ell1 is {:#?}", ell1);
    println!("ell2 is {:#?}", ell2);
    println!("ell3 is {:#?}", ell3);
}
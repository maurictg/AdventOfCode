use crate::{Line, Point};

// Get intersection of line. From: https://martin-thoma.com/how-to-check-if-two-line-segments-intersect/#Where_do_two_line_segments_intersect
pub fn get_intersection(a: &Line, b: &Line) -> Line {
    let mut a = a.clone();
    let mut b = b.clone();

    let x1;
    let x2;
    let y1;
    let y2;

    if a.0 .0 == a.1 .0 {
        // Case (A)
        // As a is a perfect vertical line, it cannot be represented
        // nicely in a mathematical way. But we directly know that
        //
        x1 = a.0 .0;
        x2 = x1;
        if b.0 .0 == b.1 .0 {
            // Case (AA): all x are the same!
            // Normalize
            if a.0 .1 > a.1 .1 {
                a = Line(a.1, a.0);
            }
            if b.0 .1 > b.1 .1 {
                b = Line(b.1, b.0);
            }
            if a.0 .1 > b.0 .1 {
                let tmp = a;
                a = b;
                b = tmp;
            }

            // Now we know that the y-value of a.0 is the
            // lowest of all 4 y values
            // this means, we are either in case (AAA):
            //   a: x--------------x
            //   b:    x---------------x
            // or in case (AAB)
            //   a: x--------------x
            //   b:    x-------x
            // in both cases:
            // get the relavant y intervall
            y1 = b.0 .1;
            y2 = a.1 .1.min(b.1 .1);
        } else {
            // Case (AB)
            // we can mathematically represent line b as
            //     y = m*x + t <=> t = y - m*x
            // m = (y1-y2)/(x1-x2)
            let m = (b.0 .1 - b.1 .1) / (b.0 .0 - b.1 .0);
            let t = b.0 .1 - m * b.0 .0;
            y1 = m * x1 + t;
            y2 = y1
        }
    } else if b.0 .0 == b.1 .0 {
        // Case (B)
        // essentially the same as Case (AB), but with
        // a and b switched
        x1 = b.0 .0;
        x2 = x1;

        //let tmp = a;
        //a = b;
        //b = tmp;
        b = a;

        let m = (b.0 .1 - b.1 .1) / (b.0 .0 - b.1 .0);
        let t = b.0 .1 - m * b.0 .0;
        y1 = m * x1 + t;
        y2 = y1
    } else {
        // panic!("Unknown");
        // Case (C)
        // Both lines can be represented mathematically

        let ma = (a.0 .1 - a.1 .1) / (a.0 .0 - a.1 .0);
        let mb = (b.0 .1 - b.1 .1) / (b.0 .0 - b.1 .0);
        let ta = a.0 .1 - ma * a.0 .0;
        let tb = b.0 .1 - mb * b.0 .0;
        if ma == mb {
            // Case (CA)
            // both lines are in parallel. As we know that they
            // intersect, the intersection could be a line
            // when we rotated this, it would be the same situation
            // as in case (AA)

            // Normalize
            if a.0 .0 > a.1 .0 {
                a = Line(a.1, a.0);
            }
            if b.0 .0 > b.1 .0 {
                b = Line(b.1, b.0);
            }
            if a.0 .0 > b.0 .0 {
                let tmp = a;
                a = b;
                b = tmp;
            }

            // get the relavant x intervall
            x1 = b.0 .0;
            x2 = a.1 .0.min(b.1 .0);
            y1 = ma * x1 + ta;
            y2 = ma * x2 + ta;
        } else {
            // Case (CB): only a point as intersection:
            // y = ma*x+ta
            // y = mb*x+tb
            // ma*x + ta = mb*x + tb
            // (ma-mb)*x = tb - ta
            // x = (tb - ta)/(ma-mb)
            x1 = (tb - ta) / (ma - mb);
            y1 = ma * x1 + ta;
            x2 = x1;
            y2 = y1;
        }
    }

    Line(Point(x1, y1), Point(x2, y2))
}
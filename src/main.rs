fn main() {
    println!("Hello, world!");
}

fn add(a:i64, b:i64) -> i64 {
    a + b
}

fn sub(a:i64, b:i64) -> i64 {
    a - b
}

fn multi(a:i64, b:i64) -> i64 {
    a * b
}

fn div(a:i64, b:i64) -> Result<i64, &'static str> {
    if b == 0 {
        return Err("Division by zero is undefined")
    }
    Ok(a / b)
}

//TESTS
#[cfg(test)]
mod tests {
    use crate::{add, sub, multi, div};

    #[test]
    fn basic_add(){
        assert_eq!(2, add(1,1));
        assert_eq!(4, add(2,2));
    }

    #[test]
    fn add_negative(){
        assert_eq!(-2, add(-1,-1));
        assert_eq!(-3, add(-5, 2));
    }

    #[test]
    fn add_limits() {
        let limit = std::i64::MAX;
        let min = std::i64::MIN;

        assert_eq!(limit, add(limit-1, 1));
        assert_eq!(min+1, add(min, 1));
        assert_eq!(-1, add(min, limit));
    }

    #[test]
    #[should_panic]
    fn add_overflow_check() {
        add(i64::MAX, 1);
    }

    #[test]
    #[should_panic]
    fn add_underflow_check() {
        add(i64::MIN, -1);
    }
    //END add tests
    //START sub tests
    #[test]
    fn basic_sub() {
        assert_eq!(0, sub(1, 1));
        assert_eq!(10, sub(20, 10));
    }

    #[test]
    fn sub_negative() {
        assert_eq!(0, sub(-1, -1));
        assert_eq!(-10, sub(-20, -10));
    }

    #[test]
    fn sub_limits() {
        assert_eq!(i64::MIN, sub(i64::MIN+1, 1));
        assert_eq!(i64::MAX, sub(i64::MAX-1, -1));
    }

    #[test]
    #[should_panic]
    fn sub_overflow() {
        sub(i64::MAX, -1);
    }

    #[test]
    #[should_panic]
    fn sub_underflow() {
        sub(i64::MIN, 1);
    }
    //END sub tests
    //START multi tests
    #[test]
    fn multi_by_one() {
        assert_eq!(1, multi(1, 1));
        assert_eq!(10, multi(10, 1));
        assert_eq!(100, multi(100, 1));
    }

    #[test]
    fn multi_by_zero() {
        assert_eq!(0, multi(1, 0));
        assert_eq!(0, multi(100, 0));
        assert_eq!(0, multi(2342342, 0));
    }

    #[test]
    fn basic_multi() {
        assert_eq!(4, multi(2, 2));
        assert_eq!(25, multi(5, 5));
        assert_eq!(32, multi(8, 4));
    }

    #[test]
    fn multi_negative() {
        assert_eq!(-1, multi(1, -1));
        assert_eq!(-1, multi(-1, 1));
        assert_eq!(-6, multi(3, -2));
        assert_eq!(-6, multi(-3, 2));
        assert_eq!(-100, multi(-50, 2));
        assert_eq!(-100, multi(50, -2));
    }

    #[test]
    fn multi_double_negative() {
        assert_eq!(1, multi(-1, -1));
        assert_eq!(9, multi(-3, -3));
        assert_eq!(100, multi(-50, -2));
    }

    #[test]
    fn multi_limit() {
        assert_eq!(i64::MAX, multi(i64::MAX/7, 7));
        assert_eq!(i64::MIN, multi(i64::MIN/2, 2));
    }

    #[test]
    #[should_panic]
    fn multi_overflow() {
        multi(i64::MAX/7, 8);
    }

    #[test]
    #[should_panic]
    fn multi_underflow() {
        multi(i64::MIN/2, 3);
    }
    //END multi tests
    //START div tests
    #[test]
    fn div_by_zero() {
        assert!(div(1, 0).is_err(), "Division by zero is undefined");
    }

    #[test]
    fn div_basic() {
        assert_eq!(0, div(0, 1).unwrap());
        assert_eq!(1, div(1, 1).unwrap());
        assert_eq!(2, div(4, 2).unwrap());
        assert_eq!(10, div(100, 10).unwrap());
    }

    fn div_negative() {
        assert_eq!(-1, div(1, -1).unwrap());
        assert_eq!(-1, div(-1, 1).unwrap());
        assert_eq!(-2, div(-4, 2).unwrap());
        assert_eq!(-2, div(4, -2).unwrap());
        assert_eq!(-10, div(-100, 10).unwrap());
        assert_eq!(-10, div(100, -10).unwrap());
    }

    fn div_double_negative() {
        assert_eq!(1, div(-1, -1).unwrap());
        assert_eq!(2, div(-4, -2).unwrap());
        assert_eq!(10, div(-100, -10).unwrap());
    }
    //END div tests
}

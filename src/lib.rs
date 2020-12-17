// traits1.rs
// Time to implement some traits!
//
// Your task is to implement the trait
// `AppendBar' for the type `String'.
//
// The trait AppendBar has only one function,
// which appends "Bar" to any object
// implementing this trait.

// I AM NOT DONE

#![feature(test)]

extern crate test;


trait AppendBar {
    fn append_bar(self) -> Self;
    fn append_bar1(self) -> Self;
    fn append_bar2(self) -> Self;
    fn append_bar3(self) -> Self;
    fn append_bar4(self) -> Self;
}

impl AppendBar for String {
    // Default
    fn append_bar(self) -> Self {
        self.append_bar2()
    }

    // Answer 1
    // This is 85 LOC and only does one alloc or realloc
    // but needs extra LOC in the non-error path and thus maybe slower.
    fn append_bar1(self) -> Self {
        self + "Bar"
    }

    // Answer 2
    // This is 75 LOC and only does one alloc or realloc
    // and has less LOC in the non-error path.
    // Bencher says FASTEST
    fn append_bar2(self) -> String {
        let mut s = self;
        s.push_str("Bar");
        s
    }

    // Answer 3
    // This is 161 LOC and I thought this might be "faster" and "smaller"
    // as it only needs to check for allocation errors on the
    // "with_capacity" but it still checks the other paths too although
    // they'll never be excersized (assuming owership works:).
    // There is a non-error "fast-path" will only do a memcpy if self already
    // has the capacity.
    // Bencher hasthis Slowest
    fn append_bar3(self) -> String {
        let mut s = String::with_capacity(self.len() + "Bar".len());
        s.push_str(self.as_str());
        s.push_str("Bar");
        s
    }

    // Answer 4
    // This is 127 LOC this is smaller and a little faster than 3
    // but slower then 1 and 2.
    fn append_bar4(self) -> String {
        let mut s = String::with_capacity(self.len() + "Bar".len());
        s = self;
        s.push_str("Bar");
        s
    }
}

fn main() {
    let s = String::from("Foo");
    let s = s.append_bar();
    println!("s: {}", s);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn is_foobar() {
        assert_eq!(String::from("Foo").append_bar(), String::from("FooBar"));
    }

    #[test]
    fn is_barbar() {
        assert_eq!(
            String::from("").append_bar().append_bar(),
            String::from("BarBar")
        );
    }

    #[bench]
    fn bench_append_bar1_new(b: &mut Bencher) {
        b.iter(|| {
            //String::from("Foo").append_bar1();
            let mut test_str = String::new();
            test_str.push_str("Foo");
            test_str.append_bar1();
        });
    }

    #[bench]
    fn bench_append_bar1_cap1(b: &mut Bencher) {
        b.iter(|| {
            //String::from("Foo").append_bar1();
            let mut test_str = String::with_capacity(1);
            test_str.push_str("Foo");
            test_str.append_bar1();
        });
    }

    #[bench]
    fn bench_append_bar1_cap100(b: &mut Bencher) {
        b.iter(|| {
            //String::from("Foo").append_bar1();
            let mut test_str = String::with_capacity(100);
            test_str.push_str("Foo");
            test_str.append_bar1();
        });
    }

    #[bench]
    fn bench_append_bar2_new(b: &mut Bencher) {
        b.iter(|| {
            //String::from("Foo").append_bar2();
            let mut test_str = String::new();
            test_str.push_str("Foo");
            test_str.append_bar2();
        });
    }

    #[bench]
    fn bench_append_bar2_cap1(b: &mut Bencher) {
        b.iter(|| {
            //String::from("Foo").append_bar2();
            let mut test_str = String::with_capacity(1);
            test_str.push_str("Foo");
            test_str.append_bar2();
        });
    }

    #[bench]
    fn bench_append_bar2_cap100(b: &mut Bencher) {
        b.iter(|| {
            //String::from("Foo").append_bar2();
            let mut test_str = String::with_capacity(100);
            test_str.push_str("Foo");
            test_str.append_bar2();
        });
    }

    #[bench]
    fn bench_append_bar3_new(b: &mut Bencher) {
        b.iter(|| {
            let mut test_str = String::new();
            test_str.push_str("Foo");
            test_str.append_bar3();
        });
    }

    #[bench]
    fn bench_append_bar3_cap1(b: &mut Bencher) {
        b.iter(|| {
            let mut test_str = String::with_capacity(1);
            test_str.push_str("Foo");
            test_str.append_bar3();
        });
    }

    #[bench]
    fn bench_append_bar3_cap100(b: &mut Bencher) {
        b.iter(|| {
            let mut test_str = String::with_capacity(100);
            test_str.push_str("Foo");
            test_str.append_bar3();
        });
    }

    #[bench]
    fn bench_append_bar4_new(b: &mut Bencher) {
        b.iter(|| {
            //String::from("Foo").append_bar4();
            let mut test_str = String::new();
            test_str.push_str("Foo");
            test_str.append_bar4();
        });
    }
    #[bench]
    fn bench_append_bar4_cap1(b: &mut Bencher) {
        b.iter(|| {
            //String::from("Foo").append_bar4();
            let mut test_str = String::with_capacity(1);
            test_str.push_str("Foo");
            test_str.append_bar4();
        });
    }
    #[bench]
    fn bench_append_bar4_cap100(b: &mut Bencher) {
        b.iter(|| {
            //String::from("Foo").append_bar4();
            let mut test_str = String::with_capacity(100);
            test_str.push_str("Foo");
            test_str.append_bar4();
        });
    }
}

//#![feature(test)]
//
//extern crate test;
//
//pub fn add_two(a: i32) -> i32 {
//    a + 2
//}
//
//#[cfg(test)]
//mod tests {
//    use super::*;
//    use test::Bencher;
//
//    #[test]
//    fn it_works() {
//        assert_eq!(4, add_two(2));
//    }
//
//    #[bench]
//    fn bench_add_two(b: &mut Bencher) {
//        b.iter(|| add_two(2));
//    }
//}

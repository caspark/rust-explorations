extern crate semver;

use semver::Version;
// use std::thread::Thread;

fn main() {
    {
        assert!(Version::parse("1.2.3") == Ok(Version {
            major: 1u64,
            minor: 2u64,
            patch: 3u64,
            pre: vec!(),
            build: vec!(),
        }));
        println!("Versions compared successfully!");
    }

    {
        let mut v = vec![];
        v.push("Hello");
        let x = &v[0].clone();
        v.push("world");
        println!("{}", x);
    }

    // let guards: Vec<_> = (0..10).map(|_| {
    //     Thread::scoped(|| {
    //         println!("Hello, world!");
    //     })
    // }).collect();

    // {
    //     let (mut x, y): (i32, i32) = (1, 2);
    //     x = 15;
    // }

    {
        // let z: i32;
        // println!("{:?}", z);
        // println!("I like zsss: {}", z); // doesn't work
    }

    {
        // if statements
        let x = 5;
        if x == 5 {
            println!("x is five!");
        } else {
            println!("x is not five!");
        }
    }

    {
        // functions
        fn add_one(x: i32) -> i32 {
            x + 1 // no semicolon
        }

        let x = 4;
        println!("One added to  {:?} is {:?}", x, add_one(x));
    }

    {
        // tuples
        let t = (1, 3);
        println!("Tuple: {:?}", t);
    }

    {
        // playing with for loops
        // this needs to be mutable because a range is really an "iterator" which has state
        // because it records what comes next
        let mut range = 0..10;
        println!("range is {:?}", range);
        for x in range {
            println!("x={} in range", x);
        }
    }

    {
        // testing vectors
        let mut nums = vec![0, 1, 2, 3, 4];
        nums.push(5);
        println!("0->4 vector after pushing 5: {:?}", nums);
        println!("Slice test {:?}", &nums);
    }



    {
        // closures
        let x: i32 = 5;
        let printer = |&:| {
            let i = x + 2;
            println!("x + 2 is: {}", i);
        };
            printer(); // prints "x is: 5"
    }

    {
        // passing a closure to another function
        fn twice<F: Fn(i32) -> i32>(x: i32, f: F) -> i32 {
            f(x) + f(x)
        }

        fn all_three<F, G, H>(x: i32, f: F, g: G, h: H) -> i32
            where F: Fn(i32) -> i32,
                  G: Fn(i32) -> i32,
                  H: Fn(i32) -> i32 {
            f(x) + g(x) + h(x)
        }

        let square = |&: x: i32| { x * x };
        fn square2(x: i32) -> i32 {
            x * x
        }

        twice(5, square);
        // twice(5, square); // doesn't work because square is moved above?
        println!("Twice 10*10 is {}", twice(10, |x| x * x));
        all_three(5, square2, square2, square2);
    }

    {
        // generics
        let x: Option<i32> = Some(5);
        let y = Some::<i32>(5);
        let z = Some(5);
        z.or(Some(1f32));
        println!("Y is {}", y);
    }
}

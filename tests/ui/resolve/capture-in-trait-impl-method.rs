// Regression test for misleading E0434 help on associated functions.
// Associated functions must use `fn` syntax, so suggesting a closure is wrong.

trait T {
    fn t() -> impl FnOnce();
}

fn f(x: String) {
    struct S;
    impl T for S {
        fn t() -> impl FnOnce() {
            move || {
                let _ = x;
                //~^ ERROR can't capture dynamic environment in a fn item
            }
        }
    }
}

fn inherent(x: String) {
    struct S;
    impl S {
        fn method() {
            let _ = x;
            //~^ ERROR can't capture dynamic environment in a fn item
        }
    }
}

fn nested_free_fn_still_suggests_closure(x: String) {
    fn inner() {
        let _ = x;
        //~^ ERROR can't capture dynamic environment in a fn item
    }
}

fn main() {}

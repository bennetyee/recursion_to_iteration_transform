pub fn fib(n: u64) -> u64 {
    if n < 2 {
        n
    } else {
        ({
            let n = n - 1;
            if n < 2 {
                n
            } else {
                ({
                    let n = n - 1;
                    if n < 2 {
                        n
                    } else {
                        fib(n-1) + fib(n-2)
                    }
                }) + ({
                    let n = n - 2;
                    if n < 2 {
                        n
                    } else {
                        fib(n-1) + fib(n-2)
                    }
                })
            }
        }) + ({
            let n = n - 2;
            if n < 2 {
                n
            } else {
                ({
                    let n = n - 1;
                    if n < 2 {
                        n
                    } else {
                        fib(n-1) + fib(n-2)
                    }
                }) + ({
                    let n = n - 2;
                    if n < 2 {
                        n
                    } else {
                        fib(n-1) + fib(n-2)
                    }
                })
            }
        })
    }
}

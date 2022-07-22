```
fn main() {
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // This expression will be assigned to `y`
        x_cube + x_squared + x  // 125 + 25 + 5 = 155
    };

    let z = {
        // The semicolon suppresses this expression and `()` is assigned to `z`
        2 * x   // 155 * 2 = 310
    };

    println!("x is {:?}", x);  // 5
    println!("y is {:?}", y);  // 155
    println!("z is {:?}", z);  // 310
}

```


## Diverging functions: They never return to caller, 
so they may be used in places where a value of any type is expected.

Can use any of the below three methods

1. 
```
use std::thread;
use std::time;
fn never_return_fn() -> ! {
       loop {
        println!("I won't die");
        thread::sleep(time::Duration::from_secs(1))
    }
}
```

2. unimplemented!()

3. panic!()


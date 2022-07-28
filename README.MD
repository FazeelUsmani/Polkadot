# Substrate Saturday Bootcamp by Polkadot India

This repository contains all the assignments, homeworks and problems discussed during the Substrate bootcamp hosted by Polkadot India.
[Substrate Bootcamp](https://lu.ma/ss2?tk=OzHtWq&utm_source=newsletter&utm_medium=sendy).
You can find the recorded videos here:


**Ownership rules:**
Rust uses a third approach: memory is managed through a system of ownership with a set of rules that the compiler checks. If any of the rules are violated, the program won’t compile. None of the features of ownership will slow down your program while it’s running.
* Each value in Rust has an owner.
* There can only be one owner at a time.
* When the owner goes out of scope, the value will be dropped.
  
 
**The Rules of References:**
* At any given time, you can have either one mutable reference or any number of immutable references.
* References must always be valid.
```
let mut x = 1;
let y = &x;
let z = &x;
let a = &mut x; // ERROR
```
```
let mut x = 2;
let a = &mut x;
println!("{}", a);
let b = &mut x;
```

**Patterns:**


There were 6 classes:
1. 9th July 2022 - Rust lesson 1 - 8
2. 16th July 2022 - Rust lesson 9 - 18 (17 optional)
3. 



### Networking Sessions:
* [23rd July 2022 - Extending Substrate Functionality with Custom Pallet by Ivan Temchenko, Senior Rust Engineer at Polkadex](https://twitter.com/_PolkadotIndia/status/1549080840822738944?s=20&t=9GfimlQ5DQmfS8v_Mjylog)



### What to do after completing Rust?
[Small projects with elegand code base](https://practice.rs/elegant-code-base.html)


### Strings

&str - immutable, fast and efficient, static, allocates space in stack
String::from() - mutable, stores in heap, String data structure, dynamic

Ex: 
```
x = "hello";
y = x;  
println!("{},{}",x,y);   // This works

x = String::from("hello");
y = x;  
println!("{},{}",x,y);   // This won't work
```

* Will lose memory once out of scope

* Use clone() when copying String else use copy instead



Array - static
Vector - dynamic

```
let x = vec![1,2,3]
x.push(5);
println!("{} {}", x.len(), x.capacity());
```


__By default, 4 bytes of capacity is allocated for vector__
```
let mut x = Vec::new();
x.push(1);
x.len(); // 1
x.capacity(); // 4
```

Attribute macros # = #[derive(Debug)]
Predefined macros ! = prinln!, format!

{:?} normal print
{:#>} pretty format

impl -> creating new functionality

traits -> extending a functionality. Ex: clone, copy, iterator

__Note:__ Traits are similar to interfaces in other languages, although with some differences.

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

* utf8_slice
You can use utf8_slice to slice UTF8 string, it can index chars instead of bytes.
```
use utf8_slice;
fn main() {
   let s = "The 🚀 goes to the 🌑!";

   let rocket = utf8_slice::slice(s, 4, 5);
   // Will equal "🚀"
}
```

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

* The impl Trait syntax works for straightforward cases but is actually syntax sugar for a longer form, which is called a trait bound.


# blockchain

__Consensus Model:__ The method that a blockchain uses to batch transactions into blocks and to select which node can submit a block to the chain is called the blockchain's consensus model or consensus algorithm. Ex:- proof-of-work consensus model - the node that completes a computational problem first has the right to submit a block to the chain.

__Smart Contract:__ A smart contract is a program that runs on a blockchain and executes transactions on behalf of users under specific conditions.

__Why Substrate?__
Substrate is Flexible, Open, Interoperable via XCM or bridges, Future-proof (This I won't believe). There are hard problems around scale, governance, interoperability, and upgradeability to address. Depends on what do you want to build? Substrate can be used in following situations:
* tailored to a very specific use case
* able to connect and communicate with other blockchains
* customizable with predefined composable modular components
* able to evolve and change with upgrades over time


__Relay Chain__: 

The Relay Chain is the central chain of Polkadot. All validators of Polkadot are staked on the Relay Chain in DOT and validate for the Relay Chain. The Relay Chain is composed of a relatively small number of transaction types that include ways to interact with the governance mechanism, parachain auctions, and participating in NPoS. The Relay Chain has deliberately minimal functionality - for instance, smart contracts are not supported. 


__Parachain__:
A parachain is an application-specific data structure that is globally coherent and validatable by the validators of the Relay Chain. They take their name from the concept of parallelized chains that run parallel to the Relay Chain. Most commonly, a parachain will take the form of a blockchain, but there is no specific need for them to be actual blockchains.
![Resources/images/parachain.png](Resources/images/parachain.png)

__Substrate Architecture:__
In a decentralized network, all nodes act as both clients that request data and as servers that respond to requests for data. 
![Resources/images/arch.avif](Resources/images/arch.avif)

* An outer node that handles network activity such as peer discovery, managing transaction requests, reaching consensus with peers, and responding to RPC calls.
* A runtime that contains all of the business logic for executing the state transition function of the blockchain. The runtime determines whether transactions are valid or invalid and is responsible for handling changes to the blockchain's state transition function.


__Network types:__

Substrate-based blockchains can be used in different types of network architecture. For example, Substrate blockchains are used to build the following network types:

* __Private networks__ that limit access to a restricted set of nodes.

* __Solo chains__ that implement their own security protocol and don't connect or communicate with any other chains. Bitcoin and Ethereum are examples of non-Substrate based solo chains.

* __Relay chains__ that provide decentralized security and communication for other chains that connect to them. Kusama and Polkadot are examples of relay chains.

* __Parachains__ that are built to connect to a relay chain and have the ability to communicate with other chains that use the same relay chain. Because parachains depend on the relay chain to finalize the blocks produced, parachains must implement the same consensus protocol as the relay chain they target.

__FRAME__ - Framework for Runtime Aggregation of Modularized Entities


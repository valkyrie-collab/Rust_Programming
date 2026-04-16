// ============================================
// ERROR: E0507 - Cannot Move Out of Borrowed Value
// ============================================

/*
error[E0507]: cannot move out of dereference of `Ref<'_, Node>`
  --> src/list/mod.rs:59:30
   |
59 |                     header = current.borrow().next;
   |                              ^^^^^^^^^^^^^^^^^^^^^ move occurs because value has type `Option<Rc<RefCell<Node>>>`, which does not implement the `Copy` trait

This error means:
1. You called .borrow() which returns a BORROWED value (Ref)
2. You tried to MOVE OUT of that borrowed value
3. Rust won't let you move owned data out of a borrow
*/

// ============================================
// PART 1: Understanding the Types
// ============================================

use std::rc::Rc;
use std::cell::RefCell;

pub struct Node {
    pub value: i32,
    pub next: Option<Rc<RefCell<Node>>>,
}

/*
Let's understand what types we're dealing with:

Type breakdown:
- Node: A struct
- RefCell<Node>: Allows INTERIOR MUTABILITY - you can mutate through shared refs
- Rc<RefCell<Node>>: Reference counted pointer to a RefCell<Node>
  - Rc = multiple owners can point to same data
  - RefCell = mutate through shared references

So: Rc<RefCell<Node>>
    └─ A counted reference to mutable data
*/

// ============================================
// PART 2: What Happens with .borrow()
// ============================================

fn explain_borrow() {
    let node = Rc::new(RefCell::new(Node { value: 5, next: None }));
    
    // Step 1: Call .borrow()
    let borrowed = node.borrow();
    //  ^^^^^^^^
    //  Type: Ref<'_, Node>
    //  This is a GUARD that gives you READ ACCESS to Node
    //  But it's TEMPORARY - it exists only for this line
    
    // Step 2: Access the field
    let next_option = &borrowed.next;
    //                 ^^^^^^^^^^
    //                 Type: &Option<Rc<RefCell<Node>>>
    //                 This is a REFERENCE to next
    //                 The Ref guard is STILL HELD
    
    // Step 3: Try to MOVE the value (THIS IS THE ERROR!)
    // let next = borrowed.next;
    //           ^^^^^^^^^^^^
    //           Type: Option<Rc<RefCell<Node>>>
    //           ERROR! Can't move because:
    //           1. borrowed is a temporary Ref
    //           2. We can't move out of a borrowed value
    //           3. What would happen to borrowed.next after the move?
}

// ============================================
// PART 3: Visualizing the Memory
// ============================================

/*
BEFORE .borrow():
┌─────────────────────────────────┐
│ Node                            │
│ value: 5                        │
│ next: Some(Rc<RefCell<Node>>) │ ← We want to move THIS
└─────────────────────────────────┘
     ^
     │
  current (Rc<RefCell<Node>>)

AFTER .borrow():
┌─────────────────────────────────┐
│ Node                            │
│ value: 5                        │
│ next: Some(Rc<RefCell<Node>>) │
└─────────────────────────────────┘
     ^                 ^
     │                 │
  current          borrowed (Ref guard)
                   (READ-ONLY guard)

TRYING TO MOVE OUT:
┌─────────────────────────────────┐
│ Node                            │
│ value: 5                        │
│ next: ??? (moved out!)          │ ← What goes here?
└─────────────────────────────────┘
     ^                 ^
     │                 │
  current          borrowed (still holds a reference!)
                   But now next is gone!

Rust says: "NO! Can't do that!"
- borrowed still expects next to be there
- If we move next out, borrowed becomes INVALID
- Rust prevents this at compile time
*/

// ============================================
// PART 4: Why Copy Matters
// ============================================

/*
The error message says:
"value has type `Option<Rc<RefCell<Node>>>`, which does not implement the `Copy` trait"

What does this mean?

COPY trait:
- Types like i32, u8, bool are Copy
- When you "move" a Copy type, it just copies the bits
- No ownership transfer, no danger

NON-COPY trait:
- Types like String, Vec, Rc are NOT Copy
- When you move them, ownership TRANSFERS
- The old location becomes invalid

Example:

// i32 is Copy
let x = 5;
let y = x;  // x is COPIED, not moved
println!("{}", x);  // ✓ x still exists

// String is NOT Copy
let s = String::from("hello");
let t = s;  // s is MOVED to t
println!("{}", s);  // ✗ ERROR: s no longer exists

// Option<Rc<RefCell<Node>>> is NOT Copy
let node = Rc::new(RefCell::new(Node { value: 5, next: None }));
let next = node.borrow().next;  // ✗ ERROR: trying to move non-Copy
*/

// ============================================
// PART 5: The Real Problem
// ============================================

/*
Your code:
    let mut current = self.head.as_ref().unwrap().clone();
    
    loop {
        match current {
            Some(ref node) => {
                if node.next.is_none() {
                    node.next = Some(new_node);
                    break;
                } else {
                    header = current.borrow().next;
                    //       ^^^^^^^^^^^^^^^^^^^^
                    //       ERROR HERE
                }
            }
        }
    }

What's happening:

Step 1: You have `current: Rc<RefCell<Node>>`
Step 2: You call `.borrow()` which returns `Ref<'_, Node>`
Step 3: You access `.next` through the Ref guard
Step 4: You try to ASSIGN it to header
        This requires MOVING the value OUT of the Ref

The Ref guard is temporary - it ends at the semicolon
After that, the Ref is dropped
But if you moved .next out, what would happen to the original node?

VISUALIZED:

Before: Node { next: Some(Rc...) }
After move: Node { next: ??? (MOVED OUT!) }

Rust won't allow this inconsistency!
*/

// ============================================
// PART 6: Solutions and Why They Work
// ============================================

// Solution 1: CLONE the Rc (not the Node!)
fn solution_1_clone_rc() {
    let node = Rc::new(RefCell::new(Node { value: 5, next: None }));
    let borrowed = node.borrow();
    
    // ✓ Clone the Rc, not the actual data
    let next_cloned = borrowed.next.as_ref().map(|rc| rc.clone());
    //  This creates ANOTHER pointer to the same Node
    //  We're not moving the Node itself
    
    // Why this works:
    // - Rc::clone() is cheap (just increments reference count)
    // - The original Node is NOT moved
    // - borrowed still points to valid data
}

// Solution 2: Use .take() with borrow_mut()
fn solution_2_take() {
    let node = Rc::new(RefCell::new(Node { value: 5, next: None }));
    
    // ✓ Use borrow_mut() + take()
    let next = node.borrow_mut().next.take();
    //   ^^^^^^^^^^^^^^
    //   borrow_mut() gives us EXCLUSIVE access (not temporary!)
    //   .take() SAFELY moves the value and replaces with None
    
    // Why this works:
    // - borrow_mut() gives us mutable access to the Node
    // - We have exclusive access - no other Refs exist
    // - .take() is designed to move values out of Options
    // - It's SAFE because we have exclusive access
}

// Solution 3: Clone the Rc before the loop
fn solution_3_clone_before_loop() {
    let current = Rc::new(RefCell::new(Node { value: 5, next: None }));
    let mut current = current.clone();  // ✓ Clone the Rc first
    
    loop {
        // Now current is a new Rc pointing to same Node
        // When we move current.next, we're moving a clone
        // The original is still valid
        
        let next = current.borrow_mut().next.take();
        
        if next.is_none() {
            break;
        } else {
            current = next.unwrap();
        }
    }
}

// ============================================
// PART 7: The Core Concept
// ============================================

/*
The fundamental issue:

Rc<RefCell<Node>> has this structure:
┌──────────────────────────────┐
│ Rc (pointer + ref count)     │ ← You can clone this
├──────────────────────────────┤
│ RefCell<Node>                │
├──────────────────────────────┤
│ Node                         │
│ - value: i32                 │
│ - next: Option<Rc<...>>      │ ← You want to MOVE THIS
└──────────────────────────────┘

When you call .borrow():
- You get temporary read access to Node
- You CAN'T move things out of Node while borrowed

The solution: Clone the Rc (the pointer), not the Node

BEFORE:
Rc (ptr) ──→ Node { next: Some(Rc-X) }
            ┌─ Ref guard (temporary)

AFTER clone:
Rc (ptr) ──→ Node { next: Some(Rc-X) }
Rc (ptr-clone) ──→ Same Node
            ┌─ Ref guard (temporary)

Now when you .take() from ptr-clone.borrow_mut(),
it's safe because the borrow is exclusive
*/

// ============================================
// PART 8: Complete Working Example
// ============================================

pub struct List {
    head: Option<Rc<RefCell<Node>>>,
}

impl List {
    pub fn new() -> Self {
        List { head: None }
    }
    
    // ✓ CORRECT: Use .take() with borrow_mut()
    pub fn append(&mut self, value: i32) {
        let new_node = Rc::new(RefCell::new(Node::new(value)));
        
        if self.head.is_none() {
            self.head = Some(new_node);
        } else {
            let mut current = self.head.as_ref().unwrap().clone();  // Clone the Rc
            
            loop {
                // borrow_mut() gives EXCLUSIVE access
                let next = current.borrow_mut().next.take();
                //         ^^^^^^^^^^^^^^
                //         Not temporary! We have exclusive access
                
                if next.is_none() {
                    // We have the node, and it's safe to modify
                    current.borrow_mut().next = Some(new_node);
                    break;
                } else {
                    // Move to next node
                    current = next.unwrap();
                }
            }
        }
    }
    
    pub fn display(&self) {
        let mut current = self.head.clone();
        
        while let Some(node_rc) = current {
            let node = node_rc.borrow();
            println!("{}", node.value);
            current = node.next.clone();  // Clone the Rc, not the Node
        }
    }
}

impl Node {
    pub fn new(value: i32) -> Self {
        Node { value, next: None }
    }
}

fn main() {
    let mut list = List::new();
    list.append(10);
    list.append(20);
    list.append(30);
    
    list.display();
}

// ============================================
// PART 9: Summary Table
// ============================================

/*
┌──────────────────────────────────────────────────────────┐
│ What You Can and Can't Do                                │
├──────────────────────────────────────────────────────────┤
│ .borrow() returns Ref (temporary)                        │
│                                                          │
│ ✗ Can't move owned data OUT of Ref                       │
│   let x = ref.next;  // ERROR!                           │
│                                                          │
│ ✓ Can read/borrow the data                               │
│   let x = &ref.next;  // OK                              │
│   println!("{:?}", ref.value);  // OK                    │
│                                                          │
│ ✓ Can clone Rc values (cheap)                            │
│   let x = ref.next.clone();  // OK                       │
│                                                          │
│ .borrow_mut() returns RefMut (exclusive)                 │
│                                                          │
│ ✓ Can move owned data OUT of RefMut                      │
│   let x = refmut.next.take();  // OK!                    │
│                                                          │
│ ✓ Can modify the data                                    │
│   refmut.value = 10;  // OK                              │
│                                                          │
│ ✓ Can reassign fields                                    │
│   refmut.next = Some(new_rc);  // OK                     │
└──────────────────────────────────────────────────────────┘
*/

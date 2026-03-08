// E0507 Error - Practical Solutions and Code Examples

use std::rc::Rc;
use std::cell::RefCell;

pub struct Node {
    pub value: i32,
    pub next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(value: i32) -> Self {
        Node {
            value,
            next: None,
        }
    }
}

// ============================================
// SOLUTION 1: Use .borrow_mut().next.take()
// (RECOMMENDED - Most Idiomatic)
// ============================================

pub struct ListSolution1 {
    head: Option<Rc<RefCell<Node>>>,
}

impl ListSolution1 {
    pub fn new() -> Self {
        ListSolution1 { head: None }
    }
    
    pub fn append(&mut self, value: i32) {
        let new_node = Rc::new(RefCell::new(Node::new(value)));
        
        if self.head.is_none() {
            self.head = Some(new_node);
        } else {
            let mut current = self.head.as_ref().unwrap().clone();
            
            loop {
                // ✓ SOLUTION 1: Use borrow_mut().next.take()
                let next = current.borrow_mut().next.take();
                //         ^^^^^^^^^^^^^^
                //         Exclusive access - safe to move
                
                if next.is_none() {
                    // Modify the current node
                    current.borrow_mut().next = Some(new_node);
                    break;
                } else {
                    // Move to the next node
                    current = next.unwrap();
                }
            }
        }
    }
    
    pub fn display(&self) {
        let mut current = self.head.clone();
        while let Some(node_rc) = current {
            println!("{}", node_rc.borrow().value);
            current = node_rc.borrow().next.clone();
        }
    }
}

// ============================================
// SOLUTION 2: Clone the Rc at Each Step
// (SIMPLE - Easier to Understand)
// ============================================

pub struct ListSolution2 {
    head: Option<Rc<RefCell<Node>>>,
}

impl ListSolution2 {
    pub fn new() -> Self {
        ListSolution2 { head: None }
    }
    
    pub fn append(&mut self, value: i32) {
        let new_node = Rc::new(RefCell::new(Node::new(value)));
        
        if self.head.is_none() {
            self.head = Some(new_node);
        } else {
            let mut current = self.head.as_ref().unwrap().clone();
            
            loop {
                // ✓ SOLUTION 2: Clone the Rc from the Option
                let next_option = current.borrow().next.clone();
                //                                          ^^^^^
                //                                    Clone the Rc (cheap)
                
                if next_option.is_none() {
                    current.borrow_mut().next = Some(new_node);
                    break;
                } else {
                    current = next_option.unwrap();
                }
            }
        }
    }
    
    pub fn display(&self) {
        let mut current = self.head.clone();
        while let Some(node_rc) = current {
            println!("{}", node_rc.borrow().value);
            current = node_rc.borrow().next.clone();
        }
    }
}

// ============================================
// SOLUTION 3: Use match instead of if let
// (ALTERNATIVE - More Pattern Matching)
// ============================================

pub struct ListSolution3 {
    head: Option<Rc<RefCell<Node>>>,
}

impl ListSolution3 {
    pub fn new() -> Self {
        ListSolution3 { head: None }
    }
    
    pub fn append(&mut self, value: i32) {
        let new_node = Rc::new(RefCell::new(Node::new(value)));
        
        match &self.head {
            None => {
                self.head = Some(new_node);
            }
            Some(head) => {
                let mut current = head.clone();
                
                loop {
                    // ✓ SOLUTION 3: match with borrow_mut()
                    match current.borrow_mut().next.take() {
                        //              ^^^^^^^^
                        //              Exclusive access to take
                        
                        None => {
                            current.borrow_mut().next = Some(new_node);
                            break;
                        }
                        Some(next_node) => {
                            current = next_node;
                        }
                    }
                }
            }
        }
    }
    
    pub fn display(&self) {
        let mut current = self.head.clone();
        while let Some(node_rc) = current {
            let node = node_rc.borrow();
            println!("{}", node.value);
            current = node.next.clone();
        }
    }
}

// ============================================
// SOLUTION 4: Extract to Helper Function
// (CLEAN - Best for Readability)
// ============================================

pub struct ListSolution4 {
    head: Option<Rc<RefCell<Node>>>,
}

impl ListSolution4 {
    pub fn new() -> Self {
        ListSolution4 { head: None }
    }
    
    pub fn append(&mut self, value: i32) {
        let new_node = Rc::new(RefCell::new(Node::new(value)));
        
        if let Some(head) = &self.head {
            Self::append_to_node(head.clone(), new_node);
        } else {
            self.head = Some(new_node);
        }
    }
    
    // ✓ SOLUTION 4: Extract logic to helper
    fn append_to_node(current: Rc<RefCell<Node>>, new_node: Rc<RefCell<Node>>) {
        let next = current.borrow_mut().next.take();
        
        if next.is_none() {
            current.borrow_mut().next = Some(new_node);
        } else {
            Self::append_to_node(next.unwrap(), new_node);
        }
    }
    
    pub fn display(&self) {
        if let Some(head) = &self.head {
            Self::display_node(head.clone());
        }
    }
    
    fn display_node(current: Rc<RefCell<Node>>) {
        let node = current.borrow();
        println!("{}", node.value);
        
        if let Some(next) = &node.next {
            Self::display_node(next.clone());
        }
    }
}

// ============================================
// TEST ALL SOLUTIONS
// ============================================

fn main() {
    println!("=== Solution 1: borrow_mut().take() ===");
    let mut list1 = ListSolution1::new();
    list1.append(10);
    list1.append(20);
    list1.append(30);
    list1.display();
    
    println!("\n=== Solution 2: Clone the Rc ===");
    let mut list2 = ListSolution2::new();
    list2.append(10);
    list2.append(20);
    list2.append(30);
    list2.display();
    
    println!("\n=== Solution 3: Match with take() ===");
    let mut list3 = ListSolution3::new();
    list3.append(10);
    list3.append(20);
    list3.append(30);
    list3.display();
    
    println!("\n=== Solution 4: Helper Function ===");
    let mut list4 = ListSolution4::new();
    list4.append(10);
    list4.append(20);
    list4.append(30);
    list4.display();
}

// ============================================
// COMPARISON OF SOLUTIONS
// ============================================

/*
┌────────────────────┬──────────────┬─────────────┬────────────┐
│ Solution           │ Performance  │ Readability │ Notes      │
├────────────────────┼──────────────┼─────────────┼────────────┤
│ 1: borrow_mut()    │ ★★★★★       │ ★★★★       │ Most        │
│    .take()         │ (excellent)  │ (good)      │ idiomatic  │
├────────────────────┼──────────────┼─────────────┼────────────┤
│ 2: Clone Rc        │ ★★★★★       │ ★★★★★      │ Simplest   │
│                    │ (excellent)  │ (excellent) │ easiest    │
├────────────────────┼──────────────┼─────────────┼────────────┤
│ 3: match + take()  │ ★★★★★       │ ★★★★       │ Good alt   │
│                    │ (excellent)  │ (good)      │ pattern    │
├────────────────────┼──────────────┼─────────────┼────────────┤
│ 4: Helper function │ ★★★★★       │ ★★★★★      │ Recursive  │
│    (recursive)     │ (excellent)  │ (excellent) │ approach   │
└────────────────────┴──────────────┴─────────────┴────────────┘

Performance note: All are O(1) per operation - difference is negligible
Cloning Rc is O(1) - just increments a reference counter
*/

// ============================================
// WHY THESE SOLUTIONS WORK
// ============================================

/*
Solution 1: borrow_mut().take()
┌─ borrow_mut() gives EXCLUSIVE access
├─ RefMut allows you to MOVE values
├─ .take() is designed for this
├─ Replaces .next with None (valid state)
└─ Safe and idiomatic

Solution 2: Clone the Rc
┌─ borrow() gives read-only access (OK)
├─ Can't move values, but CAN clone Rc
├─ Cloning Rc is cheap (just increments counter)
├─ New Rc points to same Node
├─ Original Node never moved
└─ Simple to understand

Solution 3: match + take()
┌─ Uses pattern matching on the Option
├─ take() extracts and replaces with None
├─ Similar to Solution 1, different syntax
├─ Matches on both None and Some cases
└─ Good for complex logic

Solution 4: Helper function
┌─ Uses recursion instead of iteration
├─ Cleaner separation of concerns
├─ Each call handles one node
├─ Stack space used for recursion
└─ Most functional style
*/

// ============================================
// HOW TO CHOOSE WHICH SOLUTION
// ============================================

/*
Use Solution 1 if:
✓ You want the most idiomatic Rust
✓ You're comfortable with .take()
✓ You prefer iterative code

Use Solution 2 if:
✓ You want the simplest, clearest code
✓ You prefer something beginner-friendly
✓ Performance difference doesn't matter (it doesn't!)

Use Solution 3 if:
✓ You like pattern matching
✓ You need to handle multiple cases clearly
✓ You want an alternative to if-let

Use Solution 4 if:
✓ You prefer recursive solutions
✓ You want cleaner function separation
✓ Stack space is not a concern
*/

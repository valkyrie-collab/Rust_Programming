// ============================================
// TRAITS - Complete Explanation
// ============================================

/*
TRAIT = A way to define SHARED BEHAVIOR
        that different types can implement

Think of it like a CONTRACT or INTERFACE
"If you implement this trait, you MUST have these methods"
*/

// ============================================
// PART 1: What is a Trait?
// ============================================

/*
TRAIT = A set of methods that a type PROMISES to implement

Example:
trait Summary {
    fn summarize(&self) -> String;
}

This says: "Any type that implements Summary MUST have a summarize method"
*/

trait Summary {
    fn summarize_author(&self) -> String;
    
    fn summarize(&self) -> String {
        format!("(Read more from {}...", self.summarize_author())
    }
}

/*
Breaking it down:

trait Summary {
      ^^^^^^^
      Name of the trait

    fn summarize_author(&self) -> String;
       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
       METHOD SIGNATURE - must be implemented by types
       The ; (semicolon) means: "type must provide the implementation"
    
    fn summarize(&self) -> String {
       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
       METHOD WITH DEFAULT IMPLEMENTATION
       The { } (braces) means: "here's a default implementation"
       Types can use this OR override it
}
*/

// ============================================
// PART 2: Implementing a Trait
// ============================================

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool
}

impl Summary for Tweet {
    //  ^^^^^^^
    //  Tweet is implementing the Summary trait
    
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    
    // Note: We DON'T implement summarize() here
    // We use the DEFAULT implementation from the trait
}

/*
What this means:

impl Summary for Tweet
│    │        │
│    │        └─ This is the type implementing the trait
│    └─ The trait being implemented
└─ Implementation block

Tweet now has TWO methods from Summary:
1. summarize_author() - we implemented this
2. summarize() - using the DEFAULT from trait
*/

// ============================================
// PART 3: How the Default Implementation Works
// ============================================

/*
In the trait:

fn summarize(&self) -> String {
    format!("(Read more from {}...", self.summarize_author())
}
                                 ^^^^^^^^^^^^^^^^^^^^^^
                                 Calls summarize_author()
                                 which is implemented by the type

So when you call tweet.summarize():

Step 1: Calls the default summarize() from trait
Step 2: summarize() calls self.summarize_author()
Step 3: self.summarize_author() calls Tweet's implementation
Step 4: Returns the formatted string
*/

// ============================================
// PART 4: Step-by-Step Execution
// ============================================

fn step_by_step() {
    let tweet = Tweet {
        username: String::from("horse_ebook"),
        content: String::from("of course as you probably already know, people"),
        reply: false,
        retweet: false
    };
    
    // When you call:
    println!("1. new tweet: {}", tweet.summarize());
    
    // Here's what happens INSIDE:
    
    // Step 1: summarize() is called (from trait default)
    // Step 2: Inside summarize(), it runs:
    //         format!("(Read more from {}...", self.summarize_author())
    //                                          ^^^^^^^^^^^^^^^^^^^^^
    //                                          Calls THIS method
    
    // Step 3: self.summarize_author() executes:
    //         format!("@{}", self.username)
    //                        ^^^^^^^^^^^^^^
    //                        Gets the username field
    //                        username = "horse_ebook"
    //                        Returns: "@horse_ebook"
    
    // Step 4: Back to summarize(), insert the result:
    //         format!("(Read more from {}...", "@horse_ebook")
    //         Result: "(Read more from @horse_ebook..."
    
    // Step 5: println! prints it
    // Output: 1. new tweet: (Read more from @horse_ebook...
}

// ============================================
// PART 5: Multiple Types Implementing Same Trait
// ============================================

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("by {}", self.author)
    }
    
    // Uses default summarize() from trait
}

fn multiple_implementations() {
    let tweet = Tweet {
        username: String::from("horse_ebook"),
        content: String::from("of course as you probably already know, people"),
        reply: false,
        retweet: false
    };
    
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL."
        ),
    };
    
    // Both use summarize() but with different implementations!
    println!("Tweet: {}", tweet.summarize());
    println!("Article: {}", article.summarize());
    
    /*
    Output:
    Tweet: (Read more from @horse_ebook...
    Article: (Read more from by Iceburgh...
    
    Same method, different results because:
    - tweet.summarize_author() returns "@horse_ebook"
    - article.summarize_author() returns "by Iceburgh"
    */
}

// ============================================
// PART 6: Overriding Default Implementation
// ============================================

trait Display {
    fn show(&self) -> String;
    
    fn show_with_timestamp(&self) -> String {
        format!("[{}] {}", "2024", self.show())
    }
}

struct Post {
    title: String
}

impl Display for Post {
    fn show(&self) -> String {
        format!("Post: {}", self.title)
    }
    
    // Uses default show_with_timestamp()
}

struct Story {
    title: String
}

impl Display for Story {
    fn show(&self) -> String {
        format!("Story: {}", self.title)
    }
    
    // OVERRIDE the default show_with_timestamp()
    fn show_with_timestamp(&self) -> String {
        format!("📖 {} - {}", self.title, "2024")
    }
}

fn override_example() {
    let post = Post { title: String::from("Rust Tips") };
    let story = Story { title: String::from("Once upon a time") };
    
    println!("{}", post.show_with_timestamp());
    // Uses default: [2024] Post: Rust Tips
    
    println!("{}", story.show_with_timestamp());
    // Uses overridden: 📖 Once upon a time - 2024
}

// ============================================
// PART 7: Trait as Function Parameter
// ============================================

fn notify(item: &impl Summary) {
    //         ^^^^^^^^^^^^^^
    //         Parameter accepts ANY type that implements Summary
    
    println!("Breaking news! {}", item.summarize());
}

fn trait_as_parameter() {
    let tweet = Tweet {
        username: String::from("horse_ebook"),
        content: String::from("of course as you probably already know, people"),
        reply: false,
        retweet: false
    };
    
    let article = NewsArticle {
        headline: String::from("Penguins win!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("..."),
    };
    
    notify(&tweet);    // ✓ Tweet implements Summary
    notify(&article);  // ✓ NewsArticle implements Summary
    
    // Works with ANY type that implements Summary!
}

// ============================================
// PART 8: Trait Bounds
// ============================================

fn print_summary<T: Summary>(item: &T) {
    //         ^  ^^^^^^^^
    //         │  Trait bound - T MUST implement Summary
    //         Generic type parameter
    
    println!("Summary: {}", item.summarize());
}

fn trait_bounds_example() {
    let tweet = Tweet {
        username: String::from("horse_ebook"),
        content: String::from("of course as you probably already know, people"),
        reply: false,
        retweet: false
    };
    
    print_summary(&tweet);  // ✓ Works
    
    let number = 42;
    // print_summary(&number);  // ✗ ERROR! i32 doesn't implement Summary
}

// ============================================
// PART 9: Complete Example with Explanation
// ============================================

fn complete_example() {
    // Step 1: Create a Tweet instance
    let tweet = Tweet {
        username: String::from("horse_ebook"),
        content: String::from("of course as you probably already know, people"),
        reply: false,
        retweet: false
    };
    
    // Step 2: Call summarize()
    let summary = tweet.summarize();
    
    // Step-by-step execution:
    // 1. tweet.summarize() is called
    // 2. Rust looks for summarize() method
    // 3. Finds it in Summary trait (default implementation)
    // 4. Default implementation is:
    //    format!("(Read more from {}...", self.summarize_author())
    // 5. Calls self.summarize_author()
    // 6. Finds it in Tweet's impl Summary block
    // 7. Executes: format!("@{}", self.username)
    // 8. Returns "@horse_ebook"
    // 9. Inserts into format string:
    //    "(Read more from @horse_ebook..."
    // 10. Returns this string
    
    println!("1. new tweet: {}", summary);
    // Output: 1. new tweet: (Read more from @horse_ebook...
}

// ============================================
// PART 10: Key Concepts
// ============================================

/*
TRAIT:
├─ Defines REQUIRED methods (MUST implement)
├─ Can include DEFAULT implementations (optional)
└─ Multiple types can implement the same trait

IMPL:
├─ Provides the IMPLEMENTATION for a trait
├─ MUST implement all required methods
├─ CAN override default implementations
└─ MUST be impl Summary for TypeName

BENEFIT:
├─ Code reuse (default implementations)
├─ Shared behavior across types
├─ Can pass different types to same function
├─ Type safety with trait bounds
└─ Flexibility and extensibility
*/

fn main() {
    println!("=== Complete Example ===");
    complete_example();
    
    println!("\n=== Multiple Types ===");
    multiple_implementations();
    
    println!("\n=== As Function Parameter ===");
    trait_as_parameter();
}

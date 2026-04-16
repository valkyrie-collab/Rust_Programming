// ============================================
// &[i32] vs [i32; 7] - Detailed Comparison
// ============================================

/*
KEY DIFFERENCE:

&[i32]     = Reference to a SLICE (variable length, unknown at compile time)
[i32; 7]   = Array (fixed length, EXACTLY 7 elements, known at compile time)
*/

// ============================================
// PART 1: Size - THE MOST IMPORTANT DIFFERENCE
// ============================================

fn sizes() {
    // [i32; 7] - FIXED SIZE
    let array: [i32; 7] = [1, 2, 3, 4, 5, 6, 7];
    //         ^^^^^^^
    //         Fixed size: EXACTLY 7 elements
    //         Compile-time known: 28 bytes (7 × 4 bytes)
    
    println!("Array size: {}", std::mem::size_of_val(&array));  // 28 bytes
    println!("Array len: {}", array.len());                      // 7 (always)
    
    // &[i32] - VARIABLE SIZE (at compile time)
    let slice: &[i32] = &array;
    //         ^^^^^^^
    //         Variable length (runtime known, compile-time unknown)
    //         Has TWO pieces of information:
    //         1. Pointer to data (8 bytes)
    //         2. Length of data (8 bytes)
    //         Total: 16 bytes
    
    println!("Slice size: {}", std::mem::size_of_val(&slice));  // 16 bytes
    println!("Slice len: {}", slice.len());                      // 7
    
    /*
    MEMORY LAYOUT:
    
    [i32; 7]:
    ┌──────────────────────────────────────────────────┐
    │ [1, 2, 3, 4, 5, 6, 7]                            │
    │ 28 bytes (all data inline on stack)              │
    └──────────────────────────────────────────────────┘
    Size: 28 bytes, Location: Stack
    
    &[i32]:
    ┌─────────────────┐
    │ ptr (8 bytes) ──┼──→ [1, 2, 3, 4, 5, 6, 7]
    │ len (8 bytes)   │
    └─────────────────┘
    Size: 16 bytes (metadata), Location: Stack
    Data: Points to wherever the array is (stack or heap)
    */
}

// ============================================
// PART 2: Memory Location - WHERE IS DATA?
// ============================================

fn memory_location() {
    // [i32; 7] - Data is INLINE
    let array: [i32; 7] = [1, 2, 3, 4, 5, 6, 7];
    //         ^^^^^^^
    //         Data is stored DIRECTLY in this variable
    //         Location: STACK
    //         ┌──────────────────┐
    //         │ [1,2,3,4,5,6,7]  │ ← All data here on stack
    //         └──────────────────┘
    
    // &[i32] - Data is REFERENCED
    let slice: &[i32] = &array;
    //         ^^^^^^^
    //         This variable is just a POINTER
    //         Points to: the array (which is on stack in this case)
    //         ┌──────────────┐
    //         │ ptr | len    │  ← Just metadata on stack
    //         └──────────────┘
    //              ↓
    //         [1, 2, 3, ...] ← Points to actual data elsewhere
    
    // Can also point to heap:
    let vec = vec![1, 2, 3, 4, 5, 6, 7];
    let slice2: &[i32] = &vec;
    //  ^^^^^^
    //  Points to heap data (inside the Vec)
    //  ┌──────────────┐
    //  │ ptr | len    │  ← Metadata on stack
    //  └──────────────┘
    //       ↓
    //  ┌────────────────────┐
    //  │ [1,2,3,4,5,6,7]    │ ← Actual data on heap
    //  └────────────────────┘
}

// ============================================
// PART 3: Size Known When? - CRITICAL DIFFERENCE
// ============================================

/*
[i32; 7]:
Size: KNOWN AT COMPILE TIME
└─ Compiler sees [i32; 7] and knows it's exactly 28 bytes
└─ The number 7 is part of the TYPE
└─ Can allocate stack space during compilation

&[i32]:
Size: KNOWN AT RUNTIME (for the slice itself)
└─ Compiler sees &[i32] - size is variable!
└─ The reference always takes 16 bytes
└─ But the slice it points to could be any length
└─ Length is stored in the reference (second field)
└─ Compiler doesn't know if it's 7 or 100 or 1000
*/

// Example:
fn compile_time_vs_runtime() {
    // [i32; 7] - SIZE KNOWN AT COMPILE TIME
    let array: [i32; 7] = [1, 2, 3, 4, 5, 6, 7];
    //         ^^^^^^^
    //         Compiler generates code knowing this is 28 bytes
    
    // [i32; 5] - DIFFERENT TYPE! SIZE KNOWN AT COMPILE TIME
    let array2: [i32; 5] = [1, 2, 3, 4, 5];
    //          ^^^^^^^
    //          Different type! Only 20 bytes
    //          array and array2 are DIFFERENT TYPES
    
    // These are incompatible types!
    // let combined: [i32; ?] = ???;  // ✗ Can't combine them
    
    // &[i32] - SIZE KNOWN AT RUNTIME
    let slice1: &[i32] = &array;   // Points to 7 elements
    let slice2: &[i32] = &array2;  // Points to 5 elements
    
    // These are the SAME TYPE!
    // Both are &[i32]
    // They just point to different lengths at runtime
    
    println!("array len: {}", slice1.len());   // 7 (runtime known)
    println!("array2 len: {}", slice2.len());  // 5 (runtime known)
}

// ============================================
// PART 4: Ownership - WHO OWNS THE DATA?
// ============================================

fn ownership() {
    // [i32; 7] - YOU OWN THE DATA
    let array: [i32; 7] = [1, 2, 3, 4, 5, 6, 7];
    //  ^^^^^
    //  OWNER - you own these 7 elements
    //  Can modify them
    //  Data dies when array goes out of scope
    
    // &[i32] - YOU DON'T OWN THE DATA
    let slice: &[i32] = &array;
    //  ^^^^^
    //  BORROWER - you don't own the data
    //  Can't modify (immutable borrow)
    //  Data lives as long as what it points to
    
    // When array is dropped, slice becomes invalid:
    // let slice: &[i32] = &array;
    // drop(array);
    // println!("{:?}", slice);  // ✗ ERROR! array was dropped
}

// ============================================
// PART 5: Mutability - CAN YOU MODIFY?
// ============================================

fn mutability() {
    // [i32; 7] - CAN BE MUTABLE
    let mut array: [i32; 7] = [1, 2, 3, 4, 5, 6, 7];
    //  ^^^
    //  mutable
    
    array[0] = 100;  // ✓ Can modify
    println!("{:?}", array);  // [100, 2, 3, 4, 5, 6, 7]
    
    // &[i32] - IMMUTABLE REFERENCE
    let slice: &[i32] = &array;
    // slice[0] = 200;  // ✗ ERROR! Can't modify through immutable reference
    
    // &mut [i32] - MUTABLE REFERENCE
    let mut_slice: &mut [i32] = &mut array;
    mut_slice[0] = 200;  // ✓ Can modify
    println!("{:?}", array);  // [200, 2, 3, 4, 5, 6, 7]
}

// ============================================
// PART 6: Type Signature - SAME OR DIFFERENT?
// ============================================

fn type_signatures() {
    let arr1: [i32; 7] = [1, 2, 3, 4, 5, 6, 7];
    let arr2: [i32; 7] = [10, 20, 30, 40, 50, 60, 70];
    let arr3: [i32; 5] = [1, 2, 3, 4, 5];
    
    // arr1 and arr2 are the SAME TYPE: [i32; 7]
    // arr1 and arr3 are DIFFERENT TYPES: [i32; 7] vs [i32; 5]
    
    // You CAN'T mix them:
    // let x: [i32; 7] = arr3;  // ✗ ERROR! Type mismatch
    
    // But with slices:
    let slice1: &[i32] = &arr1;
    let slice2: &[i32] = &arr2;
    let slice3: &[i32] = &arr3;
    
    // All are the SAME TYPE: &[i32]
    // You CAN have them in the same collection:
    let slices: Vec<&[i32]> = vec![slice1, slice2, slice3];
    for s in slices {
        println!("Slice len: {}", s.len());
    }
}

// ============================================
// PART 7: Function Parameters - WHY USE SLICES?
// ============================================

// Function 1: Takes array (rigid)
fn process_array(arr: [i32; 7]) {
    //              ^^^^^^^
    //              MUST be exactly 7 elements
    //              If you have 5 elements, ✗ ERROR!
    for &x in arr.iter() {
        println!("{}", x);
    }
}

// Function 2: Takes slice (flexible)
fn process_slice(slice: &[i32]) {
    //               ^^^^^^^
    //               ANY length!
    //               5, 7, 100, doesn't matter
    for &x in slice {
        println!("{}", x);
    }
}

fn calling_functions() {
    let arr7: [i32; 7] = [1, 2, 3, 4, 5, 6, 7];
    let arr5: [i32; 5] = [1, 2, 3, 4, 5];
    
    // With array function:
    process_array(arr7);  // ✓ Works
    // process_array(arr5);  // ✗ ERROR! Type mismatch
    
    // With slice function:
    process_slice(&arr7);      // ✓ Works! Automatically coerced
    process_slice(&arr5);      // ✓ Works! Automatically coerced
    
    let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    process_slice(&vec);       // ✓ Works! Coerced to &[i32]
    process_slice(&vec[2..5]); // ✓ Works! Partial slice
}

// ============================================
// PART 8: COMPLETE COMPARISON TABLE
// ============================================

/*
┌──────────────────┬──────────────────┬───────────────────┐
│ Property         │ [i32; 7]         │ &[i32]            │
├──────────────────┼──────────────────┼───────────────────┤
│ Type Name        │ Array (fixed)    │ Slice (dynamic)   │
├──────────────────┼──────────────────┼───────────────────┤
│ Size at Compile  │ Known (28 bytes) │ Unknown           │
│ Time             │                  │ (points to ?)     │
├──────────────────┼──────────────────┼───────────────────┤
│ Size at Runtime  │ Always 28 bytes  │ Known (via len)   │
├──────────────────┼──────────────────┼───────────────────┤
│ Data Location    │ Stored inline    │ Pointer to data   │
├──────────────────┼──────────────────┼───────────────────┤
│ Memory Size      │ 28 bytes         │ 16 bytes (ptr+len)│
├──────────────────┼──────────────────┼───────────────────┤
│ Owns Data?       │ ✓ Yes            │ ✗ No (borrows)    │
├──────────────────┼──────────────────┼───────────────────┤
│ Can Modify?      │ If mut           │ If &mut           │
├──────────────────┼──────────────────┼───────────────────┤
│ Length Fixed?    │ ✓ Yes (7)        │ ✗ No (runtime)    │
├──────────────────┼──────────────────┼───────────────────┤
│ Type Compatible? │ [i32; 7] only    │ Any &[i32]        │
├──────────────────┼──────────────────┼───────────────────┤
│ Can Pass to Fn?  │ Must match type  │ Very flexible     │
└──────────────────┴──────────────────┴───────────────────┘
*/

// ============================================
// PART 9: VISUAL MEMORY COMPARISON
// ============================================

fn memory_visualization() {
    let array: [i32; 7] = [1, 2, 3, 4, 5, 6, 7];
    let slice: &[i32] = &array;
    
    println!("ARRAY [i32; 7]:");
    println!("┌─────────────────────────────────┐");
    println!("│ Data:  [1, 2, 3, 4, 5, 6, 7]    │");
    println!("│ Size:  28 bytes (7 × 4)         │");
    println!("│ Owner: YES                      │");
    println!("│ Type:  [i32; 7] (fixed)         │");
    println!("│ Loc:   STACK                    │");
    println!("└─────────────────────────────────┘");
    
    println!("\n&[i32] SLICE:");
    println!("┌──────────────────┐");
    println!("│ ptr: ────┐       │");
    println!("│ len: 7   │       │");
    println!("└──────────┼───────┘");
    println!("           │");
    println!("           ↓");
    println!("┌─────────────────────────────────┐");
    println!("│ [1, 2, 3, 4, 5, 6, 7]           │");
    println!("└─────────────────────────────────┘");
    println!("Size:  16 bytes (ptr + len)");
    println!("Owner: NO (just pointing)");
    println!("Type:  &[i32] (flexible)");
}

// ============================================
// PART 10: THE KEY TAKEAWAY
// ============================================

fn key_difference() {
    /*
    THE KEY DIFFERENCE:
    
    [i32; 7] = Fixed size array
              = Size is PART OF THE TYPE
              = Type checker knows exact size at compile time
              = You own the data
              = Good for: fixed-size data, stack allocation
    
    &[i32] = Flexible reference
           = Size is NOT part of the type
           = Type checker doesn't know exact size at compile time
           = You don't own the data (just reference it)
           = Good for: variable-size data, generic functions
    
    ANALOGY:
    
    [i32; 7] = A specific box that holds exactly 7 items
               You own this box
               The box is always this size
    
    &[i32] = A window into ANY box with ANY number of items
             You don't own the box (just looking)
             Could be looking at 5 items, 7 items, or 100 items
    */
}

fn main() {
    println!("=== Sizes ===");
    sizes();
    
    println!("\n=== Memory Location ===");
    memory_location();
    
    println!("\n=== Compile Time vs Runtime ===");
    compile_time_vs_runtime();
    
    println!("\n=== Memory Visualization ===");
    memory_visualization();
}

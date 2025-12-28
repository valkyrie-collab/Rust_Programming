we declare the variables using let <variable name> and let mut "mut" basically used to make the variable mutable because by default variables in rust are immutable

when declearing any variable declearing the datatype is a good practice or we can end up with some shit error

rust maninly takes input in characters which it calles String

to convert it to interger we need to do this <variable name>.trim().parse().expect("Enter your number...);

Err(_) this "_" Specially means that error is ignored but if we want to use it or show error we can do this Err(e) => {println!("The error is: {}", e); continue;}

when we use Ok(num) or Ok(<other name>) if this name is validetaed with the return struct of the variable then the it will return that value like here in guessing game my return struct is interger 32 of if the numbers are integer then it will return integer and for any other like string it will be error

why we use usize and not u32 in rust? Because usize change on demand means it can exceed 32 to 64bits where as u32 only 32bits; this ensures PORTIBLITY because it can work on both 32bits and 64bits TYPE SAFETY and CONSISTENCY and also the stander library implements Index<usize> and but not Index<u32>

enumerate() after iter() gives index and the items of the String or list;



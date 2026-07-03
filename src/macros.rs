#[macro_export] 
macro_rules! debug_vars {
    ($($val:expr),+ $(,)?) => {
        if cfg!(debug_assertions) {
            fn type_name_of<T>(_: T) -> &'static str { std::any::type_name::<T>() }
            fn dummy_fn() {}
            let full_name = type_name_of(dummy_fn);
            let fn_path = &full_name[..full_name.len() - 10];
            let fn_name = fn_path.split("::").last().unwrap_or(fn_path);

            println!();
            println!("[INVOKED BY] fn {}()", fn_name);
        }

        {
            #![allow(unused)]
            let mut first = true;            
            $(
                if !first {
                    print!(",\n");
                }
                first = false;
                
                print!("\t");
                print!("{}: {:?}", stringify!($val), $val);
            )+
            
            println!(); 
        }
    };
}

// This macro executes the code and records the actual time it takes for the execution to complete.
#[macro_export] 
macro_rules! measure {
    ($code:block) => {{
        let start = std::time::Instant::now();
        let result = $code; 
        let time = start.elapsed();
        (result, time) 
    }};
}
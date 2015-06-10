
macro_rules! cascade {
    ($(
        if #[cfg($($meta:meta),*)] { $($it:item)* }
    ) else * else {
        $($it2:item)*
    }) => {
        __items! {
            () ;
            $( ( ($($meta),*) ($($it)*) ), )*
            ( () ($($it2)*) ),
        }
    }
}

macro_rules! __items {
    (($($not:meta,)*) ; ) => {};
    (($($not:meta,)*) ; ( ($($m:meta),*) ($($it:item)*) ), $($rest:tt)*) => {
        __apply! { cfg(all($($m,)* not(any($($not),*)))), $($it)* }
        __items! { ($($not,)* $($m,)*) ; $($rest)* }
    }
}

macro_rules! __apply {
    ($m:meta, $($it:item)*) => {
        $(#[$m] $it)*
    }
}

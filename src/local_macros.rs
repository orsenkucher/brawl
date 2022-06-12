// #[macro_export]
macro_rules! req_future {
    (
        // pub def: | ident: type, (,) cho? | {block of code}
        $v2:vis def: | $( $arg:ident: $ArgTy:ty ),* $(,)? | $body:block

        // some attributes
        $(#[$($meta:tt)*])*
        $v:vis $i:ident<$T:ident> ($inner:ident) -> $Out:ty
        // where can be 1 or 0
        // there could be multiple tt rows
        $(where $($wh:tt)*)?
    ) => {
        #[pin_project::pin_project]
        // pub struct Send<U>
        pub struct $i<$T>
        $(where $($wh)*)?
        {
            #[pin]
            // so $inner is just ident of a module (ex. inner0)
            // inner0::Send<U>
            inner: $inner::$i<$T>
        }

        impl<$T> $i<$T>
        // yo, this is convenient
        $(where $($wh)*)?
        {
            // and so now, we use first part of a macro
            $v2 fn new($( $arg: $ArgTy ),*) -> Self {
                Self{ inner: $inner::def($( $arg ),*) }
            }
        }

        // Hack
        // Create a module with name of (ex. inner0) with def fn in it.
        mod $inner {
            // lol, why to allow them, if they are useless?
            #![allow(type_alias_bounds)]

            #[allow(unused_imports)]
            use super::{*, $i as _};

            #[cfg(feature = "nightly")]
            // creating a type (ex Send<U>)
            pub(crate) type $i<$T>
            $(where $($wh)*)? = impl ::core::future::Future<Output = $Out>;

            #[cfg(feature = "nightly")]
            pub(crate) fn def<$T>($( $arg: $ArgTy ),*) -> $i<$T>
            $(where $($wh)*)?
            {
                $body
            }

            #[cfg(not(feature = "nightly"))]
            pub(crate) type $i<$T>
            $(where $($wh)*)? = ::core::pin::Pin<Box<dyn ::core::future::Future<Output = $Out> + ::core::marker::Send + 'static>>;

            #[cfg(not(feature = "nightly"))]
            pub(crate) fn def<$T>($( $arg: $ArgTy ),*) -> $i<$T>
            $(where $($wh)*)?
            {
                Box::pin($body)
            }
        }

        impl<$T> ::core::future::Future for $i<$T>
        $(where $($wh)*)?
        {
            type Output = $Out;

            fn poll(self: ::core::pin::Pin<&mut Self>, cx: &mut ::core::task::Context<'_>) -> ::core::task::Poll<Self::Output> {
                let this = self.project();
                this.inner.poll(cx)
            }
        }

    };
}

// macro_rules! req_future {
//     (
//         $v2:vis def: | $( $arg:ident: $ArgTy:ty ),* $(,)? | $body:block

//         $(#[$($meta:tt)*])*
//         $v:vis $i:ident<$T:ident> ($inner:ident) -> $Out:ty
//         $(where $($wh:tt)*)?
//     ) => {
//         #[pin_project::pin_project]
//         pub
//         // FIXME(waffle):
//         // The `pub` above should ideally be `$v`, but we currently can't do
//         // this due to compiler bug, see:
//         // - pin_project bug report <https://github.com/taiki-e/pin-project/issues/312>
//         // - related rustc issue <https://github.com/rust-lang/rust/issues/81007>
//         // - original fix (closed) <https://github.com/rust-lang/rust/pull/81029>
//         // - second iteration of the fix <https://github.com/rust-lang/rust/pull/81177>
//         struct $i<$T>
//         $(where $($wh)*)?
//         {
//             #[pin]
//             inner: $inner::$i<$T>
//         }

//         impl<$T> $i<$T>
//         $(where $($wh)*)?
//         {
//             $v2 fn new($( $arg: $ArgTy ),*) -> Self {
//                 Self { inner: $inner::def($( $arg ),*) }
//             }
//         }

//         // HACK(waffle): workaround for https://github.com/rust-lang/rust/issues/55997
//         mod $inner {
//             #![allow(type_alias_bounds)]

//             // Mostly to bring `use`s
//             #[allow(unused_imports)]
//             use super::{*, $i as _};

//             #[cfg(feature = "nightly")]
//             pub(crate) type $i<$T>
//             $(where $($wh)*)? = impl ::core::future::Future<Output = $Out>;

//             #[cfg(feature = "nightly")]
//             pub(crate) fn def<$T>($( $arg: $ArgTy ),*) -> $i<$T>
//             $(where $($wh)*)?
//             {
//                 $body
//             }

//             #[cfg(not(feature = "nightly"))]
//             pub(crate) type $i<$T>
//             $(where $($wh)*)?  = ::core::pin::Pin<Box<dyn ::core::future::Future<Output = $Out> + ::core::marker::Send + 'static>>;

//             #[cfg(not(feature = "nightly"))]
//             pub(crate) fn def<$T>($( $arg: $ArgTy ),*) -> $i<$T>
//             $(where $($wh)*)?
//             {
//                 Box::pin($body)
//             }
//         }

//         impl<$T> ::core::future::Future for $i<$T>
//         $(where $($wh)*)?
//         {
//             type Output = $Out;

//             fn poll(self: ::core::pin::Pin<&mut Self>, cx: &mut ::core::task::Context<'_>) -> ::core::task::Poll<Self::Output> {
//                 let this = self.project();
//                 this.inner.poll(cx)
//             }
//         }

//     };
// }

pub(crate) use req_future;

#[macro_export]
macro_rules! parent_error {
    (
        $vis:vis enum $name:ident {
            $(
                $variant:ident($child:ty),
            )*
        }
    ) => {
        #[derive(Debug, PartialEq, Eq, Clone)]
        $vis enum $name {
            $(
                $variant($child),
            )*
        }

        impl $crate::common::traits::Cause for $name {
            fn cause(&self) -> Option<&str> {
                match self {
                    $(
                        Self::$variant(err) => err.cause(),
                    )*
                }
            }
        }

        impl $crate::common::traits::Code for $name {
            fn code(&self) -> &str {
                match self {
                    $(
                        Self::$variant(err) => err.code(),
                    )*
                }
            }
        }

        impl $crate::common::traits::ErrorTypeProvider for $name {
            fn error_type(&self) -> $crate::common::traits::ErrorType {
                match self {
                    $(
                        Self::$variant(err) => err.error_type(),
                    )*
                }
            }
        }

        $(
            impl ::std::convert::From<$child> for $name {
                fn from(value: $child) -> Self {
                    Self::$variant(value)
                }
            }
        )*
    };
}

#[macro_export]
macro_rules! leaf_error {
    (
        $vis:vis enum $name:ident {
            $(
                $variant:ident => ($str:expr, $err:expr),
            )*
        }
    ) => {
        #[derive(Debug, PartialEq, Eq, Clone)]
        $vis enum $name {
            $(
                $variant,
            )*
        }

        impl $crate::common::traits::Code for $name {
            fn code(&self) -> &str {
                match self {
                    $(
                        Self::$variant => $str,
                    )*
                }
            }
        }

        impl $crate::common::traits::Cause for $name {
            fn cause(&self) -> Option<&str> {
                match self {
                    $(
                        Self::$variant => None,
                    )*
                }
            }
        }

        impl $crate::common::traits::ErrorTypeProvider for $name {
            fn error_type(&self) -> $crate::common::traits::ErrorType {
                match self {
                    $(
                        Self::$variant => $err,
                    )*
                }
            }
        }
    };
}

#[macro_export]
macro_rules! leaf_detail_error {
    (
        $vis:vis enum $name:ident {
            $(
                $variant:ident => ($str:expr, $err:expr),
            )*
        }
    ) => {
        #[derive(Debug, PartialEq, Eq, Clone)]
        $vis enum $name {
            $(
                $variant(String),
            )*
        }

        impl $crate::common::traits::Code for $name {
            fn code(&self) -> &str {
                match self {
                    $(
                        Self::$variant(_) => $str,
                    )*
                }
            }
        }

        impl $crate::common::traits::Cause for $name {
            fn cause(&self) -> Option<&str> {
                match self {
                    $(
                        Self::$variant(err) => Some(err),
                    )*
                }
            }
        }
        
        impl $crate::common::traits::ErrorTypeProvider for $name {
            fn error_type(&self) -> $crate::common::traits::ErrorType {
                match self {
                    $(
                        Self::$variant(_) => $err,
                    )*
                }
            }
        }
    };
}

#[macro_export]
macro_rules! create_page {
    ( $($module:ident :: $page:ident $(,)?)* ) => {

        use seed::prelude::*;
        use std::any::Any;

        mod page_trait;
        pub use page_trait::PageTrait;

        $(
            mod $module;
            pub use $module::$page;
        )*

        pub enum Page {
            $(
                $page($page),
            )*
        }

        type Message = Box<dyn Any>;

        impl Page {

            pub fn update(&mut self, msg: Message, orders: &mut impl Orders<Message>) {
                match self {
                    $(
                        Self::$page(page) => page.invoke_update(msg, orders),
                    )*
                }
            }

            pub fn view(&self) -> Vec<Node<Message>> {
                match self {
                    $(
                        Self::$page(page) => page.invoke_view(),
                    )*
                }
            }
        }

        $(
            impl From<$page> for Page {
                fn from(page: $page) -> Self {
                    Self::$page(page)
                }
            }
        )*
    };
}

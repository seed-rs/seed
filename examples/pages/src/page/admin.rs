use crate::Context;
use seed::{prelude::*, *};

const REPORT: &str = "report";

mod page;

// ------ ------
//     Init
// ------ ------

pub fn init(mut url: Url) -> Option<Model> {
    Some(Model {
        report_page: match url.pop_relative_path_part() {
            Some(REPORT) => page::report::init(url)?,
            _ => None?,
        },
    })
}

// ------ ------
//     Model
// ------ ------

pub struct Model {
    report_page: page::report::Model,
}

// ------ ------
//     Urls
// ------ ------

struct_urls!();
impl<'a> Urls<'a> {
    pub fn report_urls(self) -> page::report::Urls<'a> {
        page::report::Urls::new(self.base_url().push_path_part(REPORT))
    }
}

// ------ ------
//     View
// ------ ------

pub fn view<Ms>(model: &Model, ctx: &Context) -> Node<Ms> {
    page::report::view(&model.report_page, ctx)
}

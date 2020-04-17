use crate::Context;
use seed::{prelude::*, *};

const REPORT: &str = "report";

mod page;

// ------ ------
//     Init
// ------ ------

pub fn init(mut url: Url, model: &mut Option<Model>) -> Option<()> {
    let model = model.get_or_insert_with(Model::default);
    model.page_id.replace(match url.next_path_part() {
        Some(REPORT) => page::report::init(url, &mut model.report_model).map(|_| PageId::Report)?,
        _ => None?,
    });
    Some(())
}

// ------ ------
//     Model
// ------ ------

#[derive(Default)]
pub struct Model {
    page_id: Option<PageId>,
    report_model: Option<page::report::Model>,
}

// ------ PageId ------

#[derive(Copy, Clone, Eq, PartialEq)]
enum PageId {
    Report,
}

// ------ ------
//     Urls
// ------ ------

struct_urls!();
impl<'a> Urls<'a> {
    pub fn report_urls(self) -> page::report::Urls<'a> {
        page::report::Urls::new(self.base_url().add_path_part(REPORT))
    }
}

// ------ ------
//     View
// ------ ------

#[allow(clippy::single_match_else)]
pub fn view<Ms>(model: &Model, ctx: &Context) -> Node<Ms> {
    match model.page_id {
        Some(PageId::Report) => {
            page::report::view(model.report_model.as_ref().expect("report model"), ctx)
        }
        None => div!["404"],
    }
}

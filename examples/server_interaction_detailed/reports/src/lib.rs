#[macro_use]
extern crate seed;
use seed::prelude::*;

use futures::Future;
use serde::Deserialize;

use shared::interfaces::{Gradesheet, Line, Mission, Person, Syllabus, Upgrade, UpgradeEvent};
use shared::{datetime, util};

struct Model {
    lines: Vec<Line>,
    people: Vec<Person>,
    syllabi: Vec<Syllabus>,
    upgrades: Vec<Upgrade>,
    events: Vec<UpgradeEvent>,
    gradesheets: Vec<Gradesheet>,
    instructors: Vec<Person>,
    missions: Vec<Mission>,
    mx_start: datetime::Date,
    mx_end: datetime::Date,
}

impl Default for Model {
    fn default() -> Self {
        // todo way to take adv of Default::default for all but dates to reduce clutter?
        Self {
            lines: Vec::new(),
            people: Vec::new(),
            syllabi: Vec::new(),
            upgrades: Vec::new(),
            events: Vec::new(),
            gradesheets: Vec::new(),
            instructors: Vec::new(),
            missions: Vec::new(),
            mx_start: datetime::Date::new(1999, 9, 9),
            mx_end: datetime::Date::new(1999, 9, 9),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
struct ServerData {
    lines: Vec<Line>,
    people: Vec<Person>,

    syllabi: Vec<Syllabus>,
    upgrades: Vec<Upgrade>,
    events: Vec<UpgradeEvent>,
    gradesheets: Vec<Gradesheet>,
    instructors: Vec<Person>,
    missions: Vec<Mission>,
}

fn get_data() -> impl Future<Item = Msg, Error = Msg> {
    let url = "/get-reports-data/";

    seed::Request::new(url)
        .method(seed::Method::Get)
        .fetch_json()
        .map(Msg::LoadInitial)
        .map_err(Msg::OnFetchErr)
}

#[derive(Clone)]
enum Msg {
    GetData,
    LoadInitial(ServerData),
    OnFetchErr(JsValue),

    ChangeMxStart(datetime::Date),
    ChangeMxEnd(datetime::Date),
}

fn update(msg: Msg, model: &mut Model) -> Update<Msg> {
    match msg {
        Msg::GetData => Update::with_future_msg(get_data()).skip(),

        Msg::LoadInitial(data) => {
            model.lines = data.lines;
            model.people = data.people;
            model.missions = data.missions;

            Render.into()
        }

        Msg::OnFetchErr(err) => {
            log!(format!("Fetch error: {:?}", err)); // todo switch to error! once new seed version released
            Skip.into()
        }

        Msg::ChangeMxStart(date) => {
            model.mx_start = date;
            Render.into()
        }

        Msg::ChangeMxEnd(date) => {
            model.mx_end = date;
            Render.into()
        }
    }
}

fn gradesheets(model: &Model) -> El<Msg> {
    section![]
}

fn mx_effectivity(
    lines: &Vec<Line>,
    mx_start: &datetime::Date,
    mx_end: &datetime::Date,
) -> El<Msg> {
    //    let lines_filtered = lines.iter().filter(l )
    let lines_filtered = lines;

    let filter_format = |v| {
        let v: Vec<&Line> = lines_filtered
            .iter()
            .filter(|l| l.mx_effective == util::MX_EFF)
            .collect();
        v.len()
    };

    let eff = filter_format(util::MX_EFF);
    let ne_wx = filter_format(util::MX_EFF);
    let ne_ops = filter_format(util::MX_EFF);
    let ne_mx = filter_format(util::MX_EFF);
    let ne_unk = filter_format(util::MX_EFF);

    let percent = |num: usize| {
        let val = match lines_filtered.len() {
            0 => num as f32 / lines_filtered.len() as f32,
            _ => 0.,
        };
        unit!(val * 100., %)
    };

    let margin_style = style! {"margin-right" => px(60)};

    let display_block = |val: usize, title: &str| {
        div![
            style! {"display" => "flex"},
            h3![&margin_style, title],
            h3![&margin_style, val.to_string()],
            h3![percent(val)],
        ]
    };

    section![
        style! {"margin-bottom" => px(100)},
        h2!["Maintenance line effectivity"],
        div![
            style! {"display" => "flex"; "flex-direction" => "column"},
            div![
                style! {"display" => "flex"; "margin-bottom" => px(60)},
                h3!["Start"],
                h3!["End"],
            ],
            display_block(eff, "Effective:"),
            display_block(ne_wx, "Non-effective weather:"),
            display_block(ne_ops, "Non-effective ops:"),
            display_block(ne_mx, "Non-effective maintenance:"),
            display_block(ne_unk, "Unknown:"),
        ]
    ]
}

fn display_pct<T>(part: &Vec<T>, whole: &Vec<T>) -> String {
    if whole.len() > 0 {
        (100 * part.len() / whole.len()).to_string()
    } else {
        "0".into()
    }
}

fn display_pct2<T>(len: usize, whole: &Vec<T>) -> String {
    // Alternate variant.
    if whole.len() > 0 {
        (100 * len / whole.len()).to_string()
    } else {
        "0".into()
    }
}

fn sortie_types(lines: &Vec<Line>, people: &Vec<Person>, missions: &Vec<Mission>) -> El<Msg> {
    let lookback_days = 60; // todo make adjustable
    let today = datetime::Date::today();

    let min_date = today.sub(chrono::Duration::days(lookback_days));

    let mut rows: Vec<El<Msg>> = Vec::new();

    //    log!(format!("{:?}", today));

    for person in people.iter().filter(|p| util::is_aircrew(*p)) {
        // todo person_lines is much too verbose.

        let person_lines: Vec<&Line> = lines
            .into_iter()
            .filter(|l| {
                let l_date = datetime::Date::from_iso(&l.date);

                if let Some(pilot_id) = l.pilot {
                    let pilot = people.iter().find(|p| p.id == pilot_id);
                    //                        .expect("Can't find pilot");
                    //                if pilot.id == person.id && min_date <= l_date && l_date <= today {
                    if let Some(pilot) = pilot {
                        //                        if pilot.id == person.id {
                        if pilot.id == person.id && min_date <= l_date && l_date <= today {
                            return true;
                        }
                    }
                }
                // todo DRY
                if let Some(wso_id) = l.wso {
                    let wso = people.iter().find(|p| p.id == wso_id);
                    //                        .expect("Can't find WSO");
                    //                if wso.id == person.id && min_date <= l_date && l_date <= today {
                    if let Some(wso) = wso {
                        //                        if wso.id == person.id {
                        if wso.id == person.id && min_date <= l_date && l_date <= today {
                            return true;
                        }
                    }
                }
                false
            })
            .collect();

        let red_air: Vec<&Line> = person_lines
            .clone() // todo sloppy clone
            .into_iter()
            .filter(|l| {
                if let Some(mission_id) = l.mission2 {
                    let mission = missions
                        .iter()
                        .find(|m| m.id == mission_id)
                        .expect("Can't find mission");
                    if mission.name.to_lowercase() == "red air" {
                        return true;
                    }
                }
                false
            })
            .collect();

        let mut upgrade: Vec<&Line> = Vec::new();
        for line in person_lines.clone() {
            // todo sloppy clone
            let mut found_one = false;
            for form_line in util::formation_lines(line, lines) {
                if form_line.upgrade || form_line.wso_upgrade || form_line.upgrade_event.is_some() {
                    upgrade.push(line);
                    found_one = true;
                    break;
                }
            }
            if found_one {
                break;
            }
        }

        rows.push(tr![
            style! {"text-align" => "center"},
            td![style! {"text-align" => "left"}, util::short_name(person)],
            td![display_pct(&red_air, &person_lines)],
            td![display_pct(&upgrade, &person_lines)],
            td![display_pct2(
                &person_lines.len() - &red_air.len() - &upgrade.len(),
                &person_lines
            )],
        ]);
    }

    section![
        style! {"margin-bottom" => px(100)},
        h2!["Sortie types (last 60 days)"],
        table![
            class![
                "table",
                "table-striped",
                "table-condensed",
                "table-responsive"
            ],
            thead![tr![
                th!["Person"],
                th!["% Red air"],
                th!["% Upgrade"],
                th!["% CT"],
            ]],
            tbody![rows],
        ]
    ]
}

fn view(model: &Model) -> Vec<El<Msg>> {
    vec![
        h2!["(WIP below)"],
        gradesheets(model),
        mx_effectivity(&model.lines, &model.mx_start, &model.mx_end),
        sortie_types(&model.lines, &model.people, &model.missions),
    ]
}

#[wasm_bindgen]
pub fn render() {
    let state = seed::App::build(Model::default(), update, view)
        .finish()
        .run();

    state.update(Msg::GetData)
}

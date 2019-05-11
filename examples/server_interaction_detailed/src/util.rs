use crate::interfaces::{Line, Person};

// @TODO: edit this, it's just placeholder so project can be build
pub const MX_EFF: i8 = 0;

const PILOT_JOB: i8 = 0;
const WSO_JOB: i8 = 1;

pub fn short_name(person: &Person) -> String {
    format! {"{}, {}", person.last_name, person.first_name}
    // todo first initial.
}

fn includes_line(lines: &Vec<Line>, line: &Line) -> bool {
    // Helper function that determines if a line's within another's set.
    for l in lines {
        if l.id == line.id {
            return true;
        }
    }
    return false;
}

pub fn formation_lines(selected_line: &Line, lines: &Vec<Line>) -> Vec<Line> {
    // todo sloppy clone
    let mut sorted_lines = lines.clone();
    sorted_lines.sort_by(|a, b| a.number.cmp(&b.number));

    let mut current_form = Vec::new();
    for line in sorted_lines.into_iter() {
        if line.flight_lead {
            // We've found the start of a new form. Was the last one our result?
            if includes_line(&current_form, selected_line) {
                return current_form;
            }
            // New formation start; wipe the result. The last one wasn't it.
            current_form = vec![line.clone()];
        } else {
            current_form.push(line.clone());
        }
    }
    // We've hit the last line and haven't found a result yet. Is it the last form?
    if includes_line(&current_form, selected_line) {
        return current_form;
    }
    return current_form;
}

pub fn is_aircrew(person: &Person) -> bool {
    person.job == PILOT_JOB || person.job == WSO_JOB
}

use crate::commands::CommandResponse;

pub fn emit(response: CommandResponse) {
    if response.json {
        println!("{}", response.json_body);
    } else {
        println!("{}", response.human_body);
    }
}

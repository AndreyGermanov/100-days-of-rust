#[derive(PartialEq)]
enum DoorStatuses { OPENING, CLOSING, OPENED, CLOSED }

fn play_with_door(commands: Vec<&str>) -> Vec<String> {
    commands.iter().fold((DoorStatuses::CLOSED,vec!["Door: CLOSED".to_string()]),
                         |(mut status, mut result), command| {
        match *command {
            "button_clicked" => {
                result.push("> Button clicked.".to_string());
                match status {
                    DoorStatuses::CLOSED => {
                        result.push("Door: OPENING".to_string());
                        status = DoorStatuses::OPENING;
                    },
                    DoorStatuses::OPENED => {
                        result.push("Door: CLOSING".to_string());
                        status = DoorStatuses::CLOSING;
                    },
                    DoorStatuses::OPENING => {
                        result.push("Door: STOPPED_WHILE_OPENING".to_string());
                        status = DoorStatuses::OPENED;
                    },
                    DoorStatuses::CLOSING => {
                        result.push("Door: STOPPED_WHILE_CLOSING".to_string());
                        status = DoorStatuses::CLOSED;
                    }
                }
            }
            "cycle_complete" => {
                result.push("> Cycle complete.".to_string());
                match status {
                    DoorStatuses::OPENING => {
                        result.push("Door: OPEN".to_string());
                        status = DoorStatuses::OPENED
                    },
                    DoorStatuses::CLOSING => {
                        result.push("Door: CLOSED".to_string());
                        status = DoorStatuses::CLOSED
                    },
                    _ => {}
                }
            }
            _ => {}
        }
        (status, result)
    }).1

}

#[test]
fn test_play_with_door() {
    assert_eq!(play_with_door(vec![
        "button_clicked", "cycle_complete", "button_clicked", "button_clicked", "button_clicked",
        "button_clicked", "button_clicked", "cycle_complete"
    ]),
    vec![
        "Door: CLOSED", "> Button clicked.", "Door: OPENING", "> Cycle complete.",
        "Door: OPEN", "> Button clicked.", "Door: CLOSING", "> Button clicked.",
        "Door: STOPPED_WHILE_CLOSING", "> Button clicked.", "Door: OPENING", "> Button clicked.",
        "Door: STOPPED_WHILE_OPENING", "> Button clicked.", "Door: CLOSING", "> Cycle complete.", "Door: CLOSED",
    ])
}

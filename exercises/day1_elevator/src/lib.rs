pub type Floor = i32;

#[derive(Debug, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Button {
    LobbyCall(Direction, Floor),
    CarFloor(Floor),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Event {
    ButtonPressed(Button),
    CarArrived(Floor),
    CarDoorOpened,
    CarDoorClosed,
}

pub fn car_arrived(floor: Floor) -> Event {
    Event::CarArrived(floor)
}

pub fn car_door_opened() -> Event {
    Event::CarDoorOpened
}

pub fn car_door_closed() -> Event {
    Event::CarDoorClosed
}

pub fn lobby_call_button_pressed(floor: Floor, dir: Direction) -> Event {
    Event::ButtonPressed(Button::LobbyCall(dir, floor))
}

pub fn car_floor_button_pressed(floor: Floor) -> Event {
    Event::ButtonPressed(Button::CarFloor(floor))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_elevator_events() {
        let e1 = lobby_call_button_pressed(0, Direction::Up);
        let e2 = car_arrived(0);
        assert_eq!(e1, Event::ButtonPressed(Button::LobbyCall(Direction::Up, 0)));
        assert_eq!(e2, Event::CarArrived(0));
    }
}

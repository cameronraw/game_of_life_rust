use crate::cell_state::CellState;

pub struct Cell {
    state: CellState
}

impl Cell {
    pub fn new_live_cell() -> Self {
        Cell{
            state: CellState::ALIVE
        }
    }

    pub fn new_dead_cell() -> Self {
        Cell{
            state: CellState::DEAD
        }
    }

    pub fn calculate_new_state(&mut self, live_neighbours: u8) {
        if live_neighbours == 3 {
            self.state = CellState::ALIVE;
            return;
        }
        if live_neighbours < 2 || live_neighbours > 3 {
            self.state = CellState::DEAD;
        }
    }
}

#[cfg(test)]
mod cell_should {
    use super::*;
    #[test]
    fn be_initialized_with_live_state(){
        let _live_cell = Cell::new_live_cell();
        assert_eq!(_live_cell.state, CellState::ALIVE);
    }
    #[test]
    fn be_initialized_with_dead_state(){
        let _dead_cell = Cell::new_dead_cell();
        assert_eq!(_dead_cell.state, CellState::DEAD);
    }
}

#[cfg(test)]
mod live_cell_should {
    use super::*;
    #[test]
    fn die_with_fewer_than_two_live_neighbours(){
        let mut _live_cell = Cell::new_live_cell();
        _live_cell.calculate_new_state(1);
        assert_eq!(_live_cell.state, CellState::DEAD);
    }
    #[test]
    fn live_with_two_or_three_live_neighbours(){
        let mut _live_cell = Cell::new_live_cell();
        _live_cell.calculate_new_state(2);
        assert_eq!(_live_cell.state, CellState::ALIVE);
        _live_cell.calculate_new_state(3);
        assert_eq!(_live_cell.state, CellState::ALIVE);
    }
    #[test]
    fn die_with_more_than_three_live_neighbours(){
        let mut _live_cell = Cell::new_live_cell();
        _live_cell.calculate_new_state(4);
        assert_eq!(_live_cell.state, CellState::DEAD);
    }
}

#[cfg(test)]
mod dead_cell_should {
    use super::*;
    #[test]
    fn revive_with_three_live_neighbours(){
        let mut _dead_cell = Cell::new_dead_cell();
        _dead_cell.calculate_new_state(3);
        assert_eq!(_dead_cell.state, CellState::ALIVE);
    }
}

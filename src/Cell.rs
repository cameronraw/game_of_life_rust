use crate::CellState::CellState;

pub struct Cell {
    state: CellState
}

impl Cell {
    pub fn new(initial_cell_state: CellState) -> Self {
        Cell{
            state: initial_cell_state
        }
    }
}

#[cfg(test)]
mod cell_should {
    use super::*;
    #[test]
    fn be_initialized_with_state(){
        let _live_cell = Cell::new(CellState::ALIVE);
        assert_eq!(_live_cell.state, CellState::ALIVE);
    }
}

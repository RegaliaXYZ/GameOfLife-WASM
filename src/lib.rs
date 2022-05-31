mod random;
mod gameoflife;

use gameoflife::*;
use std::{cell::RefCell};
use wasm_bindgen::{prelude::* };

thread_local! {
  static GAME: RefCell<GameOfLife> =
    RefCell::new(GameOfLife::new(10, 10, 50));

}

#[wasm_bindgen(js_name=newGame)]
pub fn create_new_game(width: usize, height: usize, percentage: usize) {
  GAME.with(|game| {
    *game.borrow_mut() = GameOfLife::new(width, height, percentage);
  });
}

#[wasm_bindgen(js_name=getState)]
pub fn get_state() -> String {
  GAME.with(|ms| ms.borrow().to_string())
}

#[wasm_bindgen(js_name=getNextState)]
pub fn get_next_state() -> String {
  GAME.with(|ms| ms.borrow_mut().tick());
  GAME.with(|ms| ms.borrow().to_string())
}
#[wasm_bindgen(js_name=toggleState)]
pub fn toggle_state(x: usize, y: usize) {
  GAME.with(|ms| ms.borrow_mut().toggle_state_of_cell((x, y)));
}
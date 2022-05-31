mod random;
mod gameoflife;

use js_sys::Function;
use gameoflife::{GameOfLife};
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::{prelude::*, JsCast, UnwrapThrowExt};
use web_sys::{window, HtmlDivElement, HtmlElement};

thread_local! {
  static GAME: Rc<RefCell<GameOfLife>> =
    Rc::new(RefCell::new(GameOfLife::new(10, 10, 50)));

  static HANDLE_TICK: Closure<dyn FnMut()> = Closure::wrap(Box::new(|| {
    GAME.with(|game| game.borrow_mut().tick());
    render();
  }) as Box<dyn FnMut()>);

}

#[wasm_bindgen(start)]
pub fn main() {
  HANDLE_TICK.with(|tick_closure| {
    window()
      .unwrap_throw()
      .set_interval_with_callback_and_timeout_and_arguments_0(
        tick_closure.as_ref().dyn_ref::<Function>().unwrap_throw(),
        200,
      )
      .unwrap_throw()
  });

  render();
}

pub fn render() {
  GAME.with(|game| {
    let game = game.borrow();
    let document = window().unwrap_throw().document().unwrap_throw();
    let root_container = document
      .get_element_by_id("root")
      .unwrap_throw()
      .dyn_into::<HtmlElement>()
      .unwrap_throw();

    root_container.set_inner_html("");

    let width = game.width;
    let height = game.height;

    root_container
      .style()
      .set_property("display", "inline-grid")
      .unwrap_throw();
    root_container
      .style()
      .set_property(
        "grid-template",
        &format!("repeat({}, auto) / repeat({}, auto)", height, width),
      )
      .unwrap_throw();

    for y in 0..height {
      for x in 0..width {
        let pos = (x, y);
        let field_element = document
          .create_element("div")
          .unwrap_throw()
          .dyn_into::<HtmlDivElement>()
          .unwrap_throw();

        field_element.set_class_name("field");

        field_element.set_inner_text({
          if game.alive_fields.contains(&pos) {
            "🟩"
          } else {
            "⬜"
          }
        });

        root_container.append_child(&field_element).unwrap_throw();
      }
    }
  });
}
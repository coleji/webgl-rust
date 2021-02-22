extern crate wasm_bindgen;
extern crate web_sys;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext;

#[wasm_bindgen]
pub fn main() -> () {
	let document = web_sys::window().unwrap().document().unwrap();
	let canvas = document.get_element_by_id("canvas").unwrap().dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

	let gl: web_sys::WebGlRenderingContext = 
		canvas
		.get_context("webgl")
		.unwrap()
		.unwrap()
		.dyn_into::<WebGlRenderingContext>()
		.unwrap();

	gl.viewport(0, 0, gl.drawing_buffer_width(), gl.drawing_buffer_height());
	// Set the clear color.
	gl.clear_color(0.0, 0.0, 0.4, 1.0);
	// Clear the context with the newly set color. This is
	// the function call that actually does the drawing.
	gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

	log("hi from rust!")
}

#[wasm_bindgen]
extern "C" {
	// Use `js_namespace` here to bind `console.log(..)` instead of just
	// `log(..)`
	#[wasm_bindgen(js_namespace = console)]
	fn log(s: &str);

	// The `console.log` is quite polymorphic, so we can bind it with multiple
	// signatures. Note that we need to use `js_name` to ensure we always call
	// `log` in JS.
	#[wasm_bindgen(js_namespace = console, js_name = log)]
	fn log_u32(a: u32);

	// Multiple arguments too!
	#[wasm_bindgen(js_namespace = console, js_name = log)]
	fn log_many(a: &str, b: &str);
}

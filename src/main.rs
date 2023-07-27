/*

	TXTASTIC - A simple pastebin made using Rocket.
	Made by Pedro Cabral, 2023.

*/

#[macro_use]
extern crate rocket;

// Configure this size to your liking!
// This will be the size (in ASCII chars) of the IDs inside `/upload`.
const ID_SIZE: usize = 8;
const MAX_FILE_SIZE_KB: usize = 1024;

// A Responder that sends file data with a Content-Type based on its file extension.
// use rocket::fs::NamedFile;

// A data guard for FromForm types.
// use rocket::form::Form;

// An empty redirect response to a given URL.
use rocket::response::Redirect;

// This is a specialized version of `std::fs::File` for usage from the Tokio runtime.
use rocket::tokio::{fs::File, io::AsyncReadExt};

// Responder that renders a dynamic template.
use rocket_dyn_templates::{Template, context};

// Rocket `Type` and `Trait` for fancy operations.
use rocket::data::{Data, ToByteUnit};

// Local modules.
mod txt;
mod rng;

// This is the `ID` of a Txt that will be stored. 
use txt::TXT;

#[get("/")]
async fn route_get_index() -> Template {
	
	// Serve the template.
    Template::render("index", context! {})

}

#[get("/<txt>")]
async fn route_get_txt(txt: TXT) -> Option<Template> {
	
	// Try to get the file based on the `txt`.
	let mut file = api_get_txt(txt.clone()).await?;

	// This is the buffer that will hold the string.
	let mut buffer = String::new();

	// Read the file contents as a String.
	let _ = file.read_to_string(&mut buffer).await;

	// Check if it starts with `txt=`, if so, remove.
	if buffer.starts_with("txt=") {
		buffer = buffer.replacen("txt=", "", 1)
	}

	// Serve the template with the `id` and `content` inside.
    Some(Template::render("txt", context! { txt: buffer, id: txt.id() }))

}

#[get("/api/txt/<id>")]
async fn api_get_txt(id: TXT) -> Option<File> {

	// Try to open a file.
	// This is completely safe against `Full Path Disclosure`.
    File::open(id.file_path()).await.ok()

}

#[post("/api/txt", data = "<txt>")]
async fn api_post_txt(txt: Data<'_>) -> std::io::Result<Redirect> {
	
	// Create a new `TXT`.
	let id = TXT::new(ID_SIZE);

	// Create the file based on the data, but capped.
	txt.open(MAX_FILE_SIZE_KB.kilobytes()).into_file(id.file_path()).await?;

	// Redirect to the `TXT` URI.
	Ok(Redirect::to("/".to_string() + &id.id()))

}

#[launch]
fn rocket() -> _ {

	// Creates a Rocket instance with the default config provider.
	rocket::build()
		
		// Returns a fairing that initializes and maintains templating state.
		.attach(Template::fairing())
		
		// Mounts all of the routes in the supplied routes![...]
		.mount("/", routes![route_get_index, route_get_txt, api_get_txt, api_post_txt])

}
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
use std::io::Write;
use std::io;
use std::fs::{self, File};
use std::path::Path;
use rocket::build;

/*
This Api will help us create and update notes
*/
#[post("/create-update/<id>", data = "<content>")]
fn create_or_update_note(id: String, content: String) -> io::Result<String> {
    write_note(&id, &content)
}
/*
Function behind create/update Api
*/
fn write_note(id: &str, content: &str) -> io::Result<String> {
    let path = Path::new("notes/").join(id);

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let mut file = File::create(&path)?;
    file.write_all(content.as_bytes())?;

    Ok(format!("Note {} created/updated successfully", id))
}

/*
This Api will help us read notes
*/
#[get("/read-notes")]
fn get_notes() -> io::Result<String> {
    read_notes()
}
/*
Function behind read Api
*/
fn read_notes() -> io::Result<String> {
    let mut notes = String::new();
    let path = Path::new("notes/");

    if path.exists() && path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let file_path = entry.path();
            let note_content = fs::read_to_string(&file_path)?;
            notes.push_str(&note_content);
            notes.push_str("\n---\n");
        }
    }

    Ok(notes)
}

/*
This Api will help us delete notes
*/
#[post("/delete-notes/<id>")]
fn delete_note(id: String) -> io::Result<String> {
    delete_file(&id)
}
/*
Function behind delete Api
*/
fn delete_file(id: &str) -> io::Result<String> {
    let path = Path::new("notes/").join(id);
    fs::remove_file(path)?;

    Ok(format!("Note {} deleted successfully", id))
}

#[tokio::main]
async fn main() {
    let custom_welcome_message = "Welcome to My Rocket Server!";
    println!("{}",custom_welcome_message);
    build()
        .mount("/", routes![get_notes, create_or_update_note, delete_note])
        .launch()
        .await
        .expect("Rocket failed to launch");
}

#[cfg(test)]
mod tests {
    use super::*;
    use rocket::local::blocking::Client;
    use rocket::http::{Status};
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_all_operations() {
        // Create note
        let rocket = build().mount("/", routes![create_or_update_note]);
        let client = Client::tracked(rocket).expect("valid rocket instance");

        let response = client
            .post("/create-update/test_note")
            .body("Test note content")
            .dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.into_string().expect("response body"),
            "Note test_note created/updated successfully"
        );
       

        //delay
        thread::sleep(Duration::from_secs(1));

        // Read notes
        let rocket = build().mount("/", routes![get_notes]);
        let client = Client::tracked(rocket).expect("valid rocket instance");

        let response = client.get("/read-notes").dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.into_string().expect("response body"),
            "Test note content\n---\n"
        );
    
        // delay 
        thread::sleep(Duration::from_secs(1));

        // Delete note
        let rocket = build().mount("/", routes![delete_note]);
        let client = Client::tracked(rocket).expect("valid rocket instance");

        let response = client.post("/delete-notes/test_note").dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.into_string().expect("response body"),
            "Note test_note deleted successfully"
        );
     
    }
}

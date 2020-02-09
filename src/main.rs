fn index() {
    //
}
fn main() {

    rocket::ignite()
      .mount("/hello", routes![hello])
      .launch();
}

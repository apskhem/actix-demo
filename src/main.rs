#[macro_use] extern crate actix_web;
#[macro_use] extern crate serde_json;

use std::fs;
use std::env;
use std::io::Result;
use std::io::prelude::*;
use actix_web::{App, HttpResponse, HttpServer, Responder};
use actix_web::web::{Path, Json};
use actix_files;
use serde::{Serialize, Deserialize};
mod gpax;

#[derive(Serialize, Deserialize)]
struct Course {
    courseId: u32,
    courseName: String,
    credit: u8,
    gpa: f32,
}

#[get("/")]
async fn index() -> Result<impl Responder> {
    let html = fs::read_to_string("static/index.html")?;

    return Ok(HttpResponse::Ok().body(html));
}

#[get("/instruction")]
async fn instruction() -> Result<impl Responder> {
    let html = fs::read_to_string("static/instruction.html")?;

    return Ok(HttpResponse::Ok().body(html));
}

#[get("/courses")]
async fn get_courses() -> Result<impl Responder> {
    let my_courses = fs::read_to_string("myCourses.json")?;

    return Ok(HttpResponse::Ok().body(my_courses));
}

#[get("/courses/{id}")]
async fn get_courses_id(Path(id): Path<u64>) -> Result<impl Responder> {
    let my_courses = fs::read_to_string("myCourses.json")?;
    let json: serde_json::Value = serde_json::from_str(my_courses.as_str())?;
    let course_arr = &json["courses"];

    for course in course_arr.as_array().unwrap().iter() {
        if id == course["courseId"].as_u64().unwrap() {
            let c = course.to_string();
            return Ok(HttpResponse::Ok().body(c));
        }
    }

    return Ok(HttpResponse::NotFound().body("Error 404: file was not found"));
}

#[delete("/courses/{id}")]
async fn delete_courses_id(Path(id): Path<u64>) -> Result<impl Responder> {
    let my_courses = fs::read_to_string("myCourses.json")?;
    let json: serde_json::Value = serde_json::from_str(my_courses.as_str())?;
    let course_arr = &json["courses"];

    // filter out the requested id
    let filtered_arr = course_arr.as_array().unwrap().iter().filter(|x| id != x["courseId"].as_u64().unwrap()).collect();

    // group json
    let res = json!({ "success": true, "data": filtered_arr });
    let sav = json!({ "courses": filtered_arr, "gpax": gpax::cal_gpax(&filtered_arr) });

    // overwrite old file
    let mut file = fs::File::create("myCourses.json")?;
    file.write_all(sav.to_string().as_bytes())?;

    return Ok(HttpResponse::Ok().body(res.to_string()));
}

#[post("/addCourse")]
async fn add_course(course: Json<Course>) -> Result<impl Responder> {
    let my_courses = fs::read_to_string("myCourses.json")?;
    let json: serde_json::Value = serde_json::from_str(my_courses.as_str())?;
    let course_arr = &json["courses"];
    
    // format req body
    let formatted_course_json = json!(course.into_inner());

    // create new arr
    let mut new_arr = vec![];
    for course in course_arr.as_array().unwrap().iter() {
        new_arr.push(course);
    }

    new_arr.push(&formatted_course_json);

    // group json
    let res_text = json!({ "success": true, "data": formatted_course_json });
    let sav = json!({ "courses": new_arr, "gpax": gpax::cal_gpax(&new_arr) });

    // overwrite old file
    let mut file = fs::File::create("myCourses.json")?;
    file.write_all(sav.to_string().as_bytes())?;

    return Ok(HttpResponse::Created().body(res_text.to_string()));
}

#[actix_web::main]
async fn main() -> Result<()> {
    let port = env::var("PORT")
    .unwrap_or_else(|_| String::from("3000"))
    .parse()
    .unwrap();

    let app = || {
        App::new()
        .service(index)
        .service(instruction)
        .service(get_courses)
        .service(get_courses_id)
        .service(delete_courses_id)
        .service(add_course)
        .service(actix_files::Files::new("/", "./static").show_files_listing())
    };

    println!("serving at {}:{}", "0.0.0.0", port);

    return HttpServer::new(app)
    .bind(("0.0.0.0", port))?
    .run()
    .await;
}
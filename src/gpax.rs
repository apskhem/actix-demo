pub fn cal_gpax(courses: &Vec<&serde_json::Value>) -> f64 {
    let mut weight_sum = 0.0;
    let mut weight = 0.0;

    for course in courses.iter() {
        let cre = course["credit"].as_f64().unwrap();
        let gpa = course["gpa"].as_f64().unwrap();

        weight_sum += cre * gpa;
        weight += cre;
    }

    return if weight == 0.0 { 0.0 } else { weight_sum / weight };
}
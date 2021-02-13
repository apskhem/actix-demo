window.onload = () => {
    process();
}

async function process() {
    let { data } = await axios.get("/courses");

    renderForm(data);

    const courseId = document.getElementById("courseId");
    const courseName = document.getElementById("courseName");
    const credit = document.getElementById("credit");
    const gpa = document.getElementById("gpa");
    const form = document.getElementById("form");

    form.addEventListener("submit", (e) => {
        e.preventDefault();

        const d = {
            courseId: courseId.valueAsNumber,
            courseName: courseName.value,
            credit: credit.valueAsNumber,
            gpa: gpa.valueAsNumber,
        };
        
        const headers = { "content-type": "application/json" };        

        axios.post("/addCourse", d, { headers });

        data.courses.push(d);

        // rerender
        form.reset();
        renderForm(data);
    });
}

function renderForm(data) {
    const tbody = document.getElementById("item-container");

    // clear old data
    while (tbody.firstElementChild) {
        tbody.firstElementChild.remove();
    }

    // render new data
    data.courses.forEach((course, i) => {
        const tr = document.createElement("tr");
        const th = document.createElement("th");
        const td1 = document.createElement("td");
        const td2 = document.createElement("td");
        const td3 = document.createElement("td");
        const td4 = document.createElement("td");
        const td5 = document.createElement("td");
        const btn = document.createElement("button");

        th.setAttribute("scope", "row");
        th.textContent = `${i + 1}`;
        td1.textContent = `${course.courseId}`;
        td2.textContent = `${course.courseName}`;
        td3.textContent = `${course.credit}`;
        td4.textContent = `${course.gpa}`;

        btn.classList.add("btn", "btn-danger", "btn-sm");
        btn.textContent = "Delete";

        btn.addEventListener("click", () => {
            axios.delete(`/courses/${course.courseId}`);

            data.courses = data.courses.filter((x) => +x.courseId !== +course.courseId);

            // rerender
            renderForm(data);
        });
        
        td5.append(btn);
        tr.append(th, td1, td2, td3, td4, td5);
        tbody.append(tr);
    });

    calculateGPAX(data.courses);
}

function calculateGPAX(courses) {
    const card = document.getElementById("gpax");

    const [ weightSum, weight ] = courses.reduce((acc, course) => {
        acc[0] += +course.gpa * +course.credit;
        acc[1] += +course.credit;

        return acc;
    }, [ 0, 0 ]);

    card.textContent = `GPAX: ${(weight ? weightSum / weight : 0).toFixed(2)}`;
}
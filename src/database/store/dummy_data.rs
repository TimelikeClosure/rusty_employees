use super::super::store::Store;

const DATA: [(&str, [&str; 10]); 7] = [
    ("Accounting", [
        "Liyah Meadows",
        "Miranda Roche",
        "Kaci Costa",
        "Kirk Short",
        "Olivia-Mae Schneider",
        "Anil Mcgregor",
        "Benjamin Cotton",
        "Madison Wyatt",
        "Shyam Calderon",
        "Kurtis Pollard",
    ]),
    ("Design", [
        "Atif Wells",
        "Caspar Whitney",
        "Ibrar Bloom",
        "Kayden Serrano",
        "Ariyan James",
        "Zahraa Marriott",
        "Stefanie Healy",
        "Fatema Garrison",
        "Mary Spence",
        "Keavy Barnard",
    ]),
    ("Engineering", [
        "Alejandro Heath",
        "Gaia Floyd",
        "Alessandra Cresswell",
        "Hubert Farley",
        "Jordanna Allman",
        "Samiha Yoder",
        "Wendy Flowers",
        "Lacey-May Hatfield",
        "Leela Tomlinson",
        "Corey Baldwin",
    ]),
    ("Logistics", [
        "Zara Dupont",
        "Kingsley Calderon",
        "Betty Tierney",
        "Annaliese Russo",
        "Arron Thatcher",
        "Monty Turnbull",
        "Mehreen Ortiz",
        "Darin Redman",
        "Kain Burt",
        "Selina Chase",
    ]),
    ("Production", [
        "Connie Bowen",
        "Faizaan Lindsay",
        "Zayan Gentry",
        "Asa Mccormack",
        "Miya Conroy",
        "Wilfred Albert",
        "Giacomo Malone",
        "Alissa Mccarthy",
        "Lee Oakley",
        "Kie Slater",
    ]),
    ("Purchasing", [
        "Brogan Benjamin",
        "Dominik Pittman",
        "Alaw Munoz",
        "Fatima Huang",
        "Rahul Bush",
        "Lowri Griffiths",
        "Eshan Morrow",
        "Aayan Rich",
        "Sufyaan Sellers",
        "Lacey Prentice",
    ]),
    ("Sales", [
        "Waseem Guerrero",
        "Mayson Krueger",
        "Cadi Moses",
        "Abbas Peters",
        "Dawid Bowen",
        "Riaz Hull",
        "Sahib Mcgrath",
        "Catrin Leon",
        "Aleyna Markham",
        "Elouise Guest",
    ]),
];

pub fn populate(store: &mut Store) {
    DATA.iter()
    .for_each(|(department_name, employees)| {
        store
            .departments()
            .create(&department_name.to_string())
            .expect(format!("Dummy data failed to populate on forming department \"{}\"", department_name).as_str());
        let department = store
            .department(department_name)
            .expect(format!("Dummy data failed to retrieve department \"{}\"", department_name).as_str());
        employees.iter()
            .for_each(|employee_name| {
                department
                    .assign(employee_name)
                    .expect(format!("Dummy data failed to populate on assigning employee \"{}\" to department \"{}\"", employee_name, department_name).as_str());
            });
    });
}

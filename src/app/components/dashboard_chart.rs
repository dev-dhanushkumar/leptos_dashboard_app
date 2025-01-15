use crate::app::{components::DashboardWidget, models::Person};
use leptos::*;
use num_format::{Buffer, Locale};
use std::rc::Rc;
use charts_rs:: { BarChart, Series, Color, THEME_DARK};


#[component]
pub fn DashboardChart(persons_data: Vec<Person>) -> impl IntoView {

    // create a reference counting pointer to our actual persons data so
    //Rust doesn't need to create/clone copies of the actual data everytime
    let retrieved_persons_data = Rc::new(persons_data.clone());

    // for counting the total number of team members
    let team_count: String = retrieved_persons_data.len().to_string();

    //for calculating and adding the  total cost for all the team members
    let mut total_cost:i32 = 0;

    //for identifying who is the latest to join
    let mut latest_number: String = String::new();
    let mut counter: i32 = 0;

    // 2 vectors for displaying the titles and the other for counting
    // the quality/number for each title
    let mut data_vec = Vec::new();
    let mut count_vec = Vec::new();

    //loop through the returned data
    for person in persons_data {
        if counter == 0 {
            latest_number = person.name;
        }

        //adding this personcomponsation to the total team cost
        total_cost += person.compensation;

        // if the vector for displaying the title doesn't continue this person's
        //title yet
        if !data_vec.contains(&person.title) {
            // we add it to the vector
            data_vec.push(person.title);

            // we also add 1 to the count vector
            count_vec.push(1.0);
        } else {
            //if this title has previously been added to the titles vector
            // we get the index of the title in the titles vector
            let index = data_vec
                .iter()
                .position(|title| title == &person.title)
                .unwrap();

            // we also get the number in the vector using that index
            let num_at_index = count_vec[index];

            // then we increment it by 1
            count_vec[index] = num_at_index + 1.0;
        }

        counter += 1;
    }

    let mut bar_series = Series::new(String::new(), count_vec);
    bar_series.label_show = true;

    let mut bar_chart = BarChart::new_with_theme(vec![bar_series], data_vec, THEME_DARK);
    bar_chart.font_family = String::from("Noto Sans SC");
    bar_chart.background_color = Color::transparent();
    bar_chart.width = 832.0;
    bar_chart.height = 500.0;

    // to not show the y-axis with the decimal point number for count
    bar_chart.y_axis_hidden = true;

    // to convert the total cost to a string using the num-format crate's Buffer
    let mut buf = Buffer::default();
    buf.write_formatted(&total_cost, &Locale::en);
    let total_cost_str = format!("${}", buf.as_str());

    view! {
        <div class="w-full flex flex-col max-w-[64rem] mx-auto pt-8 mb-10">
            <div class="w-full h-20 grid grid-cols-3 gap-4 mx-auto px-2 max-w-[53rem]">
                <DashboardWidget title="Team Members" value=&team_count />
                <DashboardWidget title="Monthly Team Cost" value=&total_cost_str />
                <DashboardWidget title="Just Joined" value=&latest_number />
            </div>

            <div class="max-w-[54rem] mx-auto w-full flex flex-col mt-14 pb-12">
                <div class="w-full max-w-[41rem] h-20 bg-black-200 rounded py-10 px-4 pb-10"
                    inner_html=&bar_chart.svg().unwrap()>
                </div>
            </div>
        </div>
    }
}
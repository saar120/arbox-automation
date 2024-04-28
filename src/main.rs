use dotenv::dotenv;
use std::time::Duration;
use clokwerk::{Job, Scheduler, TimeUnits};
use log::{error, info};
use crate::utils::date_time::{get_day, get_day_before};

mod api;
mod users;
mod utils;

#[tokio::main]
async fn main() {
    dotenv().ok();
    utils::logger::setup_logger();

    tokio::spawn(async {
        utils::health_check::start_tcp_health_check_server().await;
    });

    info!("Starting the scheduler");

    let mut scheduler = Scheduler::with_tz(chrono_tz::Asia::Jerusalem);

    let users = users::users::load_users_from_json();

    // iterate over users and schedule their classes
    for user in users {
        /**
         * 1. Check user.box_info.lessons_open_type
         * 2. If it's TwentyFourHours, will sign the user to class 24 hours before the class
         * 3 If it's FixedWeekly, will sign the user to the entire schedule on a single chosen day - TODO
         */

        match user.box_info.lessons_open_type {
            users::models::LessonsOpenType::TwentyFourHours => {
                for schedule in user.schedule {
                    let day = get_day_before(&schedule.day).unwrap();
                    scheduler.every(day).at(&*schedule.time).run(move || {
                        let user_id = user.id;
                        info!("starting scheduling for user {}", user_id);
                        tokio::spawn(async move { // this is needed to run async code inside the scheduler
                            users::users::sign_user_to_class(user_id).await;
                        });
                        info!("schedule class for user {} finished", user_id);
                    });
                    info!("Created a job user {} for class on {:?} at {}", user.id, day, schedule.time);
                }
            }
            _ => {
                error!("Unsupported lessons_open_type");
            }
        }
    };

    info!("All users scheduled - waiting for jobs");

    loop {
        scheduler.run_pending();
        tokio::time::sleep(Duration::from_millis(1000)).await;
    }
}



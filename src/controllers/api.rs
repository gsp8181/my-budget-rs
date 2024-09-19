use std::str::FromStr;

use chrono::Local;
use rocket::{fairing::AdHoc, serde::json::Json};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use crate::{
    models::item::{JsonObject, PublicItem},
    services::apiservice::{
        calculate, end_of_week, full_weekend, get_items_today, net_saved_avg, remaining_week,
        saved_this_year, sum_of_card_held, sum_of_credits, sum_of_debits,
    },
    services::{itemstore::get_collection, settingsstore::get_setting},
    Db,
};

pub async fn test_data(db: Db) -> PublicItem {
    let data_from_db = get_collection(&db).await;
    let mut results: Vec<JsonObject> = Vec::new();
    for object in data_from_db {
        results.push(JsonObject::from(object));
    }

    //TODO: browsers time??
    let now = Local::now();

    //TODO: tuple
    let payday: u32 =
        u32::from_str(&get_setting(&db, String::from("payday"), String::from("25")).await)
            .expect("failed to read payday setting");
    let daily_rate: Decimal =
        Decimal::from_str(&get_setting(&db, String::from("dailyRate"), String::from("0")).await)
            .expect("failed to read dailyRate setting");
    let total_pay =
        Decimal::from_str(&get_setting(&db, String::from("pay"), String::from("0")).await)
            .expect("failed to read pay setting");
    let weekday_saving = Decimal::from_str(
        &get_setting(&db, String::from("weekdaySaving"), String::from("0")).await,
    )
    .expect("failed to read weekdaySaving setting");

    let calc_to_eom = &get_setting(&db, String::from("calc_to_eom"), String::from("true")).await;
    let calc_to_eom = if calc_to_eom == &String::from("true") {
        true
    } else if calc_to_eom == &String::from("false") {
        false
    } else {
        panic!("failed to read calc_to_eom setting")
    };

    PublicItem {
        amount: calculate(
            &results,
            &now,
            daily_rate,
            payday,
            weekday_saving,
            calc_to_eom,
        ),
        remaining_week: remaining_week(
            &results,
            &now,
            daily_rate,
            payday,
            weekday_saving,
            calc_to_eom,
        ),
        end_of_week: end_of_week(
            &results,
            &now,
            daily_rate,
            payday,
            weekday_saving,
            calc_to_eom,
        ),
        full_weekend: full_weekend(
            &results,
            &now,
            daily_rate,
            payday,
            weekday_saving,
            calc_to_eom,
        ),
        monthly_debits: sum_of_debits(&results),
        monthly_credits: sum_of_credits(&results, total_pay),
        net_saved_this_month: dec!(-1),
        card_held_total: sum_of_card_held(&results),
        net_saved_avg: net_saved_avg(&results, daily_rate, total_pay),
        saved_this_year: saved_this_year(&results, daily_rate, total_pay),
        today: get_items_today(&results, &now),
    }
}

#[get("/")]
async fn index(db: Db) -> Json<PublicItem> {
    Json(test_data(db).await)
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("API Stage", |rocket| async {
        rocket.mount("/api", routes![index])
    })
}

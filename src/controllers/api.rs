use std::str::FromStr;

use axum::{
    extract::{Query, State},
    Json,
};
use chrono::{DateTime, Local, NaiveDate, TimeZone};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};

use crate::{
    models::item::{JsonObject, PublicItem},
    services::apiservice::{
        calculate, end_of_week, full_weekend, get_items_between, get_items_today, net_saved_avg,
        remaining_week, saved_this_year, sum_of_card_held, sum_of_credits, sum_of_debits,
    },
    services::{
        currencystore::build_currency_rate_map, itemstore::get_collection,
        settingsstore::get_setting,
    },
    DbPool,
};

#[derive(Deserialize)]
struct ApiQuery {
    date: Option<String>,
}

async fn build_public_item(pool: &DbPool, target_date: Option<DateTime<Local>>) -> PublicItem {
    let data_from_db = get_collection(pool).await;
    let mut results: Vec<JsonObject> = Vec::new();
    for object in data_from_db {
        results.push(JsonObject::from(object));
    }

    let real_now = Local::now();
    let now = target_date.unwrap_or(real_now);

    let currency_rates = build_currency_rate_map(pool).await;

    //TODO: tuple
    let payday: u32 =
        u32::from_str(&get_setting(pool, String::from("payday"), String::from("25")).await)
            .expect("failed to read payday setting");
    let daily_rate: Decimal =
        Decimal::from_str(&get_setting(pool, String::from("dailyRate"), String::from("0")).await)
            .expect("failed to read dailyRate setting");
    let total_pay =
        Decimal::from_str(&get_setting(pool, String::from("pay"), String::from("0")).await)
            .expect("failed to read pay setting");
    let weekday_saving = Decimal::from_str(
        &get_setting(pool, String::from("weekdaySaving"), String::from("0")).await,
    )
    .expect("failed to read weekdaySaving setting");

    let calc_to_eom_raw =
        get_setting(pool, String::from("calc_to_eom"), String::from("true")).await;
    let calc_to_eom = if calc_to_eom_raw == "true" {
        true
    } else if calc_to_eom_raw == "false" {
        false
    } else {
        panic!("failed to read calc_to_eom setting")
    };

    let calc_following_month_raw =
        get_setting(pool, String::from("calc_following_month"), String::from("false")).await;
    let calc_following_month = if calc_following_month_raw == "true" {
        true
    } else if calc_following_month_raw == "false" {
        false
    } else {
        panic!("failed to read calc_following_month setting")
    };

    PublicItem {
        amount: calculate(
            &results,
            &currency_rates,
            &now,
            daily_rate,
            payday,
            weekday_saving,
            calc_to_eom,
            calc_following_month,
        ),
        remaining_week: remaining_week(
            &results,
            &currency_rates,
            &now,
            daily_rate,
            payday,
            weekday_saving,
            calc_to_eom,
            calc_following_month,
        ),
        end_of_week: end_of_week(
            &results,
            &currency_rates,
            &now,
            daily_rate,
            payday,
            weekday_saving,
            calc_to_eom,
            calc_following_month,
        ),
        full_weekend: full_weekend(
            &results,
            &currency_rates,
            &now,
            daily_rate,
            payday,
            weekday_saving,
            calc_to_eom,
            calc_following_month,
        ),
        monthly_debits: sum_of_debits(&results, &currency_rates),
        monthly_credits: sum_of_credits(&results, &currency_rates, total_pay),
        net_saved_this_month: dec!(-1),
        card_held_total: sum_of_card_held(&results, &currency_rates),
        net_saved_avg: net_saved_avg(&results, &currency_rates, daily_rate, total_pay),
        saved_this_year: saved_this_year(&results, &currency_rates, daily_rate, total_pay),
        today: if target_date.is_some() {
            get_items_between(&results, &currency_rates, &real_now, &now)
        } else {
            get_items_today(&results, &currency_rates, &now)
        },
    }
}

async fn index(
    State(pool): State<DbPool>,
    Query(params): Query<ApiQuery>,
) -> Json<PublicItem> {
    let target_date: Option<DateTime<Local>> = params.date.and_then(|d| {
        NaiveDate::parse_from_str(&d, "%Y-%m-%d")
            .ok()
            .and_then(|nd| {
                Local
                    .from_local_datetime(&nd.and_hms_opt(0, 0, 0).unwrap())
                    .single()
            })
    });
    Json(build_public_item(&pool, target_date).await)
}

#[derive(Serialize)]
struct UserInfo {
    username: String,
    email: String,
}

async fn me() -> Json<UserInfo> {
    // Stubbed values for now
    let info = UserInfo {
        username: String::from("gsp8181"),
        email: String::from("gsp8181@github"),
    };
    Json(info)
}

pub fn router() -> axum::Router<DbPool> {
    axum::Router::new()
        .route("/", axum::routing::get(index))
        .route("/me", axum::routing::get(me))
}

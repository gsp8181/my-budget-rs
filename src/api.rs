use std::str::FromStr;

use bigdecimal::{BigDecimal, FromPrimitive};
use chrono::{DateTime, Datelike, Local, Months, NaiveDate, TimeZone};
use rocket::fairing::AdHoc;
use rocket::serde::json::Json;

use crate::structs::{
    Category, DBObj, Db_Name, PublicItem, DAILY_RATE, PAYDAY, TOTAL_PAY_BOTTOM, TOTAL_PAY_TOP, WEEKDAY_SAVING,
};

use crate::store::get_collection as get_test_data;

fn test_data() -> PublicItem {
    let data = get_test_data();

    return PublicItem {
        amount: calculate(&data),
        remaining_week: remaining_week(&data),
        end_of_week: end_of_week(&data),
        full_weekend: full_weekend(&data),
        monthly_debits: sum_of_debits(&data),
        monthly_credits: sum_of_credits(&data),
        net_saved_this_month: BigDecimal::from_i32(-1).unwrap(),
        card_held_total: sum_of_card_held(&data),
        net_saved_avg: net_saved_avg(&data),
        saved_this_year: saved_this_year(&data),
        today: get_items_today(&data),
    };
}

fn get_items_today(data: &[DBObj]) -> Vec<DBObj> {
    let today1 = DBObj {
        id: Some(59),
        oldId: None,
        category: Category::recurring,
        name: String::from("Applecare"),
        day: Some(4),
        amount: BigDecimal::from_i32(-10).unwrap(),
        cardid: None,
        dbName: Db_Name::debit,
    };

    vec![today1]
}

fn saved_this_year(data: &[DBObj]) -> BigDecimal {
    let total = (sum_of_credits(data) - sum_of_debits(data)) * BigDecimal::from_u8(12).unwrap();

    let daily_total = BigDecimal::from_i32(DAILY_RATE).unwrap() * BigDecimal::from_str(&"365.25").unwrap();

    total - daily_total
    //var dailyTotal = dailyRate * (new DateTime(DateTime.Now.Year, 12, 31)).DayOfYear;
}

fn net_saved_avg(data: &[DBObj]) -> BigDecimal {
    //TODO: not 31

    let total = sum_of_credits(data) - sum_of_debits(data);

    let daily_total = BigDecimal::from_i32(DAILY_RATE).unwrap() * BigDecimal::from_u8(31).unwrap();

    total - daily_total
}

fn sum_of_card_held(data: &[DBObj]) -> BigDecimal {
    let mut amount = BigDecimal::from_u8(0).unwrap();
    for bank_obj in data {
        if let DBObj {
            dbName: Db_Name::credit,
            category: Category::creditcard,
            ..
        } = bank_obj
        {
            amount += bank_obj.amount.clone();
        }
    }
    amount
}

fn sum_of_credits(data: &[DBObj]) -> BigDecimal {
    let mut amount = BigDecimal::from_u8(0).unwrap();
    for bank_obj in data {
        if let DBObj {
            dbName: Db_Name::credit,
            day: Some(_),
            ..
        } = bank_obj
        {
            amount += bank_obj.amount.clone();
        }
    }
    amount + BigDecimal::from_str(&format!("{}.{}", TOTAL_PAY_TOP, TOTAL_PAY_BOTTOM)).unwrap()
}

fn sum_of_debits(data: &[DBObj]) -> BigDecimal {
    let mut amount = BigDecimal::from_u8(0).unwrap();
    for bank_obj in data {
        match bank_obj {
            DBObj {
                category: Category::creditcard,
                ..
            } => (),
            DBObj {
                dbName: Db_Name::debit,
                day: Some(_),
                ..
            } => {
                amount += bank_obj.amount.clone();
            }
            _ => (),
        }
    }
    amount
}

fn calculate(data: &Vec<DBObj>) -> BigDecimal {
    let mut amount = BigDecimal::from_u8(0).unwrap();

    let now = Local::now();

    let mut next_payday = match Local.with_ymd_and_hms(now.year(), now.month(), PAYDAY, 0, 0, 0) {
        chrono::offset::LocalResult::Single(single) => single,
        _ => panic!("Time zone error"), //todo: 500
    };

    let after_payday = now.day() >= PAYDAY;

    if after_payday {
        next_payday = next_payday + Months::new(1);
    }
    let end_of_month_for_payday = match Local.with_ymd_and_hms(
        next_payday.year(),
        next_payday.month(),
        get_days_from_month(next_payday.year(), next_payday.month()),
        0,
        0,
        0,
    ) {
        chrono::offset::LocalResult::Single(single) => single,
        _ => panic!("Time zone error"), //todo: 500
    };

    let days_to_calculate = match BigDecimal::from_i64((end_of_month_for_payday - now).num_days() + 1) //TODO: what 
    {
        Some(result) => result,
        None => panic!("Calc error"),
    };

    let daily_rate_to_debit = days_to_calculate * BigDecimal::from_i32(DAILY_RATE).unwrap();

    amount -= daily_rate_to_debit;

    let weekday = weekday(now);

    if weekday < BigDecimal::from_u8(5).unwrap() {
        amount -= BigDecimal::from_i32(WEEKDAY_SAVING).unwrap() * (weekday); //TODO: weekday
    }

    for bank_obj in data {
        match bank_obj {
            DBObj {
                dbName: Db_Name::credit,
                ..
            } => {
                if can_be_used_in_calculation(&bank_obj, &now, &next_payday, after_payday) {
                    amount += bank_obj.amount.clone()
                }
            }
            DBObj {
                dbName: Db_Name::debit,
                ..
            } => {
                if can_be_used_in_calculation(&bank_obj, &now, &next_payday, after_payday) {
                    amount -= bank_obj.amount.clone()
                }
            }
        }
    }
    amount
}

fn can_be_used_in_calculation(
    record: &DBObj,
    now: &DateTime<Local>,
    next_payday: &DateTime<Local>,
    after_payday: bool,
) -> bool {
    if record.day.is_none() {
        return true;
    }

    match record.category {
        Category::cardbalance => return true,
        Category::creditcard => return true,
        _ => (),
    }

    let date_obj: DateTime<Local> = if get_days_from_month(now.year(), now.month())
        >= record.day.unwrap() as u32
    //TODO: days in year month
    {
        match Local.with_ymd_and_hms(now.year(), now.month(), record.day.unwrap() as u32, 0, 0, 0) {
            chrono::offset::LocalResult::Single(single) => single,
            _ => panic!("Time zone error"), //todo: 500
        }
    } else {
        match Local.with_ymd_and_hms(now.year(), now.month() + 1, 1, 0, 0, 0) {
            chrono::offset::LocalResult::Single(single) => single,
            _ => panic!("Time zone error"), //todo: 500
        }
    };

    if after_payday && (record.day.unwrap() as u32) < next_payday.day() {
        return true;
    }

    if now < &date_obj && &date_obj < next_payday {
        return true;
    }

    return false;
}

fn remaining_week(data: &Vec<DBObj>) -> BigDecimal {
    // This calculates the total to sunday, so adding the daily rate back in and the weekend savings
    let mut amount = calculate(data);

    let weekday = weekday(Local::now());

    if weekday < BigDecimal::from_u8(5).unwrap() {
        let weekday_amount_to_debit = BigDecimal::from_i32(WEEKDAY_SAVING).unwrap() * (weekday.clone());
        amount += weekday_amount_to_debit;
    }

    amount += BigDecimal::from_i32(DAILY_RATE).unwrap() * (BigDecimal::from_u8(7).unwrap() - weekday);

    amount
}

fn end_of_week(data: &Vec<DBObj>) -> BigDecimal {
    //If friday was today, this is what the total would be
    //Add back in the weekday savings and the fridays daily rate
    let mut amount = calculate(data);

    let weekday = weekday(Local::now());

    if weekday < BigDecimal::from_u8(5).unwrap() {
        let weekday_amount_to_debit = BigDecimal::from_i32(WEEKDAY_SAVING).unwrap() * BigDecimal::from_u8(4).unwrap();
        amount += weekday_amount_to_debit + BigDecimal::from_i32(DAILY_RATE).unwrap();
    } else if weekday > BigDecimal::from_u8(5).unwrap() {
        return BigDecimal::from_u8(0).unwrap(); //TODO: maybe a different way
    }
    amount
}

fn weekday(time: DateTime<Local>) -> BigDecimal {
    //TODO: convert
    match BigDecimal::from_i32(time.weekday() as i32) {
        Some(result) => result + BigDecimal::from_u8(1).unwrap(),
        None => panic!("Calc error"),
    }
}

fn full_weekend(data: &Vec<DBObj>) -> BigDecimal {
    // I have no idea what this does
    // TODO: investigate
    // Takes off the difference between the daily rate and the weekday savings between now and the weekend?
    let mut amount = remaining_week(data);

    let weekday = weekday(Local::now());

    if weekday < BigDecimal::from_u8(5).unwrap() {
        amount -= (BigDecimal::from_u8(5).unwrap() - weekday) * (BigDecimal::from_i32(DAILY_RATE).unwrap() -  BigDecimal::from_i32(WEEKDAY_SAVING).unwrap());
    }

    amount
}

#[get("/")]
async fn index() -> Json<PublicItem> {
    let user1 = test_data();

    Json(user1)
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("API Stage", |rocket| async {
        rocket.mount("/api", routes![index])
    })
}

pub fn get_days_from_month(year: i32, month: u32) -> u32 {
    let result = NaiveDate::from_ymd(
        match month {
            12 => year + 1,
            _ => year,
        },
        match month {
            12 => 1,
            _ => month + 1,
        },
        1,
    )
    .signed_duration_since(NaiveDate::from_ymd(year, month, 1))
    .num_days();

    match i64::try_into(result) {
        Ok(u32) => u32,
        Err(e) => 28,
    }
}

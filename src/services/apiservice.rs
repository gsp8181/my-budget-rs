use chrono::{DateTime, Datelike, Local, Months, NaiveDate, TimeZone};
use rust_decimal::{prelude::FromPrimitive, Decimal};
use rust_decimal_macros::dec;

use crate::models::item::{Category, Db_Name, JsonObject};

pub fn get_items_today(data: &[JsonObject], now: &DateTime<Local>) -> Vec<JsonObject> {
    let mut results: Vec<JsonObject> = Vec::new();

    let mut dates: Vec<i32> = vec![now.day() as i32];

    if now.day() == 1 {
        let month = now.month();
        if month == 10 || month == 5 || month == 7 || month == 12 {
            dates.push(31);
        } else if month == 3 {
            dates.push(31);
            dates.push(30);
            if get_days_from_month(now.year(), 2) < 29 {
                dates.push(29);
            }
        }
    }

    data.iter()
        .filter(|x| x.day.is_some())
        .filter(|x| dates.contains(&x.day.unwrap()))
        .cloned()
        .for_each(|mut x| {
            if let Db_Name::debit = x.dbName {
                x.amount = -x.amount;
            }
            results.push(x);
        });

    results
}

pub fn saved_this_year(data: &[JsonObject], daily_rate: Decimal, total_pay: Decimal) -> Decimal {
    let total = (sum_of_credits(data, total_pay) - sum_of_debits(data)) * dec!(12);

    let daily_total = daily_rate * dec!(365.25);

    total - daily_total
    //var dailyTotal = dailyRate * (new DateTime(DateTime.Now.Year, 12, 31)).DayOfYear;
}

pub fn net_saved_avg(data: &[JsonObject], daily_rate: Decimal, total_pay: Decimal) -> Decimal {
    //TODO: not 31

    let total = sum_of_credits(data, total_pay) - sum_of_debits(data);

    let daily_total = daily_rate * dec!(31);

    total - daily_total
}

pub fn sum_of_card_held(data: &[JsonObject]) -> Decimal {
    let mut amount = dec!(0);
    for bank_obj in data {
        if let JsonObject {
            dbName: Db_Name::credit,
            category: Category::creditcard,
            ..
        } = bank_obj
        {
            amount += bank_obj.amount;
        }
    }
    amount
}

pub fn sum_of_credits(data: &[JsonObject], total_pay: Decimal) -> Decimal {
    let mut amount = dec!(0);
    for bank_obj in data {
        if let JsonObject {
            dbName: Db_Name::credit,
            day: Some(_),
            ..
        } = bank_obj
        {
            amount += bank_obj.amount;
        }
    }
    amount + total_pay
}

pub fn sum_of_debits(data: &[JsonObject]) -> Decimal {
    let mut amount = dec!(0);
    for bank_obj in data {
        match bank_obj {
            JsonObject {
                category: Category::creditcard,
                ..
            } => (),
            JsonObject {
                dbName: Db_Name::debit,
                day: Some(_),
                ..
            } => {
                amount += bank_obj.amount;
            }
            _ => (),
        }
    }
    amount
}

pub fn calculate(
    data: &Vec<JsonObject>,
    now: &DateTime<Local>,
    daily_rate: Decimal,
    payday: u32,
    weekday_saving: Decimal,
    calc_to_eom: bool,
) -> Decimal {
    let mut amount = dec!(0.0);

    let payday = if payday <= get_days_from_month(now.year(), now.month()) {
        payday
    } else {
        get_days_from_month(now.year(), now.month())
    };

    //TODO: split this off into can be used for calculation?
    let mut next_payday = match Local.with_ymd_and_hms(now.year(), now.month(), payday, 0, 0, 0) {
        chrono::offset::LocalResult::Single(single) => single,
        _ => panic!("Time zone error"), //todo: 500
    };

    let after_payday = now.day() >= payday;

    if after_payday {
        next_payday = next_payday + Months::new(1);
    }

    let end_of_month_for_payday = match Local.with_ymd_and_hms(
        next_payday.year(),
        next_payday.month(),
        if calc_to_eom {
            get_days_from_month(next_payday.year(), next_payday.month())
        } else {
            payday
        },
        0,
        0,
        0,
    ) {
        chrono::offset::LocalResult::Single(single) => single,
        _ => panic!("Time zone error"), //todo: 500
    };

    let days_to_calculate = match Decimal::from_i64((end_of_month_for_payday - now).num_days() + 1) //TODO: what 
    {
        Some(result) => result,
        None => panic!("Calc error"),
    };

    let daily_rate_to_debit = days_to_calculate * daily_rate;

    amount -= daily_rate_to_debit;

    let weekday = weekday(now);

    if weekday < dec!(5) {
        amount -= weekday_saving * (weekday); //TODO: weekday
    }

    for bank_obj in data {
        match bank_obj {
            JsonObject {
                dbName: Db_Name::credit,
                ..
            } => {
                if can_be_used_in_calculation(bank_obj, now, &next_payday, after_payday) {
                    amount += bank_obj.amount;
                }
            }
            JsonObject {
                dbName: Db_Name::debit,
                ..
            } => {
                if can_be_used_in_calculation(bank_obj, now, &next_payday, after_payday) {
                    amount -= bank_obj.amount;
                }
            }
        }
    }
    amount
}

pub fn can_be_used_in_calculation(
    record: &JsonObject,
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
        >= (record.day.unwrap() as u32)
    //TODO: panics if -1?
    //TODO: days in year month
    {
        match Local.with_ymd_and_hms(now.year(), now.month(), record.day.unwrap() as u32, 0, 0, 0) {
            //TODO: panics if -1?
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
        //TODO: panics if -1?
        return true;
    }

    if now < &date_obj && &date_obj < next_payday {
        return true;
    }

    false
}

pub fn remaining_week(
    data: &Vec<JsonObject>,
    now: &DateTime<Local>,
    daily_rate: Decimal,
    payday: u32,
    weekday_saving: Decimal,
    calc_to_eom: bool,
) -> Decimal {
    // This calculates the total to sunday, so adding the daily rate back in and the weekend savings
    let mut amount = calculate(data, now, daily_rate, payday, weekday_saving, calc_to_eom);

    let weekday = weekday(now);

    if weekday < dec!(5) {
        let weekday_amount_to_debit = weekday_saving * (weekday);
        amount += weekday_amount_to_debit;
    }

    amount += daily_rate * (dec!(7) - weekday);

    amount
}

pub fn end_of_week(
    data: &Vec<JsonObject>,
    now: &DateTime<Local>,
    daily_rate: Decimal,
    payday: u32,
    weekday_saving: Decimal,
    calc_to_eom: bool,
) -> Decimal {
    //If friday was today, this is what the total would be
    //Add back in the weekday savings and the fridays daily rate
    let mut amount = calculate(data, now, daily_rate, payday, weekday_saving, calc_to_eom);

    let weekday = weekday(now);

    if weekday < dec!(5) {
        let weekday_amount_to_debit = weekday_saving * dec!(4);
        amount += weekday_amount_to_debit + daily_rate;
    } else if weekday > dec!(5) {
        return dec!(0); //TODO: maybe a different way
    }
    amount
}

fn weekday(time: &DateTime<Local>) -> Decimal {
    //TODO: convert
    match Decimal::from_u32(time.weekday() as u32) {
        Some(result) => result + dec!(1),
        None => panic!("Calc error"),
    }
}

pub fn full_weekend(
    data: &Vec<JsonObject>,
    now: &DateTime<Local>,
    daily_rate: Decimal,
    payday: u32,
    weekday_saving: Decimal,
    calc_to_eom: bool,
) -> Decimal {
    // I have no idea what this does
    // TODO: investigate
    // Takes off the difference between the daily rate and the weekday savings between now and the weekend?
    let mut amount = remaining_week(data, now, daily_rate, payday, weekday_saving, calc_to_eom);

    let weekday = weekday(now);

    if weekday < dec!(5) {
        amount -= (dec!(5) - weekday) * (daily_rate - weekday_saving);
    }

    amount
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

    //TODO: make this have better error handling
    match i64::try_into(result) {
        Ok(u32) => u32,
        Err(e) => 28,
    }
}

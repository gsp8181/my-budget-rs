use chrono::{DateTime, Datelike, Days, Months, NaiveDate, TimeZone};
use rust_decimal::{prelude::FromPrimitive, Decimal};
use rust_decimal_macros::dec;
use std::collections::HashMap;

use crate::models::item::{Category, Db_Name, JsonObject};

/// Converts an item's amount to GBP using the currency rate map.
/// Rate of 1 (GBP) means no conversion. Rate of e.g. 0.77 means amount / 0.77.
pub fn apply_currency_conversion(item: &mut JsonObject, currency_rates: &HashMap<i32, Decimal>) {
    if let Some(cid) = item.currency_id {
        if let Some(&rate) = currency_rates.get(&cid) {
            if rate != dec!(0) && rate != dec!(1) {
                item.amount = item.amount / rate;
            }
        }
    }
}

pub fn get_items_today<Z: TimeZone>(data: &[JsonObject], currency_rates: &HashMap<i32, Decimal>, now: &DateTime<Z>) -> Vec<JsonObject> {
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
            apply_currency_conversion(&mut x, currency_rates);
            if let Db_Name::debit = x.dbName {
                x.amount = -x.amount;
            }
            results.push(x);
        });

    results
}

pub fn saved_this_year(data: &[JsonObject], currency_rates: &HashMap<i32, Decimal>, daily_rate: Decimal, total_pay: Decimal) -> Decimal {
    let total = (sum_of_credits(data, currency_rates, total_pay) - sum_of_debits(data, currency_rates)) * dec!(12);

    let daily_total = daily_rate * dec!(365.25);

    total - daily_total
    //var dailyTotal = dailyRate * (new DateTime(DateTime.Now.Year, 12, 31)).DayOfYear;
}

pub fn net_saved_avg(data: &[JsonObject], currency_rates: &HashMap<i32, Decimal>, daily_rate: Decimal, total_pay: Decimal) -> Decimal {
    //TODO: not 31

    let total = sum_of_credits(data, currency_rates, total_pay) - sum_of_debits(data, currency_rates);

    let daily_total = daily_rate * dec!(31);

    total - daily_total
}

pub fn sum_of_card_held(data: &[JsonObject], currency_rates: &HashMap<i32, Decimal>) -> Decimal {
    let mut amount = dec!(0);
    for mut bank_obj in data.iter().cloned() {
        apply_currency_conversion(&mut bank_obj, currency_rates);
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

/// Totals of the two debt lists, in GBP: (owed to me, I owe).
/// "Debt Owed To Me" (/api/debt) is stored as debt/credit and
/// "Debt I Owe" (/api/debtto) as debt/debit.
pub fn debt_totals(data: &[JsonObject], currency_rates: &HashMap<i32, Decimal>) -> (Decimal, Decimal) {
    let mut owed_to_me = dec!(0);
    let mut i_owe = dec!(0);
    for mut obj in data.iter().cloned() {
        if !matches!(obj.category, Category::debt) {
            continue;
        }
        apply_currency_conversion(&mut obj, currency_rates);
        match obj.dbName {
            Db_Name::credit => owed_to_me += obj.amount,
            Db_Name::debit => i_owe += obj.amount,
        }
    }
    (owed_to_me, i_owe)
}

pub fn sum_of_credits(data: &[JsonObject], currency_rates: &HashMap<i32, Decimal>, total_pay: Decimal) -> Decimal {
    let mut amount = dec!(0);
    for mut bank_obj in data.iter().cloned() {
        apply_currency_conversion(&mut bank_obj, currency_rates);
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

pub fn sum_of_debits(data: &[JsonObject], currency_rates: &HashMap<i32, Decimal>) -> Decimal {
    let mut amount = dec!(0);
    for mut bank_obj in data.iter().cloned() {
        apply_currency_conversion(&mut bank_obj, currency_rates);
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

pub fn calculate<Z: TimeZone>(
    data: &Vec<JsonObject>,
    currency_rates: &HashMap<i32, Decimal>,
    now: &DateTime<Z>,
    daily_rate: Decimal,
    payday: u32,
    weekday_saving: Decimal,
    calc_to_eom: bool,
    calc_following_month: bool,
) -> Decimal {
    let mut amount = dec!(0.0);

    let payday = if payday <= get_days_from_month(now.year(), now.month()) {
        payday
    } else {
        get_days_from_month(now.year(), now.month())
    };

    //TODO: split this off into can be used for calculation?
    let mut next_payday = midnight(&now.timezone(), now.year(), now.month(), payday);

    let after_payday = now.day() >= payday;

    if after_payday {
        next_payday = next_payday + Months::new(1);
    }

    let end_of_month_for_payday = midnight(
        &now.timezone(),
        next_payday.year(),
        next_payday.month(),
        if calc_to_eom {
            get_days_from_month(next_payday.year(), next_payday.month())
        } else {
            payday
        },
    );

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

    for bank_obj in data.iter().cloned() {
        let mut bank_obj = bank_obj;
        apply_currency_conversion(&mut bank_obj, currency_rates);
        match bank_obj {
            JsonObject {
                dbName: Db_Name::credit,
                ..
            } => {
                if can_be_used_in_calculation(&bank_obj, now, &next_payday, after_payday) {
                    amount += bank_obj.amount;
                }
            }
            JsonObject {
                dbName: Db_Name::debit,
                ..
            } => {
                if can_be_used_in_calculation(&bank_obj, now, &next_payday, after_payday) {
                    amount -= bank_obj.amount;
                }
            }
            _ => (),
        }
    }

    if calc_following_month {
        let days_in_payday_month = get_days_from_month(next_payday.year(), next_payday.month());
        for bank_obj in data.iter().cloned() {
            let mut bank_obj = bank_obj;
            if bank_obj.day.is_none() {
                continue;
            }
            match bank_obj.category {
                Category::cardbalance | Category::creditcard => continue,
                _ => (),
            }
            apply_currency_conversion(&mut bank_obj, currency_rates);
            let item_day = bank_obj.day.unwrap() as u32;
            if item_day > next_payday.day() && item_day <= days_in_payday_month {
                match bank_obj.dbName {
                    Db_Name::credit => amount += bank_obj.amount,
                    Db_Name::debit => amount -= bank_obj.amount,
                    _ => (),
                }
            }
        }
    }
    amount
}

pub fn can_be_used_in_calculation<Z: TimeZone>(
    record: &JsonObject,
    now: &DateTime<Z>,
    next_payday: &DateTime<Z>,
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

    let date_obj: DateTime<Z> = if get_days_from_month(now.year(), now.month())
        >= (record.day.unwrap() as u32)
    //TODO: panics if -1?
    //TODO: days in year month
    {
        midnight(&now.timezone(), now.year(), now.month(), record.day.unwrap() as u32) //TODO: panics if -1?
    } else {
        midnight(&now.timezone(), now.year(), now.month() + 1, 1)
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

pub fn remaining_week<Z: TimeZone>(
    data: &Vec<JsonObject>,
    currency_rates: &HashMap<i32, Decimal>,
    now: &DateTime<Z>,
    daily_rate: Decimal,
    payday: u32,
    weekday_saving: Decimal,
    calc_to_eom: bool,
    calc_following_month: bool,
) -> Decimal {
    // This calculates the total to sunday, so adding the daily rate back in and the weekend savings
    let mut amount = calculate(data, currency_rates, now, daily_rate, payday, weekday_saving, calc_to_eom, calc_following_month);

    let weekday = weekday(now);

    if weekday < dec!(5) {
        let weekday_amount_to_debit = weekday_saving * (weekday);
        amount += weekday_amount_to_debit;
    }

    amount += daily_rate * (dec!(7) - weekday);

    amount
}

pub fn end_of_week<Z: TimeZone>(
    data: &Vec<JsonObject>,
    currency_rates: &HashMap<i32, Decimal>,
    now: &DateTime<Z>,
    daily_rate: Decimal,
    payday: u32,
    weekday_saving: Decimal,
    calc_to_eom: bool,
    calc_following_month: bool,
) -> Decimal {
    //If friday was today, this is what the total would be
    //Add back in the weekday savings and the fridays daily rate
    let mut amount = calculate(data, currency_rates, now, daily_rate, payday, weekday_saving, calc_to_eom, calc_following_month);

    let weekday = weekday(now);

    if weekday < dec!(5) {
        let weekday_amount_to_debit = weekday_saving * dec!(4);
        amount += weekday_amount_to_debit + daily_rate;
    } else if weekday > dec!(5) {
        return dec!(0); //TODO: maybe a different way
    }
    amount
}

fn weekday<Z: TimeZone>(time: &DateTime<Z>) -> Decimal {
    //TODO: convert
    match Decimal::from_u32(time.weekday() as u32) {
        Some(result) => result + dec!(1),
        None => panic!("Calc error"),
    }
}

pub fn full_weekend<Z: TimeZone>(
    data: &Vec<JsonObject>,
    currency_rates: &HashMap<i32, Decimal>,
    now: &DateTime<Z>,
    daily_rate: Decimal,
    payday: u32,
    weekday_saving: Decimal,
    calc_to_eom: bool,
    calc_following_month: bool,
) -> Decimal {
    // I have no idea what this does
    // TODO: investigate
    // Takes off the difference between the daily rate and the weekday savings between now and the weekend?
    let mut amount = remaining_week(data, currency_rates, now, daily_rate, payday, weekday_saving, calc_to_eom, calc_following_month);

    let weekday = weekday(now);

    if weekday < dec!(5) {
        amount -= (dec!(5) - weekday) * (daily_rate - weekday_saving);
    }

    amount
}

pub fn get_items_between<Z: TimeZone>(
    data: &[JsonObject],
    currency_rates: &HashMap<i32, Decimal>,
    from: &DateTime<Z>,
    to: &DateTime<Z>,
) -> Vec<JsonObject> {
    let mut results: Vec<JsonObject> = Vec::new();

    let mut current = from.date_naive();
    let to_date = to.date_naive();

    while current <= to_date {
        let day = current.day() as i32;
        let mut dates: Vec<i32> = vec![day];

        if current.day() == 1 {
            let month = current.month();
            if month == 10 || month == 5 || month == 7 || month == 12 {
                dates.push(31);
            } else if month == 3 {
                dates.push(31);
                dates.push(30);
                if get_days_from_month(current.year(), 2) < 29 {
                    dates.push(29);
                }
            }
        }

        data.iter()
            .filter(|x| x.day.is_some())
            .filter(|x| dates.contains(&x.day.unwrap()))
            .cloned()
            .for_each(|mut x| {
                apply_currency_conversion(&mut x, currency_rates);
                if let Db_Name::debit = x.dbName {
                    x.amount = -x.amount;
                }
                results.push(x);
            });

        match current.checked_add_days(Days::new(1)) {
            Some(next) => current = next,
            None => break,
        }
    }

    results
}

/// Start of the given day in `tz`. Picks the earlier instant if midnight is
/// ambiguous, and 01:00 if midnight is skipped by a DST change.
fn midnight<Z: TimeZone>(tz: &Z, year: i32, month: u32, day: u32) -> DateTime<Z> {
    tz.with_ymd_and_hms(year, month, day, 0, 0, 0)
        .earliest()
        .or_else(|| tz.with_ymd_and_hms(year, month, day, 1, 0, 0).earliest())
        .expect("Time zone error") //todo: 500
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

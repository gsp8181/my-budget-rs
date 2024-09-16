use rocket::fairing::AdHoc;

pub mod bank;
pub mod cardbalance;
pub mod cardheld;
pub mod cash;
pub mod debt;
pub mod debtto;
pub mod misccredit;
pub mod miscdebit;
pub mod regularcredit;
pub mod regularpayment;
pub mod settings;
pub mod uncleared;

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Controllers Stage", |rocket| async {
        rocket
            .attach(bank::stage())
            .attach(regularcredit::stage())
            .attach(cardbalance::stage())
            .attach(uncleared::stage())
            .attach(regularpayment::stage())
            .attach(miscdebit::stage())
            .attach(misccredit::stage())
            .attach(debtto::stage())
            .attach(debt::stage())
            .attach(cash::stage())
            .attach(cardheld::stage())
            .attach(settings::stage())
    })
}

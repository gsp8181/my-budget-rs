mod structs;

extern crate cursive_table_view;

use std::{cmp::Ordering, fs::File, io::Read};

use cursive::{
    align::HAlign,
    event::Key,
    menu,
    view::{Nameable, Resizable, Scrollable},
    views::{LinearLayout, Panel, TextView},
    Cursive,
};
use cursive_table_view::TableView;
use structs::{BasicColumn, JsonObject, PublicItem};

fn tview(siv: &mut Cursive) {
    siv.pop_layer();

    let req = build_req("");
    let resp = req
        .call()
        .expect("Failed to read JSON for base API")
        .into_json::<PublicItem>()
        .unwrap();

    let mut table = TableView::<JsonObject, BasicColumn>::new()
        .column(BasicColumn::Name, "Name", |c| c)
        .column(BasicColumn::Amount, "Amount", |c| c.width_percent(20));

    table.set_items(resp.today.clone());

    siv.add_layer(
        LinearLayout::vertical()
            .child(LinearLayout::vertical().child(
                Panel::new(TextView::new(format!("£{}", resp.amount))).title("Balance Held"), //.fixed_width(20),
            ))
            .child(
                LinearLayout::horizontal()
                    .child(
                        Panel::new(TextView::new(format!("£{}", resp.end_of_week)))
                            .title("End of Week Total"),
                    )
                    .child(
                        Panel::new(TextView::new(format!("£{}", resp.full_weekend)))
                            .title("Full Weekend Total"),
                    )
                    .child(
                        Panel::new(TextView::new(format!("£{}", resp.remaining_week)))
                            .title("Remaining Week Total"),
                    ),
            )
            .child(
                LinearLayout::horizontal()
                    .child(
                        Panel::new(TextView::new(format!("£{}", resp.monthly_credits)))
                            .title("Total Monthly Credits"),
                    )
                    .child(
                        Panel::new(TextView::new(format!("£{}", resp.monthly_debits)))
                            .title("Total Monthly Debits"),
                    )
                    .child(
                        Panel::new(TextView::new(format!("£{}", resp.card_held_total)))
                            .title("Card Balance Held"),
                    ),
            )
            .child(
                LinearLayout::horizontal()
                    .child(
                        Panel::new(TextView::new(format!("£{}", resp.net_saved_avg)))
                            .title("Monthly Saved"),
                    )
                    .child(
                        Panel::new(TextView::new(format!("£{}", resp.saved_this_year)))
                            .title("Yearly Saved"),
                    ),
            )
            .child(
                Panel::new(
                    table
                        .with_name("table")
                        .min_size((0, 2 + &(resp.today).len())),
                )
                .title("Payments Today"),
            )
            .scrollable()
            .scroll_x(true)
            .with_name("budget"),
    );
}

fn iview(siv: &mut Cursive, api_name: &str, title: &str) {
    siv.pop_layer();

    let req = build_req(api_name);

    let resp = req
        .call()
        .unwrap()
        .into_json::<Vec<JsonObject>>()
        .expect(&format!("Failed to read JSON for {}", api_name));

    let mut table = TableView::<JsonObject, BasicColumn>::new()
        .column(BasicColumn::Name, "Name", |c| c.width_percent(80))
        .column(BasicColumn::Amount, "Amount", |c| {
            c.ordering(Ordering::Greater)
                .align(HAlign::Right)
                .width_percent(20)
        });

    table.set_items(resp.clone());

    siv.add_fullscreen_layer(
        Panel::new(
            table
                .with_name("table")
                .min_size((60, 2 + &(resp).len()))
                .scrollable()
                .scroll_x(true)
                .full_screen(),
        )
        .title(title)
        .full_width()
        .full_screen()
        .with_name("budget"),
    );
}

fn build_req(api_name: &str) -> ureq::Request {
    let mut file = File::open("config.toml").expect("Failed to open config.toml");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read config.toml");

    let tomltwable = contents.parse::<toml::Table>().unwrap();

    let mut req = ureq::get(&format!(
        "{}/api/{}",
        tomltwable["connection"]["url"]
            .as_str()
            .expect("Failed to read url property in connection section of config.toml"),
        api_name
    ));

    if let Some(table) = tomltwable["headers"].as_table() {
        for header in table.iter() {
            req = req.set(
                header.0,
                header
                    .1
                    .as_str()
                    .expect(&format!("failed to read header {}", header.0)),
            );
        }
    }
    req
}

fn main() {
    // Creates the cursive root - required for every application.
    let mut siv = cursive::default();

    tview(&mut siv);

    siv.add_global_callback('q', |s| s.quit());

    siv.menubar()
        .add_leaf("Home", tview)
        .add_subtree(
            "Credit",
            menu::Tree::new()
                .leaf("Bank", |cb| iview(cb, "bank", "Bank Accounts"))
                .leaf("Cash", |cb| iview(cb, "cash", "Cash"))
                .leaf("Regular Credit", |cb| {
                    iview(cb, "regularcredit", "Regular Credit")
                })
                .leaf("Card Items Held Off Balance", |cb| {
                    iview(cb, "cardheld", "Card Items Held Off Balance")
                })
                .leaf("Uncleared Item", |cb| {
                    iview(cb, "uncleared", "Uncleared Payment On Card")
                })
                .leaf("Debt Owed to Me", |cb| iview(cb, "debt", "Debt"))
                .leaf("Misc Credit", |cb: &mut Cursive| {
                    iview(cb, "misccredit", "Miscellaneous Credit")
                }),
        )
        .add_subtree(
            "Debit",
            menu::Tree::new()
                .leaf("Card Balance", |cb| {
                    iview(cb, "cardbalance", "Card Balance")
                })
                .leaf("Regular Payment", |cb| {
                    iview(cb, "regularpayment", "Regular Payment")
                })
                .leaf("Debt I Owe", |cb| iview(cb, "debtto", "Debt Owed"))
                .leaf("Misc Debit", |cb| {
                    iview(cb, "miscdebit", "Miscellaneous Debit")
                }),
        )
        .add_delimiter()
        .add_leaf("Quit", |s| s.quit());

    siv.set_autohide_menu(false);

    siv.add_global_callback(Key::Esc, |s| s.select_menubar());

    // Starts the event loop.
    siv.run();
}

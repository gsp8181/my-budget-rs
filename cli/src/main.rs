mod structs;

extern crate cursive_table_view;

use std::{cmp::Ordering, fs::File, io::Read};

use cursive::{
    event::Key,
    view::{Nameable, Resizable, Scrollable},
    views::{Dialog, LinearLayout, Panel, TextView},
    Cursive,
};
use cursive_table_view::TableView;
use structs::{BasicColumn, JsonObject, PublicItem};

fn tview(siv: &mut Cursive) {
    siv.pop_layer();

    let mut file = File::open("config.toml").expect("Failed to open config.toml");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read config.toml");

    let tomltwable = contents.parse::<toml::Table>().unwrap();

    // todo, parse headers

    let mut req = ureq::get(&format!(
        "{}/api/",
        tomltwable["connection"]["url"]
            .as_str()
            .expect("Failed to read url property in connection section of config.toml")
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
    let resp = req.call().unwrap().into_json::<PublicItem>().unwrap();

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

fn main() {
    // Creates the cursive root - required for every application.
    let mut siv = cursive::default();

    tview(&mut siv);

    siv.add_global_callback('q', |s| s.quit());

    siv.menubar()
        // We add a new "File" tree
        .add_delimiter()
        .add_leaf("Quit", |s| s.quit())
        .add_leaf("Refresh", tview);

    siv.set_autohide_menu(false);

    siv.add_global_callback(Key::Esc, |s| s.select_menubar());

    // Starts the event loop.
    siv.run();
}

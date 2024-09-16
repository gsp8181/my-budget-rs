/*
using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using BudgetPanel.Models;
using BudgetPanel.Services;
using Microsoft.AspNetCore.Cors;
using Microsoft.AspNetCore.Mvc;
using Newtonsoft.Json.Linq;

namespace BudgetPanel.Controllers
{
    [Route("api/[controller]")]
    [ApiController]
   // [EnableCors("MyAllowSpecificOrigins")]
    public class SettingsController : ControllerBase
    {
        private readonly BudgetService _context;
        public SettingsController(BudgetService context)
        {
            _context = context;
        }

        // GET api/values
        [HttpGet]
        public ActionResult<IEnumerable<Settings>> Get()
        {
            return Ok(_context.GetAllSettings());
        }

        // GET api/values/5
        [HttpGet("{id}")]
        public ActionResult<string> Get(string setting)
        {
            return "value";
        }

        // POST api/values
        [HttpPost]
        public ActionResult<IEnumerable<Settings>> Post([FromBody] System.Text.Json.JsonElement value)
        {
            foreach (var jsonElement in value.EnumerateObject())
            {
                //TODO: exists?
                if(jsonElement.Value.ValueKind == System.Text.Json.JsonValueKind.Number)
                {
                    _context.set_setting(jsonElement.Name, jsonElement.Value.GetDecimal().ToString());
                }
                else
                {
                    _context.set_setting(jsonElement.Name, jsonElement.Value.GetString());
                }
            }

            /*
    $json_data = Flight::request()->data;

    while($json_data->valid())
    {
        $key = $json_data->key();
        $value = $json_data->current();
        set_setting($key, $value);
        $json_data->next();
    }
             */

            //throw new NotImplementedException();

            return Ok(_context.GetAllSettings());
        }

    }
}
 */

use rocket::{fairing::AdHoc, serde::json::Json};

use crate::{
    models::settings::{SettingDatabaseObject, SettingEntryObject},
    services::settingsstore::{get_collection, get_setting, print_all_values, set_setting},
    Db,
};

#[get("/")]
async fn get(db: Db) -> Json<Vec<SettingDatabaseObject>> {
    let result: Vec<SettingDatabaseObject> = get_collection(&db).await;

    Json(result)
}

#[get("/<id>")]
async fn get_by_id(db: Db, id: String) -> String {
    get_setting(&db, id, String::from("1")).await //TODO: LIST
}

#[post("/", format = "json", data = "<obj>")]
async fn post(db: Db, obj: Json<SettingEntryObject>) {
    //TODO: no other practical way to work with the braindead API in the UI
    set_setting(&db, String::from("dailyRate"), obj.dailyRate.clone()).await;
    set_setting(&db, String::from("pay"), obj.pay.clone()).await;
    set_setting(&db, String::from("payday"), obj.payday.clone()).await;
    set_setting(
        &db,
        String::from("weekdaySaving"),
        obj.weekdaySaving.clone(),
    )
    .await;
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Settings", |rocket| async {
        rocket.mount(format!("/api/settings"), routes![get, get_by_id, post])
    })
}

/*
pub const PAYDAY: u32 = 30;
pub const WEEKDAY_SAVING: Decimal = dec!(25);
pub const DAILY_RATE: Decimal = dec!(40);
pub const TOTAL_PAY: Decimal = dec!(1000.00);
 */

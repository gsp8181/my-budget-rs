use serde_json::{json, Value};

use crate::structs::{Category, DBObj, DBObjIn, Db_Name};
use lazy_static::lazy_static;

use std::fs::File;
use std::io::Read;
use std::sync::Mutex;

lazy_static! {
    static ref my_mutex: Mutex<i32> = Mutex::new(0i32);
}

pub fn get_collection() -> Vec<DBObj> {
    //TODO: load blank if file does not exist
    let mut file = File::open("store.json").unwrap();
    let mut buff = String::new();
    file.read_to_string(&mut buff).unwrap();

    let result: Vec<DBObj> = serde_json::from_str(&buff).unwrap(); //TODO: make result error handling
    result
}

pub fn print_all_values(db_name: Db_Name, category: Category, sort_by_day: bool) -> Vec<DBObj> {
    let coll = get_collection();
    let mut v: Vec<DBObj> = Vec::new();

    for bank_obj in &coll {
        if bank_obj.category.clone() as u32 == category.clone() as u32
            && bank_obj.dbName.clone() as u32 == db_name.clone() as u32
        //TODO: horrifying
        {
            v.push(DBObj::from(bank_obj.clone()))
        }
    }

    if sort_by_day {
        v.sort_by_key(|f| f.day)
    }

    v
}

pub fn get_record_by_id(db_name: Db_Name, category: Category, id: u32) -> Option<DBObj> {
    let table = print_all_values(db_name, category, false);

    for bank_obj in &table {
        if bank_obj.id == id {
            return Some(bank_obj.clone());
        }
    }
    None
}

pub fn insert_record(
    db_name: Db_Name,
    category: Category,
    new_db_obj: DBObjIn,
    attributes: Vec<&str>,
) -> DBObj {
    let mut db = get_collection();
    let max_rec = db.iter().max_by_key(|p| p.id); //TODO: id links like this are bad bc of linked records lol
    let id = match max_rec {
        Some(rec) => rec.id + 1,
        None => 1,
    };

    //TODO: verify attributes

    let new_obj = DBObj {
        dbName: db_name,
        id: id,
        oldId: None,
        category: category,
        name: new_db_obj.name.unwrap(),
        day: new_db_obj.day,
        amount: new_db_obj.amount.unwrap(),
        cardid: new_db_obj.cardid,
    };

    let rtn_obj = new_obj.clone();

    db.push(new_obj);

    let result = write(&db); //todo: parse error

    rtn_obj
}

pub fn modify_record_by_id(
    db_name: Db_Name,
    category: Category,
    attributes: Vec<&str>,
    id: u32,
    new_db_obj: DBObjIn,
) -> std::io::Result<DBObj> {
    let mut db = get_collection();
    let index = db.iter().position(|p| p.id == id); //TODO: verify same db
    let id = match index {
        Some(rec) => rec,
        None => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "could not delete",
            ))
        } //TODO: needs work,
    };

    let mut item = db[id].clone();
    //TODO: verify attributes

    if let Some(d) = new_db_obj.name {
        item.name = d
    }

    if let Some(d) = new_db_obj.amount {
        item.amount = d
    }

    item.day = new_db_obj.day;
    item.cardid = new_db_obj.cardid;

    let result_obj = item.clone();

    let _ = std::mem::replace(&mut db[id], item);

    let result = write(&db); //todo: parse error

    Ok(result_obj)
}

pub fn delete_record_by_id(
    db_name: Db_Name,
    category: Category,
    id: u32,
) -> std::io::Result<Value> //TODO: make all result
{
    let mut db = get_collection();
    let index = db.iter().position(|p| p.id == id); //TODO: verify same db
    let id = match index {
        Some(rec) => rec,
        None => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "could not delete",
            ))
        } //TODO: needs work,
    };

    db.remove(id);

    let result = write(&db); //todo: parse error

    Ok(json!({ "status": "deleted" }))
}

fn write(vec: &Vec<DBObj>) -> std::io::Result<()> {
    let mut _mutex_changer = my_mutex.lock().unwrap();

    let file = std::fs::File::create("store.json")?;
    let mut writer = std::io::BufWriter::new(file);
    serde_json::to_writer(&mut writer, &vec)?;
    std::io::Write::flush(&mut writer)?;
    Ok(())
}

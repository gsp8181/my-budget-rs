use crate::generate_controller;

generate_controller!(CATEGORY, DB_NAME, ATTRIBUTES, CONTROLLERNAME);

const CATEGORY: Category = Category::bank;
const DB_NAME: Db_Name = Db_Name::credit;
const ATTRIBUTES: &str = "name,amount"; //TODO:enum?
const CONTROLLERNAME: &str = "bank";

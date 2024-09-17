use crate::generate_controller;

generate_controller!(CATEGORY, DB_NAME, ATTRIBUTES, CONTROLLERNAME);

const CATEGORY: Category = Category::recurring;
const DB_NAME: Db_Name = Db_Name::credit;
const ATTRIBUTES: &str = "name,amount,day"; //TODO:enum?
const CONTROLLERNAME: &str = "regularcredit";

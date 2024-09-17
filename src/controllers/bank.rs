use crate::generate_controller;

generate_controller!(Category::bank, Db_Name::credit, "name,amount", "bank");

const CATEGORY: Category = Category::bank;
const DB_NAME: Db_Name = Db_Name::credit;
const ATTRIBUTES: &str = "name,amount"; //TODO:enum?
const CONTROLLERNAME: &str = "bank";

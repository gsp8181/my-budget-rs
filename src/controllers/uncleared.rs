use crate::generate_controller;

generate_controller!(CATEGORY, DB_NAME, ATTRIBUTES, CONTROLLERNAME);

const CATEGORY: Category = Category::creditcard;
const DB_NAME: Db_Name = Db_Name::debit;
const ATTRIBUTES: &str = "name,amount,cardid"; //TODO:enum?
const CONTROLLERNAME: &str = "uncleared";

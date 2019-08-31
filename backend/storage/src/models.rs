use crate::schema::ingredients;

#[derive(Queryable)]
pub struct Ingredient {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[table_name = "ingredients"]
pub struct NewIngredient {
    pub name: String,
}

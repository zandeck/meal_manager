use crate::schema::ingredients;
use juniper::FieldResult;
use juniper::RootNode;

#[derive(Queryable, GraphQLObject)]
#[graphql(description = "This is an ingredient that can be input in a recipe")]
pub struct Ingredient {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable, GraphQLInputObject)]
#[graphql(description = "New Ingredient to be inserted")]
#[table_name = "ingredients"]
pub struct NewIngredient {
    pub name: String,
}

#[derive(Queryable)]
pub struct Recipe {
    pub name: String,
    pub create: String,
}

pub struct QueryRoot;

graphql_object!( QueryRoot: () |&self| {
    field apiVersion() -> &str {
        "1.0"
    }

    field ingredient(&executor, id: i32) -> FieldResult<Ingredient> {
        Ok(
            Ingredient {
                id: 42,
                name: "Dummy Ingredient".to_string(),
            })
    }
});

pub struct MutationRoot;
pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

graphql_object!(MutationRoot: () |&self| {
    field createHuman(&executor, new_ingredient: NewIngredient) -> FieldResult<Ingredient> {
         Ok(
            Ingredient {
                id: 42,
                name: "Lol ingredient".to_string(),
            })
    }
});

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}

use diesel::{Queryable, Selectable};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::category_lookup)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CategoryLookup {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}

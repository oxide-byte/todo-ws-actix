use rbatis::{crud, html_sql, htmlsql_select_page, impl_select};
use rbatis::executor::Executor;
use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Status {
    OPEN,
    PROGRESS,
    CLOSE,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Todo {
    pub id: Option<u64>,
    pub owner: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
    pub created: Option<DateTime>,
    pub modified: Option<DateTime>,
}

crud!(Todo{});

impl_select!(Todo{select_by_id(id:u64) -> Option => "`where id = #{id} limit 1`"});

htmlsql_select_page!(select_page_data() -> Todo => "mybatis_sql/todo.html");

#[html_sql(
r#"
    <select id="get_next_id">
        `SELECT nextval('todo_seq')`
    </select>"#
)]
pub async fn get_next_id(rb: &mut dyn Executor) -> rbatis::Result<u64> { impled!() }
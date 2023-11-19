use once_cell::sync::Lazy;
use rbatis::RBatis;
use rbdc_pg::driver::PgDriver;

use crate::todo::todo_service::TodoService;

pub static CONTEXT: Lazy<ServiceContext> = Lazy::new(|| ServiceContext::default());

pub struct ServiceContext {
    pub rb: RBatis,
    pub todo_service: TodoService,
}

#[macro_export]
macro_rules! pool {
    () => {
        &mut $crate::context::CONTEXT.rb.clone()
    };
}

#[macro_export]
macro_rules! todo_service {
    () => {
        &mut $crate::context::CONTEXT.todo_service.clone()
    };
}

impl Default for ServiceContext {
    fn default() -> Self {
        let rb = RBatis::new();
        rb.init(PgDriver {}, "postgres://todo:password@localhost:5432/todo").unwrap();

        ServiceContext {
            rb: rb,
            todo_service: TodoService {},
        }
    }
}
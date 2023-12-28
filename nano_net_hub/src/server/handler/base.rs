use axum::Router;

pub trait Controller{
    /// 注册路由
    fn router(&self) ->Router;
}
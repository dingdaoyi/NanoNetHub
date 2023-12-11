use std::fmt::{Display, Formatter};
use crate::config::database::get_conn;
use crate::models::common::page::PaginationRequest;
use crate::models::{PaginationResponse, ServerError};
use crate::SqlRow;

#[derive(Debug)]
pub struct PageSql {
    query: String,
    count_query: String,
    query_params: Vec<Value>,
    limit: u32,
    offset: u32,

}


impl PageSql {
    // 执行
    pub async fn execute<T>(&mut self) -> Result<PaginationResponse<T>, ServerError>
        where T: Send + Unpin + for<'r> sqlx::FromRow<'r, SqlRow> {
        let mut count_query =
            sqlx::query_scalar::<_, u32>(&self.count_query);
        let mut query = sqlx::query_as::<_, T>(&self.query);
        for value in &self.query_params {
            match value {
                Value::INT(value) => {
                    count_query = count_query.bind(value);
                    query = query.bind(value);
                }
                Value::BOOL(value) => {
                    count_query = count_query.bind(value);
                    query = query.bind(value);
                }
                Value::STRING(value) => {
                    count_query = count_query.bind(value);
                    query = query.bind(value);
                }
            }
        }
        let pool = get_conn();
        let result = count_query.fetch_one(&pool)
            .await?;
        let results = query
            .bind(self.limit)
            .bind(self.offset)
            .fetch_all(&pool).await?;
        Ok(PaginationResponse::new(results, result))
    }
}

pub struct PageSqlBuilder<'a> {
    table_name: String,
    page_request: PaginationRequest,
    conditions: Vec<Condition<'a>>,
    where_query: Option<String>,
}

impl<'a, > PageSqlBuilder<'a> {
    pub fn builder(table_name: impl Into<String>, page_request: &PaginationRequest) -> Self {
        Self {
            table_name: table_name.into(),
            page_request: PaginationRequest {
                page: page_request.page,
                size: page_request.size,
                sort_fields: page_request.sort_fields.clone(),
                direction: page_request.direction,
            },
            conditions: vec![],
            where_query: None,
        }
    }
    pub fn conditions(mut self, conditions: Vec<Condition<'a>>) -> Self {
        self.conditions = conditions;
        self
    }
    pub fn where_query(mut self, where_query: impl Into<String>) -> Self {
        self.where_query = Some(where_query.into());
        self
    }

    pub fn build(self) -> PageSql {
        let mut sql = format!("SELECT * FROM {} ", &self.table_name);
        let mut count_sql = format!("SELECT count(*) FROM  {} ", &self.table_name);
        let mut query_params = vec![];
        let mut param_index = 1;
        match self.where_query {
            None => {
                let conditions = self.conditions;

                if !conditions.is_empty() {
                    let mut values = vec![];
                    let mut sql_strings = vec![];
                    for condition in conditions {
                        let (sql, value) = condition.to_sql(param_index);
                        param_index = param_index + 1;
                        sql_strings.push(sql);
                        values.push(value);
                    }
                    query_params.extend(values);
                    let condition_query = sql_strings.join(" and ");
                    sql = format!("{} where {}", sql, condition_query);
                    count_sql = format!("{} where {}", count_sql, condition_query);
                }
            }
            Some(where_query) => {
                sql = format!("{} where {}", sql, where_query);
                count_sql = format!("{} where {}", count_sql, where_query);
                let conditions = self.conditions;
                if !conditions.is_empty() {
                    let mut values = vec![];
                    let mut sql_strings = vec![];
                    for condition in conditions {
                        let (sql, value) = condition.to_sql(param_index);
                        param_index = param_index + 1;
                        sql_strings.push(sql);
                        values.push(value);
                    }
                    query_params.extend(values);
                    let condition_query = sql_strings.join(" and ");
                    sql = format!("{} and  {}", sql, condition_query);
                    count_sql = format!("{} and {}", count_sql, condition_query);
                }
            }
        }
        let request = self.page_request;
        if !request.sort_fields.is_empty() {
            let sort_field = request.sort_fields.join(",");
            sql = format!("{} order by {} {}", sql, sort_field, request.direction);
        }
        PageSql {
            count_query: count_sql,
            query: format!("{} LIMIT ${} OFFSET ${}", sql, param_index, param_index + 1),
            query_params,
            limit: request.limit(),
            offset: request.offset(),
        }
    }
}

#[derive(Clone, Debug, )]
pub enum Value {
    INT(i32),
    BOOL(bool),
    STRING(String),
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Value::INT(value)
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::BOOL(value)
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::STRING(value)
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Value::STRING(value.to_string())
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::INT(value) => {
                write!(f, "{}", value)
            }
            Value::BOOL(value) => {
                write!(f, "{}", value)
            }
            Value::STRING(value) => {
                write!(f, "{}", value)
            }
        }
    }
}

pub enum Condition<'a> {
    Equal(&'a str, Value),
    Like(&'a str, Value),
}

impl<'a> Condition<'a> {
    fn to_sql(&self, index: i32) -> (String, Value) {
        match self {
            Condition::Equal(field, value) => (format!("{} = ${} ", field, index), value.clone()),
            Condition::Like(field, value) => (format!("{} like ${} ", field, index), Value::STRING(format!("{}", value))),
        }
    }
}


#[cfg(test)]
mod testing {
    use crate::models::common::page::Direction;
    use super::*;

    #[test]
    fn test_query() {
        let res = PageSqlBuilder::builder("tb_product", &PaginationRequest {
            page: 0,
            size: 20,
            sort_fields: vec!["age".into(), "age2".into()],
            direction: Direction::ASC
            ,
        }).conditions(vec![
            Condition::Like("name", "value".into()),
            Condition::Equal("sex", true.into()),
        ])
            .where_query("age=3")
            .build();
        println!("{:?}", res)
    }
}

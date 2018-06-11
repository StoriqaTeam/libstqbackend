use std;
use std::collections::BTreeMap;
use tokio_postgres::types::ToSql;

pub trait Filter {
    fn into_filtered_operation_builder(self, op: FilteredOperation, table: &'static str) -> FilteredOperationBuilder;
}

pub trait Inserter {
    fn into_insert_builder(self, table: &'static str) -> InsertBuilder;
}

pub trait Updater {
    fn into_update_builder(self, table: &'static str) -> UpdateBuilder;
}

/// Filtering operation
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FilteredOperation {
    Select,
    Delete,
}

/// Construct a simple select or delete query.
pub struct FilteredOperationBuilder {
    op: FilteredOperation,
    table: &'static str,
    extra: &'static str,
    filters: BTreeMap<&'static str, Box<ToSql + Send + 'static>>,
}

impl FilteredOperationBuilder {
    /// Create a new builder
    pub fn new(op: FilteredOperation, table: &'static str) -> Self {
        Self {
            op,
            table,
            extra: Default::default(),
            filters: Default::default(),
        }
    }

    /// Add filtering arguments
    pub fn with_arg<V: ToSql + Send + 'static>(mut self, column: &'static str, value: V) -> Self {
        self.filters.insert(column, Box::new(value));
        self
    }

    /// Add additional statements before the semicolon
    pub fn with_extra(mut self, extra: &'static str) -> Self {
        self.extra = extra;
        self
    }

    /// Build a query
    pub fn build(self) -> (String, Vec<Box<ToSql + Send + 'static>>) {
        let mut args = vec![];
        let mut query = format!(
            "{} {}",
            match self.op {
                FilteredOperation::Select => "SELECT * FROM",
                FilteredOperation::Delete => "DELETE FROM",
            },
            self.table
        );

        for (i, (col, arg)) in self.filters.into_iter().enumerate() {
            if i == 0 {
                query.push_str(" WHERE ");
            } else {
                query.push_str(" AND ");
            }
            query.push_str(&format!("{} = ${}", col, i + 1));
            args.push(arg);
        }
        let out = format!(
            "{} {}{};",
            &query,
            self.extra,
            if self.op == FilteredOperation::Delete { " RETURNING *" } else { "" }
        );

        (out, args)
    }
}

/// Construct a simple insert query.
pub struct InsertBuilder {
    table: &'static str,
    extra: &'static str,
    values: BTreeMap<&'static str, Box<ToSql + Send + 'static>>,
}

impl InsertBuilder {
    pub fn new(table: &'static str) -> Self {
        Self {
            table,
            extra: Default::default(),
            values: Default::default(),
        }
    }

    pub fn with_arg<V: ToSql + Send + 'static>(mut self, k: &'static str, v: V) -> Self {
        self.values.insert(k, Box::new(v));
        self
    }

    /// Add additional statements before the semicolon
    pub fn with_extra(mut self, extra: &'static str) -> Self {
        self.extra = extra;
        self
    }

    /// Builds a query
    pub fn build(self) -> (String, Vec<Box<ToSql + Send + 'static>>) {
        let mut args = vec![];
        let mut query = format!("INSERT INTO {}", self.table);

        let mut col_string = String::new();
        let mut arg_string = String::new();
        for (i, (col, arg)) in self.values.into_iter().enumerate() {
            if i > 0 {
                col_string.push_str(", ");
                arg_string.push_str(", ");
            }

            col_string.push_str(&col);
            arg_string.push_str(&format!("${}", i + 1));
            args.push(arg);
        }
        query = format!("{} ({}) VALUES ({})", &query, &col_string, &arg_string);

        if !self.extra.is_empty() {
            query.push_str(&format!(" {}", &self.extra));
        }

        query.push_str(" RETURNING *;");

        (query, args)
    }
}

pub struct UpdateBuilder {
    table: &'static str,
    extra: &'static str,
    values: BTreeMap<&'static str, Box<ToSql + Send + 'static>>,
    filters: BTreeMap<&'static str, Box<ToSql + Send + 'static>>,
}

impl UpdateBuilder {
    pub fn new(table: &'static str) -> Self {
        Self {
            table,
            extra: Default::default(),
            values: Default::default(),
            filters: Default::default(),
        }
    }

    /// Add filtering arguments
    pub fn with_filter<V: ToSql + Send + 'static>(mut self, column: &'static str, value: V) -> Self {
        self.filters.insert(column, Box::new(value));
        self
    }

    /// Add values to set
    pub fn with_value<V: ToSql + Send + 'static>(mut self, column: &'static str, value: V) -> Self {
        self.values.insert(column, Box::new(value));
        self
    }

    /// Add additional statements before the semicolon
    pub fn with_extra(mut self, extra: &'static str) -> Self {
        self.extra = extra;
        self
    }

    /// Builds a query
    pub fn build(self) -> (String, Vec<Box<ToSql + Send + 'static>>) {
        let mut values = vec![];
        let mut filters = vec![];

        let mut arg_index = 1;

        let mut value_string = String::new();
        for (col, arg) in self.values {
            if value_string.is_empty() {
                value_string.push_str("SET ");
            } else {
                value_string.push_str(", ");
            }

            value_string.push_str(&format!("{} = ${}", col, arg_index));
            arg_index += 1;
            values.push(arg);
        }

        let mut filter_string = String::new();
        for (col, arg) in self.filters {
            if filter_string.is_empty() {
                filter_string.push_str("WHERE ");
            } else {
                filter_string.push_str(" AND ");
            }

            filter_string.push_str(&format!("{} = ${}", col, arg_index));
            arg_index += 1;
            filters.push(arg);
        }

        let mut query = format!("UPDATE {} {} {}", self.table, &value_string, &filter_string);

        if !self.extra.is_empty() {
            query.push_str(&format!(" {}", self.extra));
        }

        query.push_str(" RETURNING *;");

        let args = std::iter::Iterator::chain(values.into_iter(), filters.into_iter()).collect::<Vec<Box<ToSql + Send + 'static>>>();

        (query, args)
    }
}

impl From<FilteredOperationBuilder> for UpdateBuilder {
    fn from(v: FilteredOperationBuilder) -> Self {
        Self {
            table: v.table,
            extra: v.extra,
            filters: v.filters,
            values: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_builder() {
        let res = UpdateBuilder::new("my_table")
            .with_filter("filter_column1", "c")
            .with_filter("filter_column2", "d")
            .with_value("value_column1", "a")
            .with_value("value_column2", "b")
            .build();

        let expectation = (
            "UPDATE my_table SET value_column1 = $1, value_column2 = $2 WHERE filter_column1 = $3 AND filter_column2 = $4 RETURNING *;",
            vec!["a", "b", "c", "d"]
                .into_iter()
                .map(|v| Box::new(v) as Box<ToSql + Send + 'static>)
                .collect::<Vec<Box<ToSql + Send + 'static>>>(),
        );

        assert_eq!(res.0, expectation.0);
        assert_eq!(format!("{:?}", res.1), format!("{:?}", expectation.1));
    }
}

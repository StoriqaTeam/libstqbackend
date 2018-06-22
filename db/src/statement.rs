use std;
use std::collections::BTreeMap;
use std::fmt;
use tokio_postgres::types::ToSql;

pub trait Filter {
    fn into_filtered_operation_builder(self, table: &'static str) -> FilteredOperationBuilder;
}

pub trait Inserter {
    fn into_insert_builder(self, table: &'static str) -> InsertBuilder;
}

pub trait Updater {
    fn into_update_builder(self, table: &'static str) -> UpdateBuilder;
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SelectOperation {
    Count,
}

impl SelectOperation {
    fn to_sql(&self) -> &'static str {
        use self::SelectOperation::*;

        match self {
            Count => "count",
        }
    }
}

/// Filtering operation
#[derive(Clone, Debug, PartialEq)]
pub enum FilteredOperation {
    Select { op: Option<SelectOperation>, limit: Option<i32> },
    Delete,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ComparisonMode {
    LT,
    LTE,
    EQ,
    GTE,
    GT,
}

type ColumnFilters = Vec<(ComparisonMode, Box<ToSql + 'static>)>;
type Filters = BTreeMap<&'static str, ColumnFilters>;

fn build_where_from_filters(filters: Filters, mut i: usize) -> (String, Vec<Box<ToSql + 'static>>) {
    let mut query = String::new();
    let mut args = vec![];

    let mut started = false;

    for (col, filter) in filters.into_iter() {
        for (mode, value) in filter.into_iter() {
            if started {
                query.push_str(" AND ");
            }
            query.push_str(&format!("{} {} ${}", col, &mode.to_string(), i));
            args.push(value);

            started = true;
            i += 1;
        }
    }

    (query, args)
}

impl fmt::Display for ComparisonMode {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use self::ComparisonMode::*;

        write!(
            fmt,
            "{}",
            match self {
                LT => "<",
                LTE => "<=",
                EQ => "=",
                GTE => ">=",
                GT => ">",
            }
        )
    }
}

/// One of the two possible range limits.
#[derive(Clone, Debug, PartialEq)]
pub struct RangeLimit<V>
where
    V: ToSql + 'static,
{
    value: V,
    inclusive: bool,
}

/// Range specifier to be used for filtering.
#[derive(Clone, Debug, PartialEq)]
pub enum Range<V>
where
    V: ToSql + 'static,
{
    Exact(V),
    From(RangeLimit<V>),
    To(RangeLimit<V>),
    Between((RangeLimit<V>, RangeLimit<V>)),
}

impl<V> From<V> for Range<V>
where
    V: ToSql + 'static,
{
    fn from(v: V) -> Self {
        Range::Exact(v)
    }
}

/// Construct a simple select or delete query.
pub struct FilteredOperationBuilder {
    table: &'static str,
    extra: &'static str,
    filters: Filters,
    limit: Option<i32>,
}

impl FilteredOperationBuilder {
    /// Create a new builder
    pub fn new(table: &'static str) -> Self {
        Self {
            table,
            extra: Default::default(),
            filters: Default::default(),
            limit: Default::default(),
        }
    }

    /// Add filtering arguments
    pub fn with_filter<R, V>(mut self, column: &'static str, range: R) -> Self
    where
        R: Into<Range<V>>,
        V: ToSql + 'static,
    {
        use self::Range::*;

        let new_filters: Vec<(ComparisonMode, Box<ToSql>)> = match range.into() {
            Exact(v) => vec![(ComparisonMode::EQ, Box::new(v))],
            From(from) => vec![(
                if from.inclusive { ComparisonMode::GTE } else { ComparisonMode::GT },
                Box::new(from.value),
            )],
            To(to) => vec![(
                if to.inclusive { ComparisonMode::LTE } else { ComparisonMode::LT },
                Box::new(to.value),
            )],
            Between((from, to)) => vec![
                (
                    if from.inclusive { ComparisonMode::GTE } else { ComparisonMode::GT },
                    Box::new(from.value),
                ),
                (
                    if to.inclusive { ComparisonMode::LTE } else { ComparisonMode::LT },
                    Box::new(to.value),
                ),
            ],
        };

        self.filters.insert(column, new_filters);
        self
    }

    pub fn with_limit(mut self, limit: Option<i32>) -> Self {
        self.limit = limit;
        self
    }

    /// Add additional statements before the semicolon
    pub fn with_extra(mut self, extra: &'static str) -> Self {
        self.extra = extra;
        self
    }

    /// Build a query
    pub fn build(self, op: FilteredOperation) -> (String, Vec<Box<ToSql + 'static>>) {
        let (where_q, args) = build_where_from_filters(self.filters, 1);

        let out = format!(
            "{} FROM {}{}{}{};",
            &match op {
                FilteredOperation::Select { op, .. } => match op {
                    None => "SELECT *".to_string(),
                    Some(op) => format!("SELECT {}(*)", op.to_sql()),
                },
                FilteredOperation::Delete => "DELETE".to_string(),
            },
            self.table,
            if !where_q.is_empty() {
                format!(" WHERE {}", where_q)
            } else {
                "".to_string()
            },
            if !self.extra.is_empty() {
                format!(" {}", self.extra)
            } else {
                "".to_string()
            },
            &match op {
                FilteredOperation::Delete => " RETURNING *".to_string(),
                FilteredOperation::Select { limit, .. } => {
                    if let Some(v) = limit {
                        format!(" LIMIT {}", v)
                    } else {
                        "".to_string()
                    }
                }
            }
        );

        (out, args)
    }
}

/// Construct a simple insert query.
pub struct InsertBuilder {
    table: &'static str,
    extra: &'static str,
    values: BTreeMap<&'static str, Box<ToSql + 'static>>,
}

impl InsertBuilder {
    pub fn new(table: &'static str) -> Self {
        Self {
            table,
            extra: Default::default(),
            values: Default::default(),
        }
    }

    pub fn with_arg<V: ToSql + 'static>(mut self, k: &'static str, v: V) -> Self {
        self.values.insert(k, Box::new(v));
        self
    }

    /// Add additional statements before the semicolon
    pub fn with_extra(mut self, extra: &'static str) -> Self {
        self.extra = extra;
        self
    }

    /// Builds a query
    pub fn build(self) -> (String, Vec<Box<ToSql + 'static>>) {
        let mut args = vec![];
        let mut query = format!("INSERT INTO {}", self.table);

        let mut col_string = String::new();
        let mut arg_string = String::new();
        for (i, (col, arg)) in self.values.into_iter().enumerate() {
            let arg_index = i + 1;
            if arg_index > 1 {
                col_string.push_str(", ");
                arg_string.push_str(", ");
            }

            col_string.push_str(&col);
            arg_string.push_str(&format!("${}", arg_index));
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

/// Construct a simple update query.
pub struct UpdateBuilder {
    extra: &'static str,
    values: BTreeMap<&'static str, Box<ToSql + 'static>>,
    filters: FilteredOperationBuilder,
}

impl UpdateBuilder {
    /// Add values to set
    pub fn with_value<V: ToSql + 'static>(mut self, column: &'static str, value: V) -> Self {
        self.values.insert(column, Box::new(value));
        self
    }

    /// Add additional statements before the semicolon
    pub fn with_extra(mut self, extra: &'static str) -> Self {
        self.extra = extra;
        self
    }

    /// Builds an UPDATE query if update values are set and SELECT query otherwise.
    pub fn build(self) -> (String, Vec<Box<ToSql + 'static>>) {
        if self.values.is_empty() {
            return self.filters.build(FilteredOperation::Select { op: None, limit: None });
        }

        let mut values = vec![];

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

        let (filter_string, filters) = build_where_from_filters(self.filters.filters, arg_index);

        let mut query = format!(
            "UPDATE {} {}{}",
            self.filters.table,
            &value_string,
            if !filter_string.is_empty() {
                format!(" WHERE {}", &filter_string)
            } else {
                "".to_string()
            }
        );

        if !self.extra.is_empty() {
            query.push_str(&format!(" {}", self.extra));
        }

        query.push_str(" RETURNING *;");

        let args = std::iter::Iterator::chain(values.into_iter(), filters.into_iter()).collect::<Vec<Box<ToSql + 'static>>>();

        (query, args)
    }
}

impl From<FilteredOperationBuilder> for UpdateBuilder {
    fn from(v: FilteredOperationBuilder) -> Self {
        Self {
            extra: v.extra,
            filters: v,
            values: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_builder() {
        let expectation = (
            "SELECT count(*) FROM my_table WHERE filter_column1 = $1 AND filter_column2 > $2 AND filter_column2 <= $3 LIMIT 5;",
            vec![3, 25, 125]
                .into_iter()
                .map(|v| Box::new(v) as Box<ToSql + 'static>)
                .collect::<Vec<Box<ToSql + 'static>>>(),
        );

        let res = FilteredOperationBuilder::new("my_table")
            .with_filter("filter_column1", 3)
            .with_filter(
                "filter_column2",
                Range::Between((
                    RangeLimit {
                        value: 25,
                        inclusive: false,
                    },
                    RangeLimit {
                        value: 125,
                        inclusive: true,
                    },
                )),
            )
            .build(FilteredOperation::Select {
                op: Some(SelectOperation::Count),
                limit: Some(5),
            });

        assert_eq!(res.0, expectation.0);
        assert_eq!(format!("{:?}", res.1), format!("{:?}", expectation.1));
    }

    #[test]
    fn test_update_builder() {
        let res = UpdateBuilder::from(
            FilteredOperationBuilder::new("my_table")
                .with_filter("filter_column1", 3)
                .with_filter(
                    "filter_column2",
                    Range::Between((
                        RangeLimit {
                            value: 25,
                            inclusive: false,
                        },
                        RangeLimit {
                            value: 125,
                            inclusive: true,
                        },
                    )),
                ),
        ).with_value("value_column1", 1)
            .with_value("value_column2", 2)
            .build();

        let expectation = (
            "UPDATE my_table SET value_column1 = $1, value_column2 = $2 WHERE filter_column1 = $3 AND filter_column2 > $4 AND filter_column2 <= $5 RETURNING *;",
            vec![1, 2, 3, 25, 125]
                .into_iter()
                .map(|v| Box::new(v) as Box<ToSql + 'static>)
                .collect::<Vec<Box<ToSql + 'static>>>(),
        );

        assert_eq!(res.0, expectation.0);
        assert_eq!(format!("{:?}", res.1), format!("{:?}", expectation.1));
    }
}

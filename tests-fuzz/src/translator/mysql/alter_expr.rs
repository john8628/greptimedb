// Copyright 2023 Greptime Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use common_query::AddColumnLocation;
use datatypes::data_type::ConcreteDataType;
use sql::statements::concrete_data_type_to_sql_data_type;

use crate::error::{Error, Result};
use crate::ir::alter_expr::AlterTableOperation;
use crate::ir::create_expr::ColumnOption;
use crate::ir::{AlterTableExpr, Column};
use crate::translator::DslTranslator;

pub struct AlterTableExprTranslator;

impl DslTranslator<AlterTableExpr, String> for AlterTableExprTranslator {
    type Error = Error;

    fn translate(&self, input: &AlterTableExpr) -> Result<String> {
        Ok(match &input.alter_options {
            AlterTableOperation::AddColumn { column, location } => {
                Self::format_add_column(&input.table_name, column, location)
            }
            AlterTableOperation::DropColumn { name } => Self::format_drop(&input.table_name, name),
            AlterTableOperation::RenameTable { new_table_name } => {
                Self::format_rename(&input.table_name, new_table_name)
            }
        })
    }
}

impl AlterTableExprTranslator {
    fn format_drop(name: &str, column: &str) -> String {
        format!("ALTER TABLE {name} DROP COLUMN {column};")
    }

    fn format_rename(name: &str, new_name: &str) -> String {
        format!("ALTER TABLE {name} RENAME {new_name};")
    }

    fn format_add_column(
        name: &str,
        column: &Column,
        location: &Option<AddColumnLocation>,
    ) -> String {
        format!(
            "{};",
            vec![
                format!(
                    "ALTER TABLE {name} ADD COLUMN {}",
                    Self::format_column(column)
                ),
                Self::format_location(location).unwrap_or_default(),
            ]
            .into_iter()
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join(" ")
        )
    }

    fn format_location(location: &Option<AddColumnLocation>) -> Option<String> {
        location.as_ref().map(|location| match location {
            AddColumnLocation::First => "FIRST".to_string(),
            AddColumnLocation::After { column_name } => format!("AFTER {column_name}"),
        })
    }

    fn format_column(column: &Column) -> String {
        vec![
            column.name.to_string(),
            Self::format_column_type(&column.column_type),
            Self::format_column_options(&column.options),
        ]
        .into_iter()
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
    }

    fn format_column_type(column_type: &ConcreteDataType) -> String {
        // Safety: We don't use the `Dictionary` type
        concrete_data_type_to_sql_data_type(column_type)
            .unwrap()
            .to_string()
    }

    fn format_column_options(options: &[ColumnOption]) -> String {
        options
            .iter()
            .map(|option| option.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    }
}

#[cfg(test)]
mod tests {
    use common_query::AddColumnLocation;
    use datatypes::data_type::ConcreteDataType;

    use super::AlterTableExprTranslator;
    use crate::ir::alter_expr::AlterTableOperation;
    use crate::ir::create_expr::ColumnOption;
    use crate::ir::{AlterTableExpr, Column};
    use crate::translator::DslTranslator;

    #[test]
    fn test_alter_table_expr() {
        let alter_expr = AlterTableExpr {
            table_name: "test".to_string(),
            alter_options: AlterTableOperation::AddColumn {
                column: Column {
                    name: "host".to_string(),
                    column_type: ConcreteDataType::string_datatype(),
                    options: vec![ColumnOption::PrimaryKey],
                },
                location: Some(AddColumnLocation::First),
            },
        };

        let output = AlterTableExprTranslator.translate(&alter_expr).unwrap();
        assert_eq!(
            "ALTER TABLE test ADD COLUMN host STRING PRIMARY KEY FIRST;",
            output
        );

        let alter_expr = AlterTableExpr {
            table_name: "test".to_string(),
            alter_options: AlterTableOperation::RenameTable {
                new_table_name: "foo".to_string(),
            },
        };

        let output = AlterTableExprTranslator.translate(&alter_expr).unwrap();
        assert_eq!("ALTER TABLE test RENAME foo;", output);

        let alter_expr = AlterTableExpr {
            table_name: "test".to_string(),
            alter_options: AlterTableOperation::DropColumn {
                name: "foo".to_string(),
            },
        };

        let output = AlterTableExprTranslator.translate(&alter_expr).unwrap();
        assert_eq!("ALTER TABLE test DROP COLUMN foo;", output);
    }
}

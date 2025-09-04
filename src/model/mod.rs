pub mod first;

#[cfg(test)]
mod test_create {
    use std::fmt::Display;

    use heck::ToUpperCamelCase;
    use sqlx::{MySqlPool, Row};

    use serde::{Deserialize, Serialize};
    use sqlx::prelude::FromRow;
    use tokio::{fs, process::Command};

    use crate::core::{config::AppConfig, state::AppState};

    /// 自动生成 数据库表 对应的 结构体
    #[tokio::test]
    async fn create_model() {
        // 操作的数据库
        let database_name = "first";
        let output_file = format!("src/model/{}.rs", database_name);

        let conf = AppConfig::init();
        let mysql = AppState::init_mysql(&conf).await.unwrap();

        let datas = TableInfo::from_db(&mysql, database_name).await;
        // 拼接头部内容，写入文件
        let table_str = format!(
            r#"use serde::{{Deserialize, Serialize}}; 
                {}"#,
            datas
                .into_iter()
                .map(|t| t.to_string())
                .collect::<Vec<String>>()
                .join("\n")
        );
        fs::write(&output_file, table_str).await.unwrap();
        // 自动格式化代码
        Command::new("rustfmt")
            .arg(&output_file)
            .status()
            .await
            .unwrap();
    }

    #[derive(Serialize, Deserialize, FromRow, Debug)]
    struct TableInfo {
        table_name: String,
        table_comment: String,
        columns: Vec<ColumnInfo>,
    }

    impl Display for TableInfo {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            // 形成结构
            write!(
                f,
                "// {}\n#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, Clone)]\npub struct {} {{\n{}\n}}",
                self.table_comment,
                self.table_name.to_upper_camel_case(),
                self.columns
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join("")
            )
        }
    }

    impl TableInfo {
        async fn from_db(pool: &MySqlPool, database_name: &str) -> Vec<TableInfo> {
            let rows = sqlx::query(
                r#"select 
                        CAST(TABLE_NAME as char) as table_name,
                        TABLE_COMMENT as table_comment 
                    FROM information_schema.tables 
                    where table_schema = ?"#,
            )
            .bind(&database_name)
            .fetch_all(pool)
            .await
            .unwrap();

            let mut result = Vec::with_capacity(rows.len());
            for row in rows {
                let table_name = row.get::<String, _>("table_name");
                let table_comment = row.get::<String, _>("table_comment");
                let columns = ColumnInfo::from_db(pool, &database_name, &table_name).await;

                result.push(TableInfo {
                    table_name,
                    table_comment,
                    columns,
                });
            }
            result
        }
    }
    #[derive(Serialize, Deserialize, FromRow, Debug)]
    struct ColumnInfo {
        column_name: String,
        ordinal_position: u8,
        is_nullable: String,
        data_type: String,
        column_comment: String,
    }

    impl Display for ColumnInfo {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let base_type = self.cover_type();
            let base_type = if self.is_nullable == "YES" {
                format!("Option<{}>", base_type)
            } else {
                base_type.to_string()
            };
            // 备注 字段 属性
            write!(
                f,
                "pub {}: {}, //  {}\n",
                self.column_name, base_type, self.column_comment
            )
        }
    }

    impl ColumnInfo {
        async fn from_db(
            pool: &MySqlPool,
            database_name: &str,
            table_name: &str,
        ) -> Vec<ColumnInfo> {
            sqlx::query_as(
                r#"select 
                            COLUMN_NAME as column_name,
                            ORDINAL_POSITION as ordinal_position,
                            IS_NULLABLE as is_nullable,
                            CAST(DATA_TYPE as char) as data_type,
                            CAST(COLUMN_COMMENT as char) as column_comment
                    from information_schema.COLUMNS 
                    where TABLE_SCHEMA = ? and TABLE_NAME = ? "#,
            )
            .bind(database_name)
            .bind(table_name)
            .fetch_all(pool)
            .await
            .unwrap()
        }

        fn cover_type(&self) -> &str {
            match self.data_type.as_str() {
                "tinyint" => "u8",
                "integer" | "int" | "int4" | "smallint" => "i32",
                "bigint" | "int8" => "i64",
                "float" | "double" | "real" | "numeric" | "decimal" => "f64",
                "text" | "varchar" | "char" | "string" => "String",
                "boolean" | "bool" => "bool",
                "datetime"
                | "timestamp"
                | "timestamp without time zone"
                | "timestamp with time zone" => "chrono::NaiveDateTime",
                "date" => "chrono::NaiveDate",
                _ => "String",
            }
        }
    }
}

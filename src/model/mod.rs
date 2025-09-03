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
        async fn from_db(pool: MySqlPool, database_name: &str) -> Vec<TableInfo> {
            let rows = sqlx::query(
            r#"SELECT CAST(TABLE_NAME AS CHAR) AS TABLE_NAME,TABLE_COMMENT FROM information_schema.tables where table_schema = ?"#,
            ).bind(&database_name)
            .fetch_all(&pool)
            .await
            .unwrap();

            let mut result = Vec::new();
            for row in rows {
                let table_name = row.get::<String, _>("TABLE_NAME");
                let columns = ColumnInfo::from_db(pool.clone(), &database_name, &table_name).await;

                result.push(TableInfo {
                    table_name,
                    table_comment: row.get::<String, _>("TABLE_COMMENT"),
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
                "    pub {}: {}, //  {}\n",
                self.column_name, base_type, self.column_comment
            )
        }
    }

    impl ColumnInfo {
        async fn from_db(
            pool: MySqlPool,
            database_name: &str,
            table_name: &str,
        ) -> Vec<ColumnInfo> {
            let rows = sqlx::query(
            r#"select COLUMN_NAME,ORDINAL_POSITION,IS_NULLABLE,CAST(DATA_TYPE AS CHAR) AS DATA_TYPE,CAST(COLUMN_COMMENT AS CHAR) AS COLUMN_COMMENT
                    from information_schema.COLUMNS 
                    where TABLE_SCHEMA = ? and TABLE_NAME = ? "#,
            ).bind(database_name).bind(table_name)
            .fetch_all(&pool)
            .await
            .unwrap();

            rows.into_iter()
                .map(|row| ColumnInfo {
                    column_name: row.get::<String, _>("COLUMN_NAME"),
                    ordinal_position: row.get::<u8, _>("ORDINAL_POSITION"),
                    is_nullable: row.get::<String, _>("IS_NULLABLE"),
                    data_type: row.get::<String, _>("DATA_TYPE"),
                    column_comment: row.get::<String, _>("COLUMN_COMMENT"),
                })
                .collect()
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

    #[tokio::test]
    async fn create_model() {
        let conf = AppConfig::init();
        let mysql = AppState::init_mysql(&conf).await.unwrap();
        // 操作的数据库
        let database_name = "first";
        let datas = TableInfo::from_db(mysql, database_name).await;
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
        fs::write(format!("src/model/{}.rs", database_name), table_str)
            .await
            .unwrap();
        // 自动格式化代码
        Command::new("rustfmt")
            .arg(format!("src/model/{}.rs", database_name))
            .status()
            .await
            .expect("Failed to format generated code");
    }
}

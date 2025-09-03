pub mod first;

#[cfg(test)]
mod test_create {

    use std::collections::HashMap;

    use heck::ToUpperCamelCase;
    use sqlx::Row;

    use serde::{Deserialize, Serialize};
    use sqlx::prelude::FromRow;
    use tokio::fs;

    use crate::core::{config::AppConfig, state::AppState};
    #[derive(Serialize, Deserialize, FromRow, Debug)]
    struct TableInfo {
        table_name: String,
        column_name: String,
        ordinal_position: u8,
        is_nullable: String,
        data_type: String,
        column_comment: String,
    }

    impl ToString for TableInfo {
        fn to_string(&self) -> String {
            let base_type = match self.data_type.as_str() {
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
            };
            let base_type = if self.is_nullable == "YES" {
                format!("Option<{}>", base_type)
            } else {
                base_type.to_string()
            };
            // 备注 字段 属性
            format!(
                r#"
            // {}
            pub {}:{}"#,
                self.column_comment, self.column_name, base_type
            )
        }
    }

    impl TableInfo {
        async fn from_db(database_name: &str) -> Vec<TableInfo> {
            let conf = AppConfig::init();
            let mysql = AppState::init_mysql(&conf).await.unwrap();

            let rows = sqlx::query(
            r#"select CAST(TABLE_NAME AS CHAR) AS TABLE_NAME,column_name,ordinal_position,is_nullable,CAST(DATA_TYPE AS CHAR)  AS DATA_TYPE ,CAST(COLUMN_COMMENT AS CHAR) AS COLUMN_COMMENT
                    from information_schema.COLUMNS 
                    where TABLE_SCHEMA = ?"#,
        ).bind(database_name)
        .fetch_all(&mysql)
        .await
        .unwrap();

            rows.into_iter()
                .map(|row| TableInfo {
                    table_name: row.get::<String, _>("TABLE_NAME"),
                    column_name: row.get::<String, _>("COLUMN_NAME"),
                    ordinal_position: row.get::<u8, _>("ORDINAL_POSITION"),
                    is_nullable: row.get::<String, _>("IS_NULLABLE"),
                    data_type: row.get::<String, _>("DATA_TYPE"),
                    column_comment: row.get::<String, _>("COLUMN_COMMENT"),
                })
                .collect()
        }
    }

    #[tokio::test]
    async fn hello() {
        // 操作的数据库
        let database_name = "first";
        // 分组
        let datas = TableInfo::from_db(database_name).await;
        let mut map: HashMap<String, Vec<TableInfo>> = HashMap::new();
        datas.into_iter().for_each(|data| {
            map.entry(data.table_name.clone())
                .or_insert_with(Vec::new)
                .push(data)
        });

        // 转换为 struct
        let table_str = map
            .iter()
            .map(|(k, v)| {
                // 形成结构
                format!(
                    r##"
                    #[derive(sqlx::FromRow, Serialize, Deserialize, Debug, Clone)]
                    pub struct {} {{{}}}
                    "##,
                    k.to_upper_camel_case(),
                    v.iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>()
                        .join(",")
                )
            })
            .collect::<Vec<String>>()
            .join("\n");

        // 拼接头部内容，写入文件
        let table_str = format!(
            r#"use serde::{{Deserialize, Serialize}}; 
                {}"#,
            table_str
        );
        fs::write(format!("src/model/{}.rs", database_name), table_str)
            .await
            .unwrap();
    }
}

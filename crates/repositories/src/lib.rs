#![feature(error_generic_member_access)]

use std::backtrace::Backtrace;

use postgres::{tls::NoTlsStream, NoTls, Socket};
use secrets::SecretsError;
use thiserror::Error;
use tokio_postgres::Connection;

pub mod builder;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("")]
    SecretsError {
        #[from]
        source: SecretsError,
        backtrace: Backtrace,
    },
    #[error("")]
    PostgresError {
        #[from]
        source: postgres::Error,
        backtrace: Backtrace,
    },
    #[error("blah")]
    DbBlah,
}

pub async fn get_connection() -> Result<
    (
        tokio_postgres::Client,
        tokio_postgres::Connection<Socket, NoTlsStream>,
    ),
    DbError,
> {
    let password = secrets::get_database_password()?;

    let (client, connection) = tokio_postgres::connect(
        &format!("postgresql://postgres:{}@localhost/postgres", password)[..],
        NoTls,
    )
    .await?;

    Ok((client, connection))
}

pub async fn get_ready_client() -> Result<tokio_postgres::Client, DbError> {
    let (client, connection) = get_connection().await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
            return Err(DbError::DbBlah);
        }
        Ok(())
    });

    Ok(client)
}

pub mod comments {
    use crate::get_connection;

    use super::DbError;
    use shared::Comment;
    use tokio_postgres::{Error, NoTls};

    pub async fn add_comment(comment: Comment) -> Result<(), DbError> {
        let (client, connection) = get_connection().await?;
        // The connection object performs the actual communication with the database,
        // so spawn it off to run on its own.
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        client
            .execute(
                "
            INSERT INTO global_comments (body, author)
            VALUES ($1, $2);
        ",
                &[&comment.body, &comment.author],
            )
            .await?;

        Ok(())
    }

    pub async fn read_comments() -> Result<Vec<Comment>, DbError> {
        let (client, connection) = get_connection().await?;

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        let result = client
            .query("SELECT body, author FROM global_comments", &[])
            .await?;

        return Ok(result
            .iter()
            .map(|row| Comment {
                body: row.get(0),
                author: row.get(1),
            })
            .collect());
    }
}

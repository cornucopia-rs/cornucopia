// This file was generated with `cornucopia`. Do not modify.

#[derive(Debug, Clone, PartialEq, serde::Serialize, Hash)]
pub struct Character {
    pub id: i32,
    pub name: db_types::string::CustomString,
    pub quality: crate::types::Quality,
    pub element: db_types::element::Element,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_date: Option<db_types::date::Date>,
}
pub struct CharacterBorrowed<'a> {
    pub id: i32,
    pub name: db_types::string::CustomStringRef<'a>,
    pub quality: crate::types::Quality,
    pub element: db_types::element::Element,
    pub release_date: Option<db_types::date::Date>,
}
impl<'a> From<CharacterBorrowed<'a>> for Character {
    fn from(
        CharacterBorrowed {
            id,
            name,
            quality,
            element,
            release_date,
        }: CharacterBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            name: name.into(),
            quality,
            element,
            release_date,
        }
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, Hash)]
pub struct SelectCharacterByElement {
    pub name: db_types::string::CustomString,
    pub element: db_types::element::Element,
    pub quality: crate::types::Quality,
}
pub struct SelectCharacterByElementBorrowed<'a> {
    pub name: db_types::string::CustomStringRef<'a>,
    pub element: db_types::element::Element,
    pub quality: crate::types::Quality,
}
impl<'a> From<SelectCharacterByElementBorrowed<'a>> for SelectCharacterByElement {
    fn from(
        SelectCharacterByElementBorrowed {
            name,
            element,
            quality,
        }: SelectCharacterByElementBorrowed<'a>,
    ) -> Self {
        Self {
            name: name.into(),
            element,
            quality,
        }
    }
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct CharacterQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<CharacterBorrowed, tokio_postgres::Error>,
    mapper: fn(CharacterBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> CharacterQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(CharacterBorrowed) -> R) -> CharacterQuery<'c, 'a, 's, C, R, N> {
        CharacterQuery {
            client: self.client,
            params: self.params,
            query: self.query,
            cached: self.cached,
            extractor: self.extractor,
            mapper,
        }
    }
    pub async fn one(self) -> Result<T, tokio_postgres::Error> {
        let row =
            crate::client::async_::one(self.client, self.query, &self.params, self.cached).await?;
        Ok((self.mapper)((self.extractor)(&row)?))
    }
    pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
        self.iter().await?.try_collect().await
    }
    pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
        let opt_row =
            crate::client::async_::opt(self.client, self.query, &self.params, self.cached).await?;
        Ok(opt_row
            .map(|row| {
                let extracted = (self.extractor)(&row)?;
                Ok((self.mapper)(extracted))
            })
            .transpose()?)
    }
    pub async fn iter(
        self,
    ) -> Result<
        impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'c,
        tokio_postgres::Error,
    > {
        let stream = crate::client::async_::raw(
            self.client,
            self.query,
            crate::slice_iter(&self.params),
            self.cached,
        )
        .await?;
        let mapped = stream
            .map(move |res| {
                res.and_then(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
            })
            .into_stream();
        Ok(mapped)
    }
}
pub struct SelectCharacterByElementQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor:
        fn(&tokio_postgres::Row) -> Result<SelectCharacterByElementBorrowed, tokio_postgres::Error>,
    mapper: fn(SelectCharacterByElementBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> SelectCharacterByElementQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(SelectCharacterByElementBorrowed) -> R,
    ) -> SelectCharacterByElementQuery<'c, 'a, 's, C, R, N> {
        SelectCharacterByElementQuery {
            client: self.client,
            params: self.params,
            query: self.query,
            cached: self.cached,
            extractor: self.extractor,
            mapper,
        }
    }
    pub async fn one(self) -> Result<T, tokio_postgres::Error> {
        let row =
            crate::client::async_::one(self.client, self.query, &self.params, self.cached).await?;
        Ok((self.mapper)((self.extractor)(&row)?))
    }
    pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
        self.iter().await?.try_collect().await
    }
    pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
        let opt_row =
            crate::client::async_::opt(self.client, self.query, &self.params, self.cached).await?;
        Ok(opt_row
            .map(|row| {
                let extracted = (self.extractor)(&row)?;
                Ok((self.mapper)(extracted))
            })
            .transpose()?)
    }
    pub async fn iter(
        self,
    ) -> Result<
        impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'c,
        tokio_postgres::Error,
    > {
        let stream = crate::client::async_::raw(
            self.client,
            self.query,
            crate::slice_iter(&self.params),
            self.cached,
        )
        .await?;
        let mapped = stream
            .map(move |res| {
                res.and_then(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
            })
            .into_stream();
        Ok(mapped)
    }
}
pub struct CharactersStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn characters() -> CharactersStmt {
    CharactersStmt("SELECT * FROM characters", None)
}
impl CharactersStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient>(
        &'s self,
        client: &'c C,
    ) -> CharacterQuery<'c, 'a, 's, C, Character, 0> {
        CharacterQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<CharacterBorrowed, tokio_postgres::Error> {
                    Ok(CharacterBorrowed {
                        id: row.try_get(0)?,
                        name: row.try_get(1)?,
                        quality: row.try_get(2)?,
                        element: row.try_get(3)?,
                        release_date: row.try_get(4)?,
                    })
                },
            mapper: |it| Character::from(it),
        }
    }
}
pub struct SelectCharacterByElementStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn select_character_by_element() -> SelectCharacterByElementStmt {
    SelectCharacterByElementStmt(
        "SELECT name, element, quality FROM characters WHERE element = $1",
        None,
    )
}
impl SelectCharacterByElementStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient>(
        &'s self,
        client: &'c C,
        element: &'a db_types::element::Element,
    ) -> SelectCharacterByElementQuery<'c, 'a, 's, C, SelectCharacterByElement, 1> {
        SelectCharacterByElementQuery {
            client,
            params: [element],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<SelectCharacterByElementBorrowed, tokio_postgres::Error> {
                Ok(SelectCharacterByElementBorrowed {
                    name: row.try_get(0)?,
                    element: row.try_get(1)?,
                    quality: row.try_get(2)?,
                })
            },
            mapper: |it| SelectCharacterByElement::from(it),
        }
    }
}

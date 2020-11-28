#[macro_export]
macro_rules! impl_tinyint_sql_op {
    ($enum_name:ident) => {
        impl<DB> ToSql<TinyInt, DB> for $enum_name
            where
                DB: Backend,
                i8: ToSql<TinyInt, DB>,
        {
            fn to_sql<W: Write>(&self, out: &mut Output<W, DB>) -> serialize::Result {
                (*self as i8).to_sql(out)
            }
        }

        impl<DB> FromSql<TinyInt, DB> for $enum_name
            where
                DB: Backend,
                i8: FromSql<TinyInt, DB>,
        {
            fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
                let v = i8::from_sql(bytes)?;
                Ok(serde_json::from_str(&v.to_string()).unwrap_or_default())
            }
        }
    };
}
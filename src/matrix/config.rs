#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MatrixConfig
{
   /// The server to connect with.
   server: String,

   /// The authentication token.
   token: String,

   /// The Matrix app's unique identifier.
   identifier: String,
}

impl MatrixConfig
{
   pub const PATH: &'static str = "config/matrix.toml";
}

impl Default for MatrixConfig
{
   fn default() -> MatrixConfig
   {
      let ue: i64 = SystemTime::now().duration_since(UNIX_EPOCH)
         .unwrap()
         .as_secs()
         .try_into()
         .unwrap();

      let dt: DateTime<Utc> = DateTime::from_utc(NaiveDateTime::from_timestamp(ue, 0), Utc);
      let ulid: Ulid = Ulid::from_datetime(dt);

      return MatrixConfig{
         server: "https://example.com/".to_string(),
         token: "<TOKEN_HERE>".to_string(),
         identifier: ulid.to_string(),
      };
   }
}


// IMPORTS //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use chrono::{
   DateTime,
   NaiveDateTime,
   Utc,
};

use std::time::{SystemTime, UNIX_EPOCH};

use ulid::Ulid;

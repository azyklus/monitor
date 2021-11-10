/// Giphy API configuration details.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GiphyConfig
{
   /// Giphy API token.
   token: String,
}

impl GiphyConfig
{
   /// Creates a new GiphyConfig instance.
   pub fn new(token: String) -> GiphyConfig
   {
      return GiphyConfig{ token };
   }

   /// Returns the GIPHY API token.
   pub fn token(&self) -> String { return self.token.clone(); }
}

impl Default for GiphyConfig
{
   fn default() -> GiphyConfig
   {
      return GiphyConfig{
         token: "<TOKEN_HERE>".to_string(),
      };
   }
}

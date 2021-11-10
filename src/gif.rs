/// An interface to the Giphy API.
pub struct GiphyBot
{
   api: AsyncApi,
   config: GiphyConfig,
}

impl GiphyBot
{
   /// Creates a new instance of the Giphy crawler.
   pub fn new(config: &GiphyConfig) -> Result<GiphyBot, GenericError>
   {
      let http: Client = Client::new();
      let mut api: AsyncApi = AsyncApi::new(config.token(), http);

      return Ok(GiphyBot{
         api,
         config: config.clone(),
      });
   }

   /// Search for a GIF via the GIPHY API.
   pub async fn search(&self, query: &str) -> Result<Vec<Gif>, GenericError>
   {
      let mut sr = SearchRequest::new(query).with_limit(25);
      let res = sr.send_to(&self.api).await.unwrap();

      return Ok(res.data);
   }

   /// Get a random GIF from the API.
   pub async fn random(&self) -> Result<Gif, GenericError>
   {
      let mut rr = RandomRequest::new();
      let res = rr.send_to(&self.api).await.unwrap();

      return Ok(res.data);
   }

   /// Get a set of "trending" GIFs from the API.
   pub async fn trending(&self) -> Result<Vec<Gif>, GenericError>
   {
      let mut tr = TrendingRequest::new();
      let res = tr.send_to(&self.api).await.unwrap();

      return Ok(res.data);
   }
}


// MODULES //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

/// Giphy API configuration.
pub mod config;


// IMPORTS //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use crate::{
   errors::*,
   gif::config::GiphyConfig,
};

use giphy::v1::{
   gifs::{RandomRequest, SearchRequest, TrendingRequest},
   Gif,
   PaginatedGifListResponse,
   r#async::*,
};

use reqwest::Client;

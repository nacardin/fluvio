use std::fmt::Debug;

use kf_protocol::derive::{Decode, Encode};
use kf_protocol::api::Request;

use flv_metadata::core::Spec;
use flv_metadata::topic::TopicSpec;
use flv_metadata::spu::SpuSpec;
use flv_metadata::spu::CustomSpuSpec;
use flv_metadata::spg::SpuGroupSpec;
use crate::ScPublicApiKey;
use crate::AdminRequest;

/// marker trait
pub trait ListFilter {}

/// filter by name
pub type NameFilter = String;

impl ListFilter for NameFilter{}

pub trait ListSpec: Spec {

    /// filter type
    type Filter: ListFilter;
    
    /// convert to list request with filters
    fn into_list_request(filters: Vec<Self::Filter>) -> ListRequest;
}

#[derive(Debug)]
pub enum ListRequest {
    Topic(Vec<NameFilter>),
    Spu(Vec<NameFilter>),
    SpuGroup(Vec<NameFilter>),
    CustomSpu(Vec<NameFilter>)
}

impl Default for ListRequest {
    fn default() -> Self {
        Self::Spu(vec![])
    }
}


impl Request for ListRequest {
    const API_KEY: u16 = ScPublicApiKey::List as u16;
    const DEFAULT_API_VERSION: i16 = 0;
    type Response = ListResponse;
}

impl AdminRequest for ListRequest{}


#[derive(Debug)]
pub enum ListResponse {
    Topic(Vec<Metadata<TopicSpec>>),
    Spu(Vec<Metadata<SpuSpec>>),
    CustomSpu(Vec<Metadata<CustomSpuSpec>>),
    SpuGroup(Vec<Metadata<SpuGroupSpec>>)
}

impl Default for ListResponse {
    fn default() -> Self {
        Self::Topic(vec![])
    }
}



#[derive(Encode, Decode, Default, Debug)]
#[cfg_attr(feature = "use_serde", derive(serde::Serialize,serde::Deserialize),serde(rename_all = "camelCase"))]
pub struct Metadata<S> 
    where S: Spec + Debug, 
        S::Status: Debug
{
    pub name: String,
    pub spec: S,
    pub status: S::Status
}

// later this can be written using procedure macro
mod encoding {

    use std::io::Error;
    use std::io::ErrorKind;

    use log::trace;

    use kf_protocol::Encoder;
    use kf_protocol::Decoder;
    use kf_protocol::Version;
    use kf_protocol::bytes::{Buf, BufMut};
    

    use super::*;

    impl ListRequest {
        /// type represent as string
        fn type_string(&self) -> &'static str {
            match self {
                Self::Topic(_) => TopicSpec::LABEL,
                Self::Spu(_) => SpuSpec::LABEL,
                Self::SpuGroup(_) => SpuGroupSpec::LABEL,
                Self::CustomSpu(_) => CustomSpuSpec::LABEL
            }
        }
    }


    impl Encoder for ListRequest {
    
        fn write_size(&self, version: Version) -> usize {
            let type_size = self.type_string().to_owned().write_size(version);
        
            type_size
                + match self {
                    Self::Topic(s) => s.write_size(version),
                    Self::CustomSpu(s) => s.write_size(version),
                    Self::SpuGroup(s) => s.write_size(version),
                    Self::Spu(s) => s.write_size(version)
                }
        }

        // encode match
        fn encode<T>(&self, dest: &mut T, version: Version) -> Result<(), Error>
        where
            T: BufMut,
        {

            self.type_string().to_owned().encode(dest,version)?;

            match self {
                Self::Topic(s) => s.encode(dest, version)?,
                Self::CustomSpu(s) => s.encode(dest, version)?,
                Self::SpuGroup(s) => s.encode(dest, version)?,
                Self::Spu(s) => s.encode(dest, version)?
            }

            Ok(())
        }
    }

    impl Decoder for ListRequest {
        fn decode<T>(&mut self, src: &mut T, version: Version) -> Result<(), Error>
        where
            T: Buf,
        {
            let mut typ = "".to_owned();
            typ.decode(src, version)?;
            trace!("decoded type: {}", typ);

            match typ.as_ref() {
                TopicSpec::LABEL => {
                    let mut response:  Vec<NameFilter> = vec![];
                    response.decode(src, version)?;
                    *self = Self::Topic(response);
                    Ok(())
                },

                CustomSpuSpec::LABEL => {
                    let mut response: Vec<NameFilter>  = vec![];
                    response.decode(src, version)?;
                    *self = Self::CustomSpu(response);
                    Ok(())
                },

                SpuGroupSpec::LABEL => {
                    let mut response: Vec<NameFilter>  = vec![];
                    response.decode(src, version)?;
                    *self = Self::SpuGroup(response);
                    Ok(())
                },

                SpuSpec::LABEL => {
                    let mut response: Vec<NameFilter> = vec![];
                    response.decode(src, version)?;
                    *self = Self::SpuGroup(response);
                    Ok(())
                }

                // Unexpected type
                _ => Err(Error::new(
                    ErrorKind::InvalidData,
                    format!("invalid request type {}", typ),
                )),
            }
        }
    }
    




    impl ListResponse {
        /// type represent as string
        fn type_string(&self) -> &'static str {
            match self {
                Self::Topic(_) => TopicSpec::LABEL,
                Self::Spu(_) => SpuSpec::LABEL,
                Self::SpuGroup(_) => SpuGroupSpec::LABEL,
                Self::CustomSpu(_) => CustomSpuSpec::LABEL
            }
        }
    }



    impl Encoder for ListResponse {
    
        fn write_size(&self, version: Version) -> usize {
            let type_size = self.type_string().to_owned().write_size(version);
        
            type_size
                + match self {
                    Self::Topic(s) => s.write_size(version),
                    Self::CustomSpu(s) => s.write_size(version),
                    Self::SpuGroup(s) => s.write_size(version),
                    Self::Spu(s) => s.write_size(version)
                }
        }

        // encode match
        fn encode<T>(&self, dest: &mut T, version: Version) -> Result<(), Error>
        where
            T: BufMut,
        {

            self.type_string().to_owned().encode(dest,version)?;

            match self {
                Self::Topic(s) => s.encode(dest, version)?,
                Self::CustomSpu(s) => s.encode(dest, version)?,
                Self::SpuGroup(s) => s.encode(dest, version)?,
                Self::Spu(s) => s.encode(dest, version)?
            }

            Ok(())
        }
    }

    impl Decoder for ListResponse {
        fn decode<T>(&mut self, src: &mut T, version: Version) -> Result<(), Error>
        where
            T: Buf,
        {
            let mut typ = "".to_owned();
            typ.decode(src, version)?;
            trace!("decoded type: {}", typ);

            match typ.as_ref() {
                TopicSpec::LABEL => {
                    let mut response:  Vec<Metadata<TopicSpec>> = vec![];
                    response.decode(src, version)?;
                    *self = Self::Topic(response);
                    Ok(())
                },

                CustomSpuSpec::LABEL => {
                    let mut response: Vec<Metadata<CustomSpuSpec>> = vec![];
                    response.decode(src, version)?;
                    *self = Self::CustomSpu(response);
                    Ok(())
                },

                SpuGroupSpec::LABEL => {
                    let mut response: Vec<Metadata<SpuGroupSpec>>  = vec![];
                    response.decode(src, version)?;
                    *self = Self::SpuGroup(response);
                    Ok(())
                },

                SpuSpec::LABEL => {
                    let mut response: Vec<Metadata<SpuGroupSpec>>= vec![];
                    response.decode(src, version)?;
                    *self = Self::SpuGroup(response);
                    Ok(())
                }

                // Unexpected type
                _ => Err(Error::new(
                    ErrorKind::InvalidData,
                    format!("invalid spec type {}", typ),
                )),
            }
        }
    }


}
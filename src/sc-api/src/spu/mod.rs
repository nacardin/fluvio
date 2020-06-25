pub use flv_metadata::spu::*;

mod convert {
    
    use std::io::Error;
    use std::io::ErrorKind;
    use std::convert::TryInto;

    use crate::objects::*;
    use super::*;
    
    impl From<CustomSpuSpec> for AllCreatableSpec {
        fn from(spec: CustomSpuSpec) -> Self {
            Self::CustomSpu(spec)
        }
    }

    impl DeleteSpec for CustomSpuSpec  {

        fn into_request<K>(key: K) -> DeleteRequest where K: Into<Self::DeleteKey> {
            DeleteRequest::CustomSpu(key.into())
        }

    }

    impl ListSpec for SpuSpec {

        fn into_list_request() -> ListRequest {
            ListRequest::Spu
        }
    }

    impl TryInto<Vec<Metadata<SpuSpec>>> for ListResponse {
        type Error = Error;
        
        fn try_into(self) -> Result<Vec<Metadata<SpuSpec>>, Self::Error> {

            match self {
                ListResponse::Spu(s) => Ok(s),
                _ => Err(Error::new(ErrorKind::Other,"not spu"))
            }

        }
    }



}